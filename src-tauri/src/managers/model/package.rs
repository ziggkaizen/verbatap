//! Required model artifacts: catalog-owned roles and integrity, independent of engines.

use super::*;
use sha2::{Digest, Sha256};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Clone, Deserialize)]
pub struct ModelArtifact {
    pub role: String,
    pub filename: String,
    pub size_bytes: u64,
    pub sha256: String,
}

/// An ordered, nonempty set of required artifacts with unique roles and paths.
/// Constructed through validation, including when deserialized from a catalog.
#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "Vec<ModelArtifact>")]
pub struct ModelPackage {
    artifacts: Vec<ModelArtifact>,
}

/// A verified artifact path for an engine to consume. Order follows the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedModelArtifact {
    pub role: String,
    pub path: PathBuf,
}

impl TryFrom<Vec<ModelArtifact>> for ModelPackage {
    type Error = String;

    fn try_from(mut artifacts: Vec<ModelArtifact>) -> std::result::Result<Self, String> {
        if artifacts.is_empty() {
            return Err("a package requires at least one artifact".into());
        }
        let mut roles = HashSet::new();
        let mut filenames = HashSet::new();
        let mut size = 0_u64;
        for artifact in &mut artifacts {
            // Catalog paths use '/' on every platform. Exclude Windows path
            // prefixes, traversal and reserved partial names before joining.
            if artifact.role.trim().is_empty()
                || !roles.insert(artifact.role.clone())
                || artifact.filename.contains(['\\', ':'])
                || artifact.filename.split('/').any(|p| {
                    p.is_empty() || p == "." || p == ".." || p.to_lowercase().ends_with(".partial")
                })
                || !filenames.insert(artifact.filename.to_lowercase())
                || artifact.size_bytes == 0
                || artifact.sha256.len() != 64
                || !artifact.sha256.bytes().all(|b| b.is_ascii_hexdigit())
            {
                return Err(format!(
                    "invalid or duplicate artifact: {}",
                    artifact.filename
                ));
            }
            size = size
                .checked_add(artifact.size_bytes)
                .ok_or("package size overflow")?;
            artifact.sha256.make_ascii_lowercase();
        }
        for a in &artifacts {
            if artifacts.iter().any(|b| {
                b.filename
                    .to_lowercase()
                    .starts_with(&format!("{}/", a.filename.to_lowercase()))
            }) {
                return Err("artifact paths cannot contain each other".into());
            }
        }
        Ok(Self { artifacts })
    }
}

impl ModelArtifact {
    pub(super) fn is_verified(&self, path: &Path) -> bool {
        path.metadata()
            .is_ok_and(|m| m.is_file() && m.len() == self.size_bytes)
            && ModelManager::compute_sha256(path)
                .is_ok_and(|hash| hash.eq_ignore_ascii_case(&self.sha256))
    }
}

impl ModelPackage {
    pub fn artifacts(&self) -> &[ModelArtifact] {
        &self.artifacts
    }

    pub fn size_bytes(&self) -> u64 {
        self.artifacts.iter().map(|a| a.size_bytes).sum()
    }

    /// hf-hub preallocates the artifact size plus an eight-byte committed
    /// offset footer. File length alone is therefore not downloaded progress.
    fn hf_partial_size(path: &Path, size_bytes: u64) -> u64 {
        let read = || -> std::io::Result<u64> {
            let mut file = File::open(path)?;
            if size_bytes.checked_add(8) != Some(file.metadata()?.len()) {
                return Ok(0);
            }
            file.seek(SeekFrom::End(-8))?;
            let mut footer = [0_u8; 8];
            file.read_exact(&mut footer)?;
            let committed = u64::from_le_bytes(footer);
            Ok(if committed <= size_bytes {
                committed
            } else {
                0
            })
        };
        read().unwrap_or(0)
    }

    /// A private models-dir namespace, stable across sessions and safe for any id.
    pub(super) fn directory(model_id: &str) -> String {
        format!(".packages/{:x}", Sha256::digest(model_id.as_bytes()))
    }

    fn resolve_with(
        &self,
        local_dir: &Path,
        cached: impl Fn(&str) -> Option<PathBuf>,
    ) -> Result<Vec<ResolvedModelArtifact>> {
        self.artifacts
            .iter()
            .map(|artifact| {
                let path = cached(&artifact.filename)
                    .filter(|p| artifact.is_verified(p))
                    .or_else(|| {
                        let path = local_dir.join(&artifact.filename);
                        artifact.is_verified(&path).then_some(path)
                    })
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "Missing or unverified artifact: {} ({})",
                            artifact.role,
                            artifact.filename
                        )
                    })?;
                Ok(ResolvedModelArtifact {
                    role: artifact.role.clone(),
                    path,
                })
            })
            .collect()
    }

    pub(super) fn resolve(
        &self,
        models_dir: &Path,
        descriptor: &ModelDescriptor,
    ) -> Result<Vec<ResolvedModelArtifact>> {
        let local_dir = models_dir.join(Self::directory(&descriptor.id));
        self.resolve_with(&local_dir, |filename| match &descriptor.source {
            ModelSource::HuggingFace { repo_id, revision } => {
                hf_cached_path(repo_id, revision, filename)
            }
            _ => None,
        })
    }

    pub(super) fn disk_status(
        &self,
        models_dir: &Path,
        descriptor: &ModelDescriptor,
        downloading: bool,
    ) -> DiskStatus {
        let local_dir = models_dir.join(Self::directory(&descriptor.id));
        let hf_blobs = match &descriptor.source {
            ModelSource::HuggingFace { repo_id, revision } => Some(
                Cache::from_env()
                    .path()
                    .join(
                        Repo::with_revision(repo_id.clone(), RepoType::Model, revision.clone())
                            .folder_name(),
                    )
                    .join("blobs"),
            ),
            _ => None,
        };
        DiskStatus {
            is_downloaded: self.resolve(models_dir, descriptor).is_ok(),
            is_downloading: downloading,
            partial_size: self
                .artifacts
                .iter()
                .map(|a| {
                    let local = local_dir
                        .join(format!("{}.partial", a.filename))
                        .metadata()
                        .map(|m| m.len().min(a.size_bytes))
                        .unwrap_or(0);
                    let hf = hf_blobs
                        .as_ref()
                        .map(|blobs| blobs.join(format!("{}.sync.part", a.sha256.to_lowercase())))
                        .map(|p| Self::hf_partial_size(&p, a.size_bytes))
                        .unwrap_or(0);
                    local.max(hf)
                })
                .sum(),
        }
    }

    /// Catalog defaults own their HF repository under the existing hard-delete
    /// policy. Removing that repository also removes interrupted HF transfers.
    pub(super) fn delete_files(models_dir: &Path, descriptor: &ModelDescriptor) -> Result<()> {
        let cache_dir = match &descriptor.source {
            ModelSource::HuggingFace { repo_id, revision } => Some(
                Cache::from_env().path().join(
                    Repo::with_revision(repo_id.clone(), RepoType::Model, revision.clone())
                        .folder_name(),
                ),
            ),
            _ => None,
        };
        Self::delete_in(
            &models_dir.join(Self::directory(&descriptor.id)),
            cache_dir.as_deref(),
        )
    }

    fn delete_in(local_dir: &Path, cache_dir: Option<&Path>) -> Result<()> {
        if let Some(cache_dir) = cache_dir {
            if cache_dir.exists() {
                fs::remove_dir_all(cache_dir)?;
            }
        }
        if local_dir.exists() {
            fs::remove_dir_all(local_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fixture() -> ModelPackage {
        let artifact = |role: &str, filename: &str, bytes: &[u8]| ModelArtifact {
            role: role.into(),
            filename: filename.into(),
            size_bytes: bytes.len() as u64,
            sha256: format!("{:x}", Sha256::digest(bytes)),
        };
        ModelPackage::try_from(vec![
            artifact("encoder", "encoder.gguf", b"encoder"),
            artifact("decoder", "nested/decoder.gguf", b"decoder"),
        ])
        .unwrap()
    }

    #[test]
    fn package_requires_every_complete_verified_artifact() {
        let dir = TempDir::new().unwrap();
        let package = fixture();
        fs::write(dir.path().join("encoder.gguf"), b"encoder").unwrap();
        assert!(package.resolve_with(dir.path(), |_| None).is_err());
        fs::create_dir(dir.path().join("nested")).unwrap();
        fs::write(dir.path().join("nested/decoder.gguf.partial"), b"decoder").unwrap();
        assert!(package.resolve_with(dir.path(), |_| None).is_err());
        fs::write(dir.path().join("nested/decoder.gguf"), b"dec").unwrap();
        assert!(package.resolve_with(dir.path(), |_| None).is_err());
        fs::write(dir.path().join("nested/decoder.gguf"), b"corrupt").unwrap();
        assert!(package.resolve_with(dir.path(), |_| None).is_err());
        fs::write(dir.path().join("nested/decoder.gguf"), b"decoder").unwrap();
        let resolved = package.resolve_with(dir.path(), |_| None).unwrap();
        assert_eq!(
            resolved,
            vec![
                ResolvedModelArtifact {
                    role: "encoder".into(),
                    path: dir.path().join("encoder.gguf")
                },
                ResolvedModelArtifact {
                    role: "decoder".into(),
                    path: dir.path().join("nested/decoder.gguf")
                },
            ]
        );
        assert_eq!(
            resolved,
            package.resolve_with(dir.path(), |_| None).unwrap()
        );
        fs::remove_file(dir.path().join("encoder.gguf")).unwrap();
        assert!(package.resolve_with(dir.path(), |_| None).is_err());
    }

    #[test]
    fn package_resolves_mixed_cache_and_local_files_and_rejects_bad_cache() {
        let cache = TempDir::new().unwrap();
        let local = TempDir::new().unwrap();
        let package = fixture();
        fs::write(cache.path().join("encoder.gguf"), b"encoder").unwrap();
        fs::create_dir(local.path().join("nested")).unwrap();
        fs::write(local.path().join("nested/decoder.gguf"), b"decoder").unwrap();
        let cached = |filename: &str| Some(cache.path().join(filename));
        let paths = package.resolve_with(local.path(), cached).unwrap();
        assert_eq!(paths[0].path, cache.path().join("encoder.gguf"));
        assert_eq!(paths[1].path, local.path().join("nested/decoder.gguf"));
        fs::write(cache.path().join("encoder.gguf"), b"invalid").unwrap();
        assert!(package.resolve_with(local.path(), cached).is_err());
        fs::write(local.path().join("encoder.gguf"), b"encoder").unwrap();
        assert_eq!(
            package.resolve_with(local.path(), cached).unwrap()[0].path,
            local.path().join("encoder.gguf")
        );
    }

    #[test]
    fn package_rejects_ambiguous_paths_roles_and_missing_integrity() {
        assert!(ModelPackage::try_from(vec![]).is_err());
        for filename in [
            "../escape",
            "/absolute",
            "C:/drive",
            "nested\\file",
            "x.partial",
            "a//b",
        ] {
            let mut artifacts = fixture().artifacts;
            artifacts[0].filename = filename.into();
            assert!(ModelPackage::try_from(artifacts).is_err(), "{filename}");
        }
        let mut artifacts = fixture().artifacts;
        artifacts[1].role = artifacts[0].role.clone();
        assert!(ModelPackage::try_from(artifacts).is_err());
        let mut artifacts = fixture().artifacts;
        artifacts[1].filename = "ENCODER.GGUF".into();
        assert!(ModelPackage::try_from(artifacts).is_err());
        let mut artifacts = fixture().artifacts;
        artifacts[0].sha256.clear();
        assert!(ModelPackage::try_from(artifacts).is_err());
    }

    #[test]
    fn package_status_tracks_partial_bytes_and_refreshes_missing_artifacts() {
        let dir = TempDir::new().unwrap();
        let package = fixture();
        let mut descriptor = crate::catalog::CATALOG[0].clone();
        descriptor.id = "test/package".into();
        descriptor.source = ModelSource::Local;
        descriptor.files.clear();
        descriptor.package = Some(package.clone());
        let root = dir.path().join(ModelPackage::directory(&descriptor.id));
        fs::create_dir_all(root.join("nested")).unwrap();
        fs::write(root.join("encoder.gguf.partial"), b"enc").unwrap();
        let status = package.disk_status(dir.path(), &descriptor, true);
        assert!(!status.is_downloaded);
        assert!(status.is_downloading);
        assert_eq!(status.partial_size, 3);
        fs::remove_file(root.join("encoder.gguf.partial")).unwrap();
        fs::write(root.join("encoder.gguf"), b"encoder").unwrap();
        fs::write(root.join("nested/decoder.gguf"), b"decoder").unwrap();
        let status = package.disk_status(dir.path(), &descriptor, false);
        assert!(status.is_downloaded);
        assert_eq!(status.partial_size, 0);
        fs::remove_file(root.join("nested/decoder.gguf")).unwrap();
        assert!(
            !package
                .disk_status(dir.path(), &descriptor, false)
                .is_downloaded
        );
    }

    #[test]
    fn hf_partial_progress_uses_committed_footer_instead_of_preallocated_length() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("blob.sync.part");
        let mut bytes = vec![0; 100];
        bytes.extend_from_slice(&25_u64.to_le_bytes());
        fs::write(&path, &bytes).unwrap();
        assert_eq!(ModelPackage::hf_partial_size(&path, 100), 25);
        assert_eq!(ModelPackage::hf_partial_size(&path, 99), 0);
        bytes[100..].copy_from_slice(&101_u64.to_le_bytes());
        fs::write(&path, &bytes).unwrap();
        assert_eq!(ModelPackage::hf_partial_size(&path, 100), 0);
    }

    #[test]
    fn package_progress_keeps_one_total_between_artifacts() {
        let progress = DownloadProgress {
            model_id: "package".into(),
            downloaded: 3,
            total: 7,
            percentage: 3.0 / 7.0 * 100.0,
        };
        let combined = progress.with_package_progress(Some((7, 14)));
        assert_eq!(combined.model_id, "package");
        assert_eq!(combined.downloaded, 10);
        assert_eq!(combined.total, 14);
        assert_eq!(combined.percentage, 10.0 / 14.0 * 100.0);
        let legacy = progress.with_package_progress(None);
        assert_eq!(legacy.total, progress.total);
        assert_eq!(legacy.downloaded, progress.downloaded);
        assert_eq!(legacy.percentage, progress.percentage);
    }

    #[test]
    fn package_delete_removes_complete_partial_and_hf_resume_files_and_can_repeat() {
        let dir = TempDir::new().unwrap();
        let local = dir.path().join("package");
        let cache = dir.path().join("models--org--repo");
        fs::create_dir_all(local.join("nested")).unwrap();
        fs::create_dir_all(cache.join("blobs")).unwrap();
        fs::write(local.join("encoder.gguf"), b"encoder").unwrap();
        fs::write(local.join("nested/decoder.gguf.partial"), b"dec").unwrap();
        fs::write(cache.join("blobs/hash.sync.part"), b"partial").unwrap();
        ModelPackage::delete_in(&local, Some(&cache)).unwrap();
        assert!(!local.exists());
        assert!(!cache.exists());
        ModelPackage::delete_in(&local, Some(&cache)).unwrap();
        fs::create_dir_all(local.join("nested")).unwrap();
        fs::write(local.join("encoder.gguf"), b"encoder").unwrap();
        fs::write(local.join("nested/decoder.gguf"), b"decoder").unwrap();
        assert!(fixture().resolve_with(&local, |_| None).is_ok());
    }

    #[test]
    fn package_error_or_cancellation_cleans_one_model_token_without_marking_ready() {
        let mut descriptor = crate::catalog::CATALOG[0].clone();
        descriptor.package = Some(fixture());
        descriptor.files.clear();
        let info = descriptor.to_model_info(&DiskStatus {
            is_downloading: true,
            ..Default::default()
        });
        let id = info.id.clone();
        let models = Mutex::new(HashMap::from([(id.clone(), info)]));
        let flags = Arc::new(Mutex::new(HashMap::from([(
            id.clone(),
            CancellationToken::new(),
        )])));
        {
            let _cleanup = DownloadCleanup {
                available_models: &models,
                cancel_flags: &flags,
                model_id: id.clone(),
                disarmed: false,
            };
            flags.lock().unwrap().get(&id).unwrap().cancel();
        }
        assert!(flags.lock().unwrap().is_empty());
        let models = models.lock().unwrap();
        assert!(!models[&id].is_downloading);
        assert!(!models[&id].is_downloaded);
    }
}
