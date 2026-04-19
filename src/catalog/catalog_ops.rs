use std::collections::HashSet;
use std::fs;
use std::path::Path;

use crate::error::AppError;

use super::compat::CompatCatalog;
use super::types::{Catalog, LocalSkill, RemoteSkill, RemoteSource};
use super::validation::{validate_relative_path, validate_toon_metadata};

impl Catalog {
    pub fn load_from_file(path: &Path) -> Result<Self, AppError> {
        let content = fs::read_to_string(path)?;
        let compat: CompatCatalog = toml::from_str(&content)?;
        let parsed = compat.into_catalog();
        parsed.validate()?;
        Ok(parsed)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), AppError> {
        self.validate()?;
        let serialized = toml::to_string_pretty(self)?;
        fs::write(path, format!("{serialized}\n"))?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), AppError> {
        let mut local_names = HashSet::new();
        for local in &self.locals {
            if local.name.trim().is_empty() || local.relative_path.trim().is_empty() {
                return Err(AppError::ValidationError(
                    "local skill requires name and relative_path".to_string(),
                ));
            }
            if !local_names.insert(local.name.clone()) {
                return Err(AppError::ValidationError(format!(
                    "duplicate local skill name: {}",
                    local.name
                )));
            }
            validate_relative_path(&local.relative_path)?;
            validate_toon_metadata(&local.metadata, &format!("local skill {}", local.name))?;
        }

        let mut source_names = HashSet::new();
        for source in &self.sources {
            if source.name.trim().is_empty()
                || source.repo_url.trim().is_empty()
                || source.git_ref.trim().is_empty()
            {
                return Err(AppError::ValidationError(
                    "source requires name, repo_url, and ref".to_string(),
                ));
            }
            if !source_names.insert(source.name.clone()) {
                return Err(AppError::ValidationError(format!(
                    "duplicate source name: {}",
                    source.name
                )));
            }
            let mut remote_names = HashSet::new();
            for skill in &source.skills {
                if skill.name.trim().is_empty() || skill.path.trim().is_empty() {
                    return Err(AppError::ValidationError(format!(
                        "remote skill in source {} requires name and path",
                        source.name
                    )));
                }
                if !remote_names.insert(skill.name.clone()) {
                    return Err(AppError::ValidationError(format!(
                        "duplicate remote skill name in source {}: {}",
                        source.name, skill.name
                    )));
                }
                validate_relative_path(&skill.path)?;
                validate_toon_metadata(
                    &skill.metadata,
                    &format!("remote skill {} in source {}", skill.name, source.name),
                )?;
            }
        }

        Ok(())
    }

    pub fn add_source(&mut self, source: RemoteSource) -> Result<(), AppError> {
        self.sources.push(source);
        self.validate()
    }

    pub fn find_local_skill(&self, name: &str) -> Option<&LocalSkill> {
        self.locals.iter().find(|entry| entry.name == name)
    }

    pub fn find_source(&self, name: &str) -> Option<&RemoteSource> {
        self.sources.iter().find(|entry| entry.name == name)
    }

    pub fn find_remote_skill<'a>(
        &'a self,
        name: &str,
        source_filter: Option<&str>,
    ) -> Option<(&'a RemoteSource, &'a RemoteSkill)> {
        if let Some(source_name) = source_filter {
            let source = self.find_source(source_name)?;
            let skill = source.skills.iter().find(|entry| entry.name == name)?;
            return Some((source, skill));
        }

        for source in &self.sources {
            if let Some(skill) = source.skills.iter().find(|entry| entry.name == name) {
                return Some((source, skill));
            }
        }

        None
    }
}
