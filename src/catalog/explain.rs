use std::path::Path;

use crate::error::AppError;

use super::matching::SkillMatch;
use super::types::{CatalogCache, ExplainMatch};

pub fn explain_skill_selection(
    cache: &mut CatalogCache,
    repo_root: &Path,
    skill_name: Option<&str>,
    intent: Option<&str>,
    source_name: Option<&str>,
) -> Result<ExplainMatch, AppError> {
    let catalog = cache.catalog();
    let selected = if let Some(name) = skill_name {
        if let Some(local) = catalog.find_local_skill(name) {
            SkillMatch::new(
                None,
                local.name.clone(),
                local.relative_path.clone(),
                local.metadata.clone(),
                1,
                vec!["matched explicit skill name".to_string()],
            )
        } else if let Some((source, remote)) = catalog.find_remote_skill(name, source_name) {
            SkillMatch::new(
                Some(source.name.clone()),
                remote.name.clone(),
                remote.path.clone(),
                remote.metadata.clone(),
                1,
                vec!["matched explicit skill name".to_string()],
            )
        } else {
            return Err(AppError::SkillNotFound(name.to_string()));
        }
    } else {
        let intent_text = intent.ok_or_else(|| {
            AppError::InputError("provide --skill or --intent for explain".to_string())
        })?;
        let index_path_for = |skill_path: &str| {
            repo_root
                .join(skill_path)
                .join("references")
                .join("index.toml")
        };
        let best = catalog
            .matches_for_intent(intent_text)
            .into_iter()
            .find(|m| {
                if m.source_name.is_some() && !index_path_for(&m.skill_path).is_file() {
                    return false;
                }
                true
            })
            .ok_or_else(|| AppError::SkillNotFound(intent_text.to_string()))?;
        SkillMatch::new(
            best.source_name,
            best.skill_name,
            best.skill_path,
            best.metadata,
            best.score,
            best.reasons,
        )
    };

    let index = cache.load_reference_index(repo_root, &selected.skill_path)?;
    let (reference, reference_reasons) = index.best_match(intent).ok_or_else(|| {
        AppError::ValidationError(format!(
            "no indexed references available for skill {}",
            selected.skill_name
        ))
    })?;
    let mut reasons = selected.reasons.clone();
    reasons.extend(reference_reasons);

    Ok(ExplainMatch {
        skill_name: selected.skill_name,
        source_name: selected.source_name,
        skill_path: selected.skill_path,
        skill_metadata: selected.metadata,
        reference,
        match_reasons: reasons,
    })
}
