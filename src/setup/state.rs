use std::fs;
use std::path::Path;

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct SetupState {
    pub git_url: String,
    pub git_ref: String,
}

pub fn load_state(path: &Path) -> Result<SetupState, AppError> {
    let raw = fs::read_to_string(path)?;
    let v: toml::Value = raw
        .parse()
        .map_err(|e: toml::de::Error| AppError::SerializationError(e.to_string()))?;
    let git_url = v
        .get("git_url")
        .and_then(|x| x.as_str())
        .ok_or_else(|| AppError::SerializationError("setup.toml: missing git_url".into()))?
        .to_string();
    let git_ref = v
        .get("git_ref")
        .and_then(|x| x.as_str())
        .ok_or_else(|| AppError::SerializationError("setup.toml: missing git_ref".into()))?
        .to_string();
    Ok(SetupState { git_url, git_ref })
}

pub fn save_state(path: &Path, state: &SetupState) -> Result<(), AppError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let raw = format!(
        "git_url = {:?}\ngit_ref = {:?}\n",
        state.git_url, state.git_ref
    );
    fs::write(path, raw)?;
    Ok(())
}
