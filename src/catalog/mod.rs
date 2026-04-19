//! Catalog loading, TOON metadata validation, intent matching, and reference routing.

mod cache;
mod catalog_ops;
pub(crate) mod compat;
mod explain;
mod health;
mod matching;
mod recommend;
mod reference_index;
mod types;
mod validation;

pub use explain::explain_skill_selection;
pub use health::{health_check_local_skill, health_check_local_skill_minimal};
pub use matching::SkillMatch;
pub use recommend::recommend_for_intent;
pub use types::{
    Catalog, CatalogCache, CatalogHealthReport, ExplainMatch, LocalSkill, RecommendResponse,
    RecommendationEntry, ReferenceEntry, ReferenceIndex, RemoteSkill, RemoteSource, SkillRole,
    ToonMetadata, ToonNavigation, ToonText, ToonTrigger,
};
pub use validation::validate_relative_path;

#[cfg(test)]
mod tests;
