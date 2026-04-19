use std::collections::HashSet;
use std::path::PathBuf;

use crate::error::AppError;

use super::types::ToonMetadata;

pub fn validate_relative_path(path: &str) -> Result<(), AppError> {
    let pb = PathBuf::from(path);
    if pb.is_absolute() {
        return Err(AppError::ValidationError(format!(
            "path must be relative, got absolute path: {}",
            path
        )));
    }
    for component in pb.components() {
        if matches!(component, std::path::Component::ParentDir) {
            return Err(AppError::ValidationError(format!(
                "path must not contain parent traversal: {}",
                path
            )));
        }
    }
    Ok(())
}

pub(crate) fn validate_toon_metadata(
    metadata: &ToonMetadata,
    context: &str,
) -> Result<(), AppError> {
    if metadata.trigger.summary.trim().is_empty() {
        return Err(AppError::ValidationError(format!(
            "missing trigger.summary for {}",
            context
        )));
    }
    if metadata.trigger.intent_tags.is_empty() {
        return Err(AppError::ValidationError(format!(
            "missing trigger.intent_tags for {}",
            context
        )));
    }
    if metadata.objective.summary.trim().is_empty() {
        return Err(AppError::ValidationError(format!(
            "missing objective.summary for {}",
            context
        )));
    }
    if metadata.output.summary.trim().is_empty() {
        return Err(AppError::ValidationError(format!(
            "missing output.summary for {}",
            context
        )));
    }
    if metadata.navigation.summary.trim().is_empty() {
        return Err(AppError::ValidationError(format!(
            "missing navigation.summary for {}",
            context
        )));
    }
    let mut seen = HashSet::new();
    for tag in &metadata.trigger.intent_tags {
        if tag.trim().is_empty() {
            return Err(AppError::ValidationError(format!(
                "empty intent tag for {}",
                context
            )));
        }
        if !seen.insert(tag.to_lowercase()) {
            return Err(AppError::ValidationError(format!(
                "duplicate intent tag '{}' for {}",
                tag, context
            )));
        }
    }
    Ok(())
}
