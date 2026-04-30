//! Path safety for fetching skill files relative to the skillsmith checkout.

use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::catalog::types::Catalog;
use crate::error::AppError;

/// Join `relative` beneath `repo_root` while rejecting escapes (`..`), absolute paths, and NULs.
pub fn join_repo_relative(repo_root: &Path, relative: &str) -> Result<PathBuf, AppError> {
    let trimmed = relative.trim();
    if trimmed.is_empty() {
        return Err(AppError::InputError(
            "empty relative path for skillsmith_fetch_file".into(),
        ));
    }
    if trimmed.as_bytes().contains(&0) || trimmed.contains('\\') {
        return Err(AppError::InputError(
            "disallowed characters in repository-relative path".into(),
        ));
    }
    let rel_path = Path::new(trimmed);
    if rel_path.is_absolute() {
        return Err(AppError::InputError(
            "absolute paths are not allowed for skillsmith_fetch_file".into(),
        ));
    }
    let mut acc = repo_root.to_path_buf();
    for comp in rel_path.components() {
        match comp {
            Component::Normal(part) => {
                let seg = part
                    .to_str()
                    .ok_or_else(|| AppError::InputError("non-utf8 path segment".into()))?;
                if seg == ".." || seg.contains("/") || seg.is_empty() {
                    return Err(AppError::InputError(
                        "relative path escapes repository root".into(),
                    ));
                }
                acc.push(part);
            }
            Component::CurDir => {}
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(AppError::InputError(
                    "relative path escapes repository root".into(),
                ));
            }
        }
    }
    Ok(acc)
}

pub fn catalog_skill_roots(catalog: &Catalog, repo_root: &Path) -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = catalog
        .locals
        .iter()
        .map(|l| repo_root.join(&l.relative_path))
        .collect();
    for source in &catalog.sources {
        for skill in &source.skills {
            roots.push(repo_root.join(&skill.path));
        }
    }
    roots.sort();
    roots.dedup();
    roots
}

/// True when `candidate` equals a root or lies under some listed prefix root directory.
pub fn is_under_catalog_skill(candidate: &Path, roots: &[PathBuf]) -> bool {
    for root in roots {
        if candidate == root.as_path() || candidate.starts_with(root) {
            return true;
        }
    }
    false
}

/// Read UTF-8 from `path` up to `max_bytes`, appending `[truncated]` when clipped.
pub fn read_utf8_capped(path: &Path, max_bytes: usize) -> Result<String, AppError> {
    let meta = fs::metadata(path)?;
    if meta.len() <= max_bytes as u64 {
        return fs::read_to_string(path).map_err(AppError::from);
    }

    let mut bytes = vec![0u8; max_bytes];
    let mut file = fs::File::open(path)?;
    let n = std::io::Read::read(&mut file, &mut bytes)?;
    bytes.truncate(n);
    let mut end = bytes.len();
    while end > 0 && std::str::from_utf8(&bytes[..end]).is_err() {
        end -= 1;
    }
    let slice = std::str::from_utf8(&bytes[..end])
        .map_err(|_| AppError::FilesystemError("file is not valid utf8 near truncation".into()))?;
    Ok(format!("{slice}...[truncated]"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn join_repo_relative_rejects_parent_segments() {
        let tmp = TempDir::new().expect("temp dir");
        let err = join_repo_relative(tmp.path(), "../etc/passwd").expect_err("expected err");
        assert!(matches!(err, AppError::InputError(_)));
    }

    #[test]
    fn join_repo_relative_resolves_plain_relative() {
        let tmp = TempDir::new().expect("temp dir");
        fs::write(tmp.path().join("README"), "hi").unwrap();
        let p = join_repo_relative(tmp.path(), "README").expect("join");
        assert_eq!(p, tmp.path().join("README"));
    }
}
