use std::path::Path;

use crate::error::AppError;

use super::matching::SkillMatch;
use super::types::{CatalogCache, RecommendResponse, RecommendationEntry};

pub fn recommend_for_intent(
    cache: &mut CatalogCache,
    repo_root: &Path,
    intent: &str,
    limit: usize,
    skill_filter: Option<&str>,
    source_filter: Option<&str>,
    include_deprecated: bool,
) -> Result<RecommendResponse, AppError> {
    let mut matches: Vec<_> = {
        let catalog = cache.catalog();
        catalog
            .matches_for_intent(intent)
            .into_iter()
            .map(SkillMatch::into_parts)
            .collect()
    };

    if let Some(name) = skill_filter {
        matches.retain(|m| m.1 == name);
    }
    if let Some(src) = source_filter {
        matches.retain(|m| match src {
            "local" => m.0.is_none(),
            _ => m.0.as_deref() == Some(src),
        });
    }

    let take_n = if limit == 0 { usize::MAX } else { limit };
    let mut recommendations = Vec::new();

    for (source_name, skill_name, skill_path, metadata, score, reasons) in matches.into_iter() {
        if metadata.deprecated && !include_deprecated {
            continue;
        }
        if limit != 0 && recommendations.len() >= take_n {
            break;
        }
        let index_path = repo_root
            .join(&skill_path)
            .join("references")
            .join("index.toml");
        if source_name.is_some() && !index_path.is_file() {
            // Remote catalog entries point at paths inside another repo; skip until installed
            // or cloned alongside the catalog (see `install` / setup).
            continue;
        }
        let index = cache.load_reference_index(repo_root, &skill_path)?;
        let (reference, reference_reasons) = index.best_match(Some(intent)).ok_or_else(|| {
            AppError::ValidationError(format!(
                "no indexed references available for skill {}",
                skill_name
            ))
        })?;
        recommendations.push(RecommendationEntry {
            skill_name,
            source: source_name,
            skill_path,
            score,
            skill_role: metadata.trigger.skill_role,
            order_weight: metadata.trigger.order_weight,
            reasons,
            suggested_reference_file: reference.file,
            reference_reasons,
            deprecated: metadata.deprecated,
            token_hint: metadata.token_hint,
            tier: metadata.tier.clone(),
        });
    }

    Ok(RecommendResponse {
        schema_version: 1,
        intent: intent.to_string(),
        recommendations,
    })
}
