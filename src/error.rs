use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    SourceFetchError(String),
    #[error("{0}")]
    InstallConflictError(String),
    #[error("{0}")]
    FilesystemError(String),
    #[error("{0}")]
    InputError(String),
    #[error("skill not found: {0}")]
    SkillNotFound(String),
    #[error("source not found: {0}")]
    SourceNotFound(String),
    #[error("command failed: {0}")]
    CommandError(String),
    #[error("serialization error: {0}")]
    SerializationError(String),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::ValidationError(_) => "validation_error",
            Self::SourceFetchError(_) => "source_fetch_error",
            Self::InstallConflictError(_) => "install_conflict_error",
            Self::FilesystemError(_) => "filesystem_error",
            Self::InputError(_) => "input_error",
            Self::SkillNotFound(_) => "skill_not_found",
            Self::SourceNotFound(_) => "source_not_found",
            Self::CommandError(_) => "command_error",
            Self::SerializationError(_) => "serialization_error",
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::FilesystemError(value.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(value: toml::de::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

impl From<toml::ser::Error> for AppError {
    fn from(value: toml::ser::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

impl From<serde_yaml::Error> for AppError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

impl From<walkdir::Error> for AppError {
    fn from(value: walkdir::Error) -> Self {
        Self::FilesystemError(value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerializationError(value.to_string())
    }
}

pub fn fmt_path(path: &PathBuf) -> String {
    path.to_string_lossy().into_owned()
}
