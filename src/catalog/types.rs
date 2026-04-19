use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    #[serde(default)]
    pub locals: Vec<LocalSkill>,
    #[serde(default)]
    pub sources: Vec<RemoteSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalSkill {
    pub name: String,
    pub relative_path: String,
    pub metadata: ToonMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSource {
    pub name: String,
    pub repo_url: String,
    #[serde(rename = "ref")]
    pub git_ref: String,
    #[serde(default)]
    pub skills: Vec<RemoteSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteSkill {
    pub name: String,
    pub path: String,
    pub metadata: ToonMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToonMetadata {
    pub trigger: ToonTrigger,
    pub objective: ToonText,
    pub output: ToonText,
    pub navigation: ToonNavigation,
}

/// Cross-skill ordering: `process` skills sort before `meta`, then `implementation`
/// (Superpowers-style “process before implementation” when scores tie).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SkillRole {
    Process,
    Meta,
    #[default]
    Implementation,
}

impl SkillRole {
    pub fn sort_key(self) -> u8 {
        match self {
            SkillRole::Process => 0,
            SkillRole::Meta => 1,
            SkillRole::Implementation => 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToonTrigger {
    pub summary: String,
    #[serde(default)]
    pub intent_tags: Vec<String>,
    #[serde(default)]
    pub when_to_use: Vec<String>,
    #[serde(default)]
    pub skill_role: SkillRole,
    /// Lower values sort earlier among skills with the same match score and role.
    #[serde(default)]
    pub order_weight: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToonText {
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToonNavigation {
    pub summary: String,
    #[serde(default)]
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceIndex {
    #[serde(default)]
    pub references: Vec<ReferenceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceEntry {
    pub file: String,
    pub metadata: ToonMetadata,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExplainMatch {
    pub skill_name: String,
    pub source_name: Option<String>,
    pub skill_path: String,
    pub skill_metadata: ToonMetadata,
    pub reference: ReferenceEntry,
    pub match_reasons: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CatalogHealthReport {
    pub issues: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CatalogCache {
    pub(super) catalog: Catalog,
    pub(super) reference_indexes: HashMap<PathBuf, ReferenceIndex>,
}

/// Stable JSON output for agents: ranked skills with a suggested reference file each (`schema_version` 1).
#[derive(Debug, Clone, Serialize)]
pub struct RecommendResponse {
    pub schema_version: u32,
    pub intent: String,
    pub recommendations: Vec<RecommendationEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecommendationEntry {
    pub skill_name: String,
    pub source: Option<String>,
    pub skill_path: String,
    pub score: usize,
    pub skill_role: SkillRole,
    pub order_weight: i32,
    pub reasons: Vec<String>,
    pub suggested_reference_file: String,
    pub reference_reasons: Vec<String>,
}
