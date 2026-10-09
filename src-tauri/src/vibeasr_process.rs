//! Owns the local persistent VibeASR process and its line-based protocol.

use std::collections::VecDeque;
use std::fmt;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

const READY_SENTINEL: &str = "---READY---";
const END_SENTINEL: &str = "---END---";
const STDERR_DIAGNOSTIC_LIMIT: usize = 16 * 1024;
const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Configuration for starting one persistent VibeASR server process.
#[derive(Clone, Debug)]
pub struct VibeAsrProcessConfig {
    /// Executable path for the VibeASR server.
    pub executable_path: PathBuf,
    /// VAE model path passed to `--vae-model`.
    pub vae_model_path: PathBuf,
    /// Language model path passed to `--lm-model`.
    pub lm_model_path: PathBuf,
    /// Maximum time to wait for the server readiness sentinel.
    pub startup_timeout: Duration,
    /// Maximum time to wait for one transcription response.
    pub request_timeout: Duration,
    /// Maximum time to wait for graceful shutdown before terminating the child.
    pub shutdown_timeout: Duration,
}

impl VibeAsrProcessConfig {
    /// Creates a configuration with bounded defaults for server lifecycle waits.
    pub fn new(
        executable_path: impl Into<PathBuf>,
        vae_model_path: impl Into<PathBuf>,
        lm_model_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            executable_path: executable_path.into(),
            vae_model_path: vae_model_path.into(),
            lm_model_path: lm_model_path.into(),
            startup_timeout: Duration::from_secs(300),
            request_timeout: Duration::from_secs(300),
            shutdown_timeout: Duration::from_secs(2),
        }
    }
}

/// Error returned when the VibeASR process or protocol cannot complete an operation.
#[derive(Debug)]
pub struct VibeAsrProcessError {
    message: String,
    stderr: Option<String>,
}

impl VibeAsrProcessError {
    /// Creates an error with the latest bounded stderr diagnostics, when available.
    fn new(message: impl Into<String>, stderr: Option<String>) -> Self {
        Self {
            message: message.into(),
            stderr: stderr.filter(|diagnostics| !diagnostics.trim().is_empty()),
        }
    }
}

impl fmt::Display for VibeAsrProcessError {
    /// Formats the failure and includes bounded stderr diagnostics when present.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)?;
        if let Some(stderr) = &self.stderr {
            write!(formatter, "; stderr: {}", stderr.trim())?;
        }
        Ok(())
    }
}

impl std::error::Error for VibeAsrProcessError {}

/// Owns one persistent VibeASR child process and its protocol pipes.
#[derive(Debug)]
pub struct VibeAsrProcessClient {
    config: VibeAsrProcessConfig,
    child: Child,
    stdin: Option<ChildStdin>,
    stdout_events: Receiver<StdoutEvent>,
    stderr_diagnostics: Arc<Mutex<BoundedDiagnostics>>,
    reader_threads: Vec<JoinHandle<()>>,
    usable: bool,
    stopped: bool,
}

impl VibeAsrProcessClient {
    /// Starts the server with direct executable arguments and waits for exact readiness.
    pub fn start(config: VibeAsrProcessConfig) -> Result<Self, VibeAsrProcessError> {
        let mut child = Command::new(&config.executable_path)
            .arg("--vae-model")
            .arg(&config.vae_model_path)
            .arg("--lm-model")
            .arg(&config.lm_model_path)
            .arg("--no-token-stream")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| {
                VibeAsrProcessError::new(format!("failed to start VibeASR server: {error}"), None)
            })?;

        let stdin = child.stdin.take();
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();
        let (stdout_sender, stdout_events) = mpsc::channel();
        let stderr_diagnostics = Arc::new(Mutex::new(BoundedDiagnostics::default()));
        let mut reader_threads = Vec::with_capacity(2);

        let stdout_thread = match stdout {
            Some(stdout) => spawn_stdout_reader(stdout, stdout_sender),
            None => Err(io::Error::other("child stdout pipe was not available")),
        };
        match stdout_thread {
            Ok(thread) => reader_threads.push(thread),
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(VibeAsrProcessError::new(
                    format!("failed to read VibeASR stdout: {error}"),
                    None,
                ));
            }
        }

        let stderr_thread = match stderr {
            Some(stderr) => spawn_stderr_reader(stderr, Arc::clone(&stderr_diagnostics)),
            None => Err(io::Error::other("child stderr pipe was not available")),
        };
        match stderr_thread {
            Ok(thread) => reader_threads.push(thread),
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                join_reader_threads(reader_threads);
                return Err(VibeAsrProcessError::new(
                    format!("failed to read VibeASR stderr: {error}"),
                    None,
                ));
            }
        }

        let mut client = Self {
            config,
            child,
            stdin,
            stdout_events,
            stderr_diagnostics,
            reader_threads,
            usable: false,
            stopped: false,
        };

        if let Err(message) = client.wait_for_readiness() {
            client.usable = false;
            let cleanup_error = client.stop_process(false).err();
            let message = cleanup_error.map_or(message.clone(), |cleanup| {
                format!("{message}; child cleanup failed: {cleanup}")
            });
            return Err(client.error(message));
        }

        client.usable = true;
        Ok(client)
    }

    /// Sends one existing WAV path and returns the complete response payload.
    pub fn request(&mut self, audio_path: impl AsRef<Path>) -> Result<String, VibeAsrProcessError> {
        if !self.usable || self.stopped {
            return Err(self.error("VibeASR client is no longer usable"));
        }

        let audio_path = audio_path.as_ref();
        let Some(path_text) = audio_path.to_str() else {
            return Err(self.error("audio path cannot be represented as UTF-8"));
        };
        if path_text.contains(['\r', '\n']) || matches!(path_text, "EXIT" | "exit" | "quit") {
            return Err(self.error("audio path conflicts with the VibeASR line protocol"));
        }

        let write_result = self
            .stdin
            .as_mut()
            .ok_or_else(|| io::Error::new(io::ErrorKind::BrokenPipe, "child stdin is closed"))
            .and_then(|stdin| {
                stdin.write_all(path_text.as_bytes())?;
                stdin.write_all(b"\n")?;
                stdin.flush()
            });
        if let Err(error) = write_result {
            return Err(self.fail(format!("failed to send VibeASR request: {error}")));
        }

        let deadline = Instant::now() + self.config.request_timeout;
        let mut payload_lines = Vec::new();
        loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                return Err(self.fail("VibeASR request timed out"));
            }
            match self.stdout_events.recv_timeout(remaining) {
                Ok(StdoutEvent::Line(line)) if line == END_SENTINEL => {
                    let payload = payload_lines.join("\n");
                    if payload
                        .lines()
                        .next()
                        .is_some_and(|line| line.starts_with("[ERROR]"))
                    {
                        return Err(self.error(format!("VibeASR request failed: {payload}")));
                    }
                    return Ok(payload);
                }
                Ok(StdoutEvent::Line(line)) => payload_lines.push(line),
                Ok(StdoutEvent::Closed) => {
                    return Err(self.fail("VibeASR child closed stdout before ---END---"));
                }
                Ok(StdoutEvent::ReadError(error)) => {
                    return Err(self.fail(format!("failed to read VibeASR response: {error}")));
                }
                Err(RecvTimeoutError::Timeout) => {
                    return Err(self.fail("VibeASR request timed out"));
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(self.fail("VibeASR stdout reader stopped before ---END---"));
                }
            }
        }
    }

    /// Sends `EXIT`, waits for graceful termination, and reaps the child.
    pub fn shutdown(&mut self) -> Result<(), VibeAsrProcessError> {
        if self.stopped {
            return Ok(());
        }
        self.usable = false;

        match self.stop_process(true) {
            Ok(false) => Ok(()),
            Ok(true) => Err(self.error("VibeASR shutdown timed out; child was terminated")),
            Err(error) => Err(self.error(format!("failed to stop VibeASR child: {error}"))),
        }
    }

    /// Waits until the server's exact readiness line arrives or a startup failure occurs.
    fn wait_for_readiness(&mut self) -> Result<(), String> {
        match self.stdout_events.recv_timeout(self.config.startup_timeout) {
            Ok(StdoutEvent::Line(line)) if line == READY_SENTINEL => Ok(()),
            Ok(StdoutEvent::Line(line)) => {
                Err(format!("unexpected stdout before readiness: {line:?}"))
            }
            Ok(StdoutEvent::Closed) => Err(format!(
                "VibeASR child exited before readiness ({})",
                self.child_status_description()
            )),
            Ok(StdoutEvent::ReadError(error)) => {
                Err(format!("failed reading VibeASR readiness: {error}"))
            }
            Err(RecvTimeoutError::Timeout) => {
                Err("VibeASR startup timed out waiting for ---READY---".to_string())
            }
            Err(RecvTimeoutError::Disconnected) => {
                Err("VibeASR stdout reader stopped before readiness".to_string())
            }
        }
    }

    /// Marks the instance unusable, terminates its child, and adds bounded diagnostics.
    fn fail(&mut self, message: impl Into<String>) -> VibeAsrProcessError {
        self.usable = false;
        let message = match self.stop_process(false) {
            Ok(_) => message.into(),
            Err(error) => format!("{}; child cleanup failed: {error}", message.into()),
        };
        self.error(message)
    }

    /// Formats an error with the latest bounded stderr output.
    fn error(&self, message: impl Into<String>) -> VibeAsrProcessError {
        VibeAsrProcessError::new(message, Some(self.stderr_snapshot()))
    }

    /// Returns a stable snapshot of the bounded stderr diagnostic tail.
    fn stderr_snapshot(&self) -> String {
        let diagnostics = self
            .stderr_diagnostics
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        String::from_utf8_lossy(&diagnostics.bytes.iter().copied().collect::<Vec<_>>()).into_owned()
    }

    /// Returns the child's exit status when it is already available.
    fn child_status_description(&mut self) -> String {
        match self.child.try_wait() {
            Ok(Some(status)) => status.to_string(),
            Ok(None) => "still running".to_string(),
            Err(error) => format!("exit status unavailable: {error}"),
        }
    }

    /// Stops and reaps the owned child, returning whether forced termination was needed.
    fn stop_process(&mut self, request_graceful_exit: bool) -> Result<bool, String> {
        if self.stopped {
            return Ok(false);
        }

        let mut forced = false;
        if self
            .child
            .try_wait()
            .map_err(|error| error.to_string())?
            .is_none()
        {
            if request_graceful_exit {
                if let Some(stdin) = self.stdin.as_mut() {
                    let _ = stdin.write_all(b"EXIT\n").and_then(|_| stdin.flush());
                }
                if !wait_for_exit(&mut self.child, self.config.shutdown_timeout)? {
                    terminate_child(&mut self.child)?;
                    forced = true;
                }
            } else {
                terminate_child(&mut self.child)?;
            }
        }

        self.stdin.take();
        join_reader_threads(std::mem::take(&mut self.reader_threads));
        self.stopped = true;
        Ok(forced)
    }
}

impl Drop for VibeAsrProcessClient {
    /// Requests bounded graceful shutdown and forcefully reaps an unresponsive child.
    fn drop(&mut self) {
        let _ = self.stop_process(true);
    }
}

/// A message emitted by the dedicated stdout reader.
#[derive(Debug)]
enum StdoutEvent {
    Line(String),
    Closed,
    ReadError(String),
}

/// Retains only the most recent bounded stderr bytes.
#[derive(Debug, Default)]
struct BoundedDiagnostics {
    bytes: VecDeque<u8>,
}

impl BoundedDiagnostics {
    /// Appends bytes while keeping the diagnostic tail within the configured limit.
    fn push(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.bytes.push_back(*byte);
            if self.bytes.len() > STDERR_DIAGNOSTIC_LIMIT {
                self.bytes.pop_front();
            }
        }
    }
}

/// Starts a thread that reads stdout lines without blocking request timeouts.
fn spawn_stdout_reader(
    stdout: ChildStdout,
    sender: Sender<StdoutEvent>,
) -> io::Result<JoinHandle<()>> {
    thread::Builder::new()
        .name("vibeasr-stdout".to_string())
        .spawn(move || read_stdout_lines(stdout, sender))
}

/// Sends each complete stdout line to the client and reports EOF or read failure.
fn read_stdout_lines(stdout: ChildStdout, sender: Sender<StdoutEvent>) {
    let mut reader = BufReader::new(stdout);
    loop {
        let mut bytes = Vec::new();
        match reader.read_until(b'\n', &mut bytes) {
            Ok(0) => {
                let _ = sender.send(StdoutEvent::Closed);
                return;
            }
            Ok(_) => {
                if bytes.last() == Some(&b'\n') {
                    bytes.pop();
                }
                if bytes.last() == Some(&b'\r') {
                    bytes.pop();
                }
                let line = String::from_utf8_lossy(&bytes).into_owned();
                if sender.send(StdoutEvent::Line(line)).is_err() {
                    return;
                }
            }
            Err(error) => {
                let _ = sender.send(StdoutEvent::ReadError(error.to_string()));
                return;
            }
        }
    }
}

/// Starts a thread that continuously drains stderr into bounded diagnostics.
fn spawn_stderr_reader(
    stderr: ChildStderr,
    diagnostics: Arc<Mutex<BoundedDiagnostics>>,
) -> io::Result<JoinHandle<()>> {
    thread::Builder::new()
        .name("vibeasr-stderr".to_string())
        .spawn(move || drain_stderr(stderr, diagnostics))
}

/// Drains stderr chunks so the child cannot block on a full pipe.
fn drain_stderr(mut stderr: ChildStderr, diagnostics: Arc<Mutex<BoundedDiagnostics>>) {
    let mut buffer = [0_u8; 4096];
    loop {
        match stderr.read(&mut buffer) {
            Ok(0) | Err(_) => return,
            Ok(bytes_read) => {
                diagnostics
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .push(&buffer[..bytes_read]);
            }
        }
    }
}

/// Waits for a child process using bounded polling.
fn wait_for_exit(child: &mut Child, timeout: Duration) -> Result<bool, String> {
    let deadline = Instant::now() + timeout;
    loop {
        if child
            .try_wait()
            .map_err(|error| error.to_string())?
            .is_some()
        {
            return Ok(true);
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Ok(false);
        }
        thread::sleep(PROCESS_POLL_INTERVAL.min(remaining));
    }
}

/// Terminates and reaps a child, tolerating exit between the liveness check and kill.
fn terminate_child(child: &mut Child) -> Result<(), String> {
    if let Err(kill_error) = child.kill() {
        if child
            .try_wait()
            .map_err(|wait_error| wait_error.to_string())?
            .is_none()
        {
            return Err(kill_error.to_string());
        }
    }
    child.wait().map_err(|error| error.to_string())?;
    Ok(())
}

/// Joins reader threads after their child pipe handles have closed.
fn join_reader_threads(threads: Vec<JoinHandle<()>>) {
    for thread in threads {
        let _ = thread.join();
    }
}

#[cfg(test)]
mod tests {
    //! Exercises the client through a real lightweight child process.

    use super::{VibeAsrProcessClient, VibeAsrProcessConfig};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::OnceLock;
    use std::time::Duration;
    use tempfile::TempDir;

    static FIXTURE: OnceLock<(TempDir, PathBuf)> = OnceLock::new();

    /// Compiles the protocol fixture once for all process-boundary tests.
    fn fixture_executable() -> &'static Path {
        FIXTURE
            .get_or_init(|| {
                let directory = tempfile::tempdir().expect("fixture temp directory");
                let executable = directory.path().join(if cfg!(windows) {
                    "vibeasr fixture Ω.exe"
                } else {
                    "vibeasr fixture Ω"
                });
                let source = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests/fixtures/vibeasr_process_server.rs");
                let output = Command::new("rustc")
                    .arg("--edition=2021")
                    .arg(&source)
                    .arg("-o")
                    .arg(&executable)
                    .output()
                    .expect("launch rustc for process fixture");
                assert!(
                    output.status.success(),
                    "fixture compilation failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                (directory, executable)
            })
            .1
            .as_path()
    }

    /// Creates paths with spaces and Unicode for direct argument and stdin tests.
    fn fixture_paths(directory: &Path, mode: &str) -> (PathBuf, PathBuf, PathBuf) {
        let path_directory = directory.join("paths with spaces ø & $(no-injection)");
        std::fs::create_dir_all(&path_directory).expect("create Unicode fixture paths");
        let executable_directory = path_directory.join("executables Ω");
        std::fs::create_dir_all(&executable_directory)
            .expect("create fixture executable directory");
        let executable = fixture_executable();
        let copied_executable = executable_directory.join(executable.file_name().unwrap());
        std::fs::copy(executable, &copied_executable).expect("copy fixture into Unicode path");
        let vae_model = path_directory.join("VAE model café.gguf");
        let lm_model = path_directory.join(format!("{mode} model 语言.gguf"));
        std::fs::write(&vae_model, b"fixture model").expect("create VAE model placeholder");
        std::fs::write(&lm_model, b"fixture model").expect("create LM model placeholder");
        (copied_executable, vae_model, lm_model)
    }

    /// Builds a client configuration with short deterministic test timeouts.
    fn config(directory: &Path, mode: &str) -> VibeAsrProcessConfig {
        let (executable, vae_model, lm_model) = fixture_paths(directory, mode);
        VibeAsrProcessConfig {
            executable_path: executable,
            vae_model_path: vae_model,
            lm_model_path: lm_model,
            startup_timeout: Duration::from_millis(500),
            request_timeout: Duration::from_millis(500),
            shutdown_timeout: Duration::from_millis(500),
        }
    }

    /// Verifies exact readiness, multiline responses, Unicode paths, reuse, and graceful reaping.
    #[test]
    fn persistent_child_handles_repeated_unicode_path_requests() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let mut client = VibeAsrProcessClient::start(config(directory.path(), "normal"))
            .expect("start persistent fixture");
        let process_id = client.child.id();
        let audio_path = directory
            .path()
            .join("audio recording 咖啡 with spaces.wav");
        std::fs::write(&audio_path, b"fixture audio").expect("create WAV placeholder");

        let first = client.request(&audio_path).expect("first request");
        let second = client.request(&audio_path).expect("second request");

        assert_eq!(
            first,
            format!("pid={process_id};audio={}\nline two", audio_path.display())
        );
        assert_eq!(second, first);
        client.shutdown().expect("graceful shutdown");
        assert_process_stopped(process_id);
    }

    /// Verifies startup EOF returns an error containing bounded child diagnostics.
    #[test]
    fn early_exit_before_readiness_is_reported() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let error = VibeAsrProcessClient::start(config(directory.path(), "early-exit"))
            .expect_err("early child exit must fail startup");

        assert!(error.to_string().contains("before readiness"));
        assert!(error
            .to_string()
            .contains("fixture exited before readiness"));
    }

    /// Verifies a missing readiness sentinel times out and reaps the child.
    #[test]
    fn startup_without_readiness_times_out_and_reaps_child() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let mut startup = config(directory.path(), "no-ready");
        startup.startup_timeout = Duration::from_millis(100);

        let error =
            VibeAsrProcessClient::start(startup).expect_err("missing readiness must time out");
        assert!(error.to_string().contains("startup timed out"));
        assert!(error.to_string().contains("fixture waiting for readiness"));

        let mut incorrect = config(directory.path(), "wrong-ready");
        incorrect.startup_timeout = Duration::from_millis(500);
        let error = VibeAsrProcessClient::start(incorrect)
            .expect_err("near-match readiness sentinel must be rejected");
        assert!(error
            .to_string()
            .contains("unexpected stdout before readiness"));
    }

    /// Verifies upstream error payloads cannot become successful transcriptions.
    #[test]
    fn upstream_error_response_is_reported() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let mut client = VibeAsrProcessClient::start(config(directory.path(), "normal"))
            .expect("start persistent fixture");
        let audio_path = directory.path().join("error_case.wav");
        std::fs::write(&audio_path, b"fixture audio").expect("create WAV placeholder");

        let error = client
            .request(audio_path)
            .expect_err("[ERROR] must fail request");
        assert!(error.to_string().contains("fixture rejected request"));

        let recovery_path = directory.path().join("recovery.wav");
        std::fs::write(&recovery_path, b"fixture audio").expect("create recovery WAV");
        assert!(client.request(recovery_path).is_ok());
    }

    /// Verifies EOF during a response and request timeouts both fail and poison the client.
    #[test]
    fn incomplete_and_timed_out_requests_fail_without_respawning() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let mut client = VibeAsrProcessClient::start(config(directory.path(), "normal"))
            .expect("start persistent fixture");
        let first_process_id = client.child.id();
        let partial_path = directory.path().join("mid_response.wav");
        std::fs::write(&partial_path, b"fixture audio").expect("create WAV placeholder");
        let error = client
            .request(&partial_path)
            .expect_err("EOF before END must fail request");
        assert!(error.to_string().contains("before ---END---"));
        assert!(client.request(&partial_path).is_err());
        assert_process_stopped(first_process_id);

        let mut timeout_client = VibeAsrProcessClient::start(config(directory.path(), "normal"))
            .expect("start second fixture");
        let second_process_id = timeout_client.child.id();
        timeout_client.config.request_timeout = Duration::from_millis(100);
        let hang_path = directory.path().join("hang_request.wav");
        std::fs::write(&hang_path, b"fixture audio").expect("create WAV placeholder");
        let error = timeout_client
            .request(hang_path)
            .expect_err("request timeout must fail");
        assert!(error.to_string().contains("request timed out"));
        assert!(timeout_client.request("later.wav").is_err());
        assert_process_stopped(second_process_id);

        let mut exiting_client =
            VibeAsrProcessClient::start(config(directory.path(), "exit-after-ready"))
                .expect("start fixture that exits after readiness");
        let error = exiting_client
            .request("later.wav")
            .expect_err("unexpected process exit must not be success");
        assert!(
            error.to_string().contains("before ---END---")
                || error.to_string().contains("send VibeASR request")
        );
    }

    /// Verifies stderr is drained and retained diagnostics remain bounded on failure.
    #[test]
    fn stderr_is_drained_and_error_diagnostics_are_bounded() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let mut client = VibeAsrProcessClient::start(config(directory.path(), "stderr-flood"))
            .expect("large stderr output must not block startup");
        let audio_path = directory.path().join("error_case.wav");
        std::fs::write(&audio_path, b"fixture audio").expect("create WAV placeholder");

        let error = client
            .request(audio_path)
            .expect_err("fixture error response");
        let rendered = error.to_string();
        assert!(rendered.contains("diagnostic tail"));
        assert!(rendered.len() < 20 * 1024);
    }

    /// Verifies shutdown forcibly terminates a child that ignores EXIT.
    #[test]
    fn non_cooperative_shutdown_kills_and_reaps_child() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let mut startup = config(directory.path(), "ignore-exit");
        startup.shutdown_timeout = Duration::from_millis(100);
        let mut client = VibeAsrProcessClient::start(startup).expect("start fixture");
        let process_id = client.child.id();

        let error = client
            .shutdown()
            .expect_err("forced termination should report timeout");
        assert!(error.to_string().contains("shutdown timed out"));
        assert_process_stopped(process_id);
    }

    /// Verifies dropping the client leaves no fixture child running.
    #[test]
    fn dropping_client_terminates_owned_child() {
        let directory = tempfile::tempdir().expect("test temp directory");
        let client =
            VibeAsrProcessClient::start(config(directory.path(), "normal")).expect("start fixture");
        let process_id = client.child.id();

        drop(client);

        assert_process_stopped(process_id);
    }

    /// Waits for a process ID to disappear using the host's process table.
    fn assert_process_stopped(process_id: u32) {
        let deadline = std::time::Instant::now() + Duration::from_secs(3);
        while std::time::Instant::now() < deadline {
            let running = if cfg!(windows) {
                Command::new("tasklist")
                    .args(["/FI", &format!("PID eq {process_id}"), "/NH"])
                    .output()
                    .expect("query Windows process table")
                    .stdout
                    .windows(process_id.to_string().len())
                    .any(|window| window == process_id.to_string().as_bytes())
            } else {
                Command::new("kill")
                    .args(["-0", &process_id.to_string()])
                    .status()
                    .expect("query Unix process table")
                    .success()
            };
            if !running {
                return;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        panic!("fixture process {process_id} remained alive");
    }
}
