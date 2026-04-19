use std::fs;
use std::path::Path;

use crate::error::AppError;

use super::types::{CatalogHealthReport, LocalSkill, ReferenceIndex};

pub fn health_check_local_skill(
    repo_root: &Path,
    skill: &LocalSkill,
) -> Result<CatalogHealthReport, AppError> {
    let mut issues = Vec::new();
    let skill_dir = repo_root.join(&skill.relative_path);
    if !skill_dir.is_dir() {
        issues.push(format!(
            "missing local skill directory: {}",
            skill_dir.to_string_lossy()
        ));
        return Ok(CatalogHealthReport { issues });
    }

    let router = skill_dir.join("references").join("reference-router.md");
    if !router.is_file() {
        issues.push(format!(
            "missing reference router: {}",
            router.to_string_lossy()
        ));
    }

    let index_path = skill_dir.join("references").join("index.toml");
    if !index_path.is_file() {
        issues.push(format!(
            "missing reference index: {}",
            index_path.to_string_lossy()
        ));
        return Ok(CatalogHealthReport { issues });
    }

    let content = fs::read_to_string(&index_path)?;
    let index: ReferenceIndex = toml::from_str(&content)?;
    if let Err(err) = index.validate(&index_path) {
        issues.push(err.to_string());
        return Ok(CatalogHealthReport { issues });
    }

    for reference in &index.references {
        let path = skill_dir.join("references").join(&reference.file);
        if !path.is_file() {
            issues.push(format!(
                "missing indexed reference for {}: {}",
                skill.name,
                path.to_string_lossy()
            ));
        }
    }

    Ok(CatalogHealthReport { issues })
}

/// Minimal on-disk check for mixed-catalog / Superpowers-style trees (`validate --profile minimal`).
pub fn health_check_local_skill_minimal(
    repo_root: &Path,
    skill: &LocalSkill,
) -> Result<CatalogHealthReport, AppError> {
    let mut issues = Vec::new();
    let skill_dir = repo_root.join(&skill.relative_path);
    if !skill_dir.is_dir() {
        issues.push(format!(
            "missing local skill directory: {}",
            skill_dir.to_string_lossy()
        ));
        return Ok(CatalogHealthReport { issues });
    }

    let skill_md = skill_dir.join("SKILL.md");
    if !skill_md.is_file() {
        issues.push(format!(
            "SKILL.md missing for {}: {}",
            skill.name,
            skill_md.to_string_lossy()
        ));
    }

    Ok(CatalogHealthReport { issues })
}
