//! Implements a small deterministic child process for VibeASR protocol tests.

use std::io::{self, BufRead, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

/// Runs the fixture protocol using the same argument and pipe boundary as the server.
fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let vae_model = argument_value(&arguments, "--vae-model");
    let lm_model = argument_value(&arguments, "--lm-model");
    if vae_model.is_none()
        || lm_model.is_none()
        || !arguments.iter().any(|arg| arg == "--no-token-stream")
    {
        eprintln!("fixture received invalid server arguments: {arguments:?}");
        std::process::exit(2);
    }
    if !Path::new(vae_model.as_deref().unwrap()).is_file()
        || !Path::new(lm_model.as_deref().unwrap()).is_file()
        || !vae_model
            .as_deref()
            .unwrap()
            .contains("VAE model café.gguf")
        || !lm_model.as_deref().unwrap().contains("model 语言.gguf")
    {
        eprintln!("fixture received incorrect model paths: {arguments:?}");
        std::process::exit(2);
    }

    let mode = lm_model.unwrap();
    if mode.contains("early-exit") {
        eprintln!("fixture exited before readiness");
        std::process::exit(3);
    }
    if mode.contains("no-ready") {
        eprintln!("fixture waiting for readiness");
        thread::sleep(Duration::from_secs(30));
        return;
    }
    if mode.contains("wrong-ready") {
        println!("---READY--- ");
        io::stdout().flush().expect("flush near-match readiness");
        thread::sleep(Duration::from_secs(30));
        return;
    }
    if mode.contains("stderr-flood") {
        for _ in 0..256 {
            eprint!("{}", "diagnostic padding ".repeat(128));
        }
        eprintln!("diagnostic tail");
        let _ = io::stderr().flush();
    }

    println!("---READY---");
    io::stdout().flush().expect("flush readiness");
    if mode.contains("no-read") {
        thread::sleep(Duration::from_secs(30));
        return;
    }
    if mode.contains("exit-after-ready") {
        return;
    }

    for line in io::stdin().lock().lines() {
        let line = line.expect("read request");
        if line == "EXIT" || line == "exit" || line == "quit" {
            if mode.contains("ignore-exit") {
                thread::sleep(Duration::from_secs(30));
            }
            return;
        }
        if line.contains("hang_request") {
            thread::sleep(Duration::from_secs(30));
            return;
        }
        if line.contains("mid_response") {
            print!("partial response\n");
            io::stdout().flush().expect("flush partial response");
            return;
        }
        if line.contains("error_case") {
            if mode.contains("stderr-flood") {
                eprintln!("diagnostic tail");
                let _ = io::stderr().flush();
                thread::sleep(Duration::from_millis(50));
            }
            println!("[ERROR] fixture rejected request");
            println!("---END---");
            io::stdout().flush().expect("flush error response");
            continue;
        }
        println!("pid={};audio={line}", std::process::id());
        println!("line two");
        println!("---END---");
        io::stdout().flush().expect("flush response");
    }
}

/// Returns the argument value following the requested option.
fn argument_value(arguments: &[String], option: &str) -> Option<String> {
    arguments
        .iter()
        .position(|argument| argument == option)
        .and_then(|index| arguments.get(index + 1))
        .cloned()
}
