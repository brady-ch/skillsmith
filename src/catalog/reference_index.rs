use std::collections::HashSet;
use std::path::Path;

use crate::error::AppError;

use super::matching::{metadata_match_score, tokenize};
use super::types::{LocalSkill, ReferenceEntry, ReferenceIndex, RemoteSkill};
use super::validation::{validate_relative_path, validate_toon_metadata};

impl ReferenceIndex {
    pub fn validate(&self, path: &Path) -> Result<(), AppError> {
        if self.references.is_empty() {
            return Err(AppError::ValidationError(format!(
                "reference index requires at least one entry: {}",
                path.to_string_lossy()
            )));
        }
        let mut names = HashSet::new();
        for entry in &self.references {
            if entry.file.trim().is_empty() {
                return Err(AppError::ValidationError(format!(
                    "reference entry missing file in {}",
                    path.to_string_lossy()
                )));
            }
            if !names.insert(entry.file.clone()) {
                return Err(AppError::ValidationError(format!(
                    "duplicate reference entry {} in {}",
                    entry.file,
                    path.to_string_lossy()
                )));
            }
            validate_relative_path(&entry.file)?;
            validate_toon_metadata(&entry.metadata, &format!("reference {}", entry.file))?;
        }
        Ok(())
    }

    pub fn best_match(&self, intent: Option<&str>) -> Option<(ReferenceEntry, Vec<String>)> {
        let Some(intent) = intent else {
            let mut entries = self.references.clone();
            entries.sort_by_key(|entry| entry.metadata.navigation.priority);
            let best = entries.into_iter().next()?;
            return Some((
                best,
                vec!["selected highest-priority reference".to_string()],
            ));
        };
        let tokens = tokenize(intent);
        let mut scored: Vec<(usize, ReferenceEntry, Vec<String>)> = self
            .references
            .iter()
            .map(|entry| {
                let (score, reasons) = metadata_match_score(&entry.metadata, &tokens);
                (score, entry.clone(), reasons)
            })
            .collect();
        scored.sort_by(|left, right| {
            right.0.cmp(&left.0).then_with(|| {
                left.1
                    .metadata
                    .navigation
                    .priority
                    .cmp(&right.1.metadata.navigation.priority)
            })
        });
        let (score, entry, mut reasons) = scored.into_iter().next()?;
        if score == 0 {
            reasons.push(
                "fell back to first indexed reference because no intent terms matched".to_string(),
            );
        }
        Some((entry, reasons))
    }
}

impl LocalSkill {
    pub fn short_description(&self) -> &str {
        &self.metadata.objective.summary
    }
}

impl RemoteSkill {
    pub fn short_description(&self) -> &str {
        &self.metadata.objective.summary
    }
}
