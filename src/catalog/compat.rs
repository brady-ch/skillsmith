use serde::{Deserialize, Serialize};

use super::matching::tokenize;
use super::types::{
    Catalog, LocalSkill, RemoteSkill, RemoteSource, SkillRole, ToonMetadata, ToonNavigation,
    ToonText, ToonTrigger,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct CompatCatalog {
    #[serde(default)]
    pub(super) locals: Vec<CompatLocalSkill>,
    #[serde(default)]
    pub(super) sources: Vec<CompatRemoteSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct CompatLocalSkill {
    pub(super) name: String,
    pub(super) relative_path: String,
    #[serde(default)]
    pub(super) description: String,
    pub(super) metadata: Option<ToonMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct CompatRemoteSource {
    pub(super) name: String,
    pub(super) repo_url: String,
    #[serde(rename = "ref")]
    pub(super) git_ref: String,
    #[serde(default)]
    pub(super) skills: Vec<CompatRemoteSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct CompatRemoteSkill {
    pub(super) name: String,
    pub(super) path: String,
    #[serde(default)]
    pub(super) description: String,
    pub(super) metadata: Option<ToonMetadata>,
}

impl CompatCatalog {
    pub(super) fn into_catalog(self) -> Catalog {
        Catalog {
            locals: self
                .locals
                .into_iter()
                .map(|skill| LocalSkill {
                    name: skill.name.clone(),
                    relative_path: skill.relative_path,
                    metadata: skill
                        .metadata
                        .unwrap_or_else(|| migrate_description(&skill.name, &skill.description)),
                })
                .collect(),
            sources: self
                .sources
                .into_iter()
                .map(|source| RemoteSource {
                    name: source.name,
                    repo_url: source.repo_url,
                    git_ref: source.git_ref,
                    skills: source
                        .skills
                        .into_iter()
                        .map(|skill| {
                            let name = skill.name.clone();
                            RemoteSkill {
                                name,
                                path: skill.path,
                                metadata: skill.metadata.unwrap_or_else(|| {
                                    migrate_description(&skill.name, &skill.description)
                                }),
                            }
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}

fn migrate_description(name: &str, description: &str) -> ToonMetadata {
    let fallback = if description.trim().is_empty() {
        format!("Use when working with {}", name)
    } else {
        description.trim().to_string()
    };
    ToonMetadata {
        trigger: ToonTrigger {
            summary: fallback.clone(),
            intent_tags: tokenize(name),
            when_to_use: vec![fallback.clone()],
            skill_role: SkillRole::Implementation,
            order_weight: 0,
        },
        objective: ToonText {
            summary: fallback.clone(),
        },
        output: ToonText {
            summary: fallback.clone(),
        },
        navigation: ToonNavigation {
            summary: "Load the router first, then one matching reference.".to_string(),
            priority: 50,
        },
    }
}
