//! Data directory and catalog resolution for skillsmith installs.

use std::path::PathBuf;

use crate::error::AppError;

pub const CATALOG_REL: &str = "catalog/catalog.toml";

pub fn skillsmith_data_dir() -> Option<PathBuf> {
    dirs::data_local_dir().map(|d| d.join("skillsmith"))
}

pub fn upstream_checkout_dir() -> Option<PathBuf> {
    skillsmith_data_dir().map(|d| d.join("upstream"))
}

pub fn setup_state_path() -> Option<PathBuf> {
    skillsmith_data_dir().map(|d| d.join("setup.toml"))
}

pub fn skillsmith_env_snippet_path() -> Option<PathBuf> {
    skillsmith_data_dir().map(|d| d.join("skillsmith.env"))
}

/// Resolve `(repo_root, catalog_path)` for CLI and TUI.
///
/// Order: `SKILLSMITH_REPO_ROOT`, then `./catalog/catalog.toml` in cwd, then data-dir upstream.
pub fn resolve_catalog_paths() -> Result<(PathBuf, PathBuf), AppError> {
    let cwd = std::env::current_dir().map_err(|e| AppError::FilesystemError(e.to_string()))?;

    if let Ok(env_root) = std::env::var("SKILLSMITH_REPO_ROOT") {
        let p = PathBuf::from(env_root);
        let cat = p.join(CATALOG_REL);
        if cat.is_file() {
            return Ok((p, cat));
        }
        return Err(AppError::FilesystemError(format!(
            "SKILLSMITH_REPO_ROOT is set to {} but {} is not a file",
            p.display(),
            cat.display()
        )));
    }

    let here = cwd.join(CATALOG_REL);
    if here.is_file() {
        return Ok((cwd.clone(), here));
    }

    if let Some(up) = upstream_checkout_dir() {
        let cat = up.join(CATALOG_REL);
        if cat.is_file() {
            return Ok((up, cat));
        }
    }

    Err(AppError::FilesystemError(
        "could not find catalog/catalog.toml. Run `skillsmith setup` or set SKILLSMITH_REPO_ROOT."
            .into(),
    ))
}

pub fn ensure_data_dir() -> Result<PathBuf, AppError> {
    let base = skillsmith_data_dir()
        .ok_or_else(|| AppError::FilesystemError("could not resolve data_local_dir".into()))?;
    std::fs::create_dir_all(&base)?;
    Ok(base)
}

/// Default matches `[package].repository` in Cargo.toml (`CARGO_PKG_REPOSITORY` when built with Cargo).
pub fn default_git_url() -> String {
    std::env::var("SKILLSMITH_GIT_URL").unwrap_or_else(|_| {
        option_env!("CARGO_PKG_REPOSITORY")
            .unwrap_or("https://github.com/brady-ch/skillsmith")
            .to_string()
    })
}

pub fn default_git_ref() -> String {
    std::env::var("SKILLSMITH_GIT_REF").unwrap_or_else(|_| "main".to_string())
}
