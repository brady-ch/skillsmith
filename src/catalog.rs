use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AppError;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ToonTrigger {
    pub summary: String,
    #[serde(default)]
    pub intent_tags: Vec<String>,
    #[serde(default)]
    pub when_to_use: Vec<String>,
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

#[derive(Debug, Clone)]
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
    catalog: Catalog,
    reference_indexes: HashMap<PathBuf, ReferenceIndex>,
}

impl Catalog {
    pub fn load_from_file(path: &Path) -> Result<Self, AppError> {
        let content = fs::read_to_string(path)?;
        let compat: CompatCatalog = toml::from_str(&content)?;
        let parsed = compat.into_catalog()?;
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

    pub fn matches_for_intent<'a>(&'a self, intent: &str) -> Vec<SkillMatch<'a>> {
        let tokens = tokenize(intent);
        let mut matches = Vec::new();

        for local in &self.locals {
            let (score, reasons) = metadata_match_score(&local.metadata, &tokens);
            if score > 0 {
                matches.push(SkillMatch::new(
                    None,
                    local.name.clone(),
                    local.relative_path.clone(),
                    local.metadata.clone(),
                    score,
                    reasons,
                ));
            }
        }

        for source in &self.sources {
            for skill in &source.skills {
                let (score, reasons) = metadata_match_score(&skill.metadata, &tokens);
                if score > 0 {
                    matches.push(SkillMatch::new(
                        Some(source.name.clone()),
                        skill.name.clone(),
                        skill.path.clone(),
                        skill.metadata.clone(),
                        score,
                        reasons,
                    ));
                }
            }
        }

        matches.sort_by(|left, right| {
            right
                .score
                .cmp(&left.score)
                .then_with(|| left.skill_name.cmp(&right.skill_name))
        });
        matches
    }
}

impl CatalogCache {
    pub fn new(catalog: Catalog) -> Self {
        Self {
            catalog,
            reference_indexes: HashMap::new(),
        }
    }

    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    pub fn load_reference_index(
        &mut self,
        repo_root: &Path,
        relative_skill_path: &str,
    ) -> Result<ReferenceIndex, AppError> {
        let path = repo_root
            .join(relative_skill_path)
            .join("references")
            .join("index.toml");
        if let Some(index) = self.reference_indexes.get(&path) {
            return Ok(index.clone());
        }
        let content = fs::read_to_string(&path)?;
        let index: ReferenceIndex = toml::from_str(&content)?;
        index.validate(&path)?;
        self.reference_indexes.insert(path, index.clone());
        Ok(index)
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

#[derive(Debug, Clone)]
pub struct SkillMatch<'a> {
    pub source_name: Option<String>,
    pub skill_name: String,
    pub skill_path: String,
    pub metadata: ToonMetadata,
    pub score: usize,
    pub reasons: Vec<String>,
    #[allow(dead_code)]
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> SkillMatch<'a> {
    fn new(
        source_name: Option<String>,
        skill_name: String,
        skill_path: String,
        metadata: ToonMetadata,
        score: usize,
        reasons: Vec<String>,
    ) -> Self {
        Self {
            source_name,
            skill_name,
            skill_path,
            metadata,
            score,
            reasons,
            marker: std::marker::PhantomData,
        }
    }
}

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
        let best = catalog
            .matches_for_intent(intent_text)
            .into_iter()
            .next()
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

fn validate_toon_metadata(metadata: &ToonMetadata, context: &str) -> Result<(), AppError> {
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

fn metadata_match_score(metadata: &ToonMetadata, tokens: &[String]) -> (usize, Vec<String>) {
    let mut score = 0;
    let mut reasons = Vec::new();
    for token in tokens {
        if metadata
            .trigger
            .intent_tags
            .iter()
            .any(|tag| tag.eq_ignore_ascii_case(token))
        {
            score += 3;
            reasons.push(format!("matched intent tag '{}'", token));
            continue;
        }
        if metadata
            .trigger
            .when_to_use
            .iter()
            .any(|entry| contains_token(entry, token))
        {
            score += 2;
            reasons.push(format!("matched trigger phrase '{}'", token));
            continue;
        }
        if contains_token(&metadata.trigger.summary, token)
            || contains_token(&metadata.objective.summary, token)
            || contains_token(&metadata.output.summary, token)
            || contains_token(&metadata.navigation.summary, token)
        {
            score += 1;
            reasons.push(format!("matched TOON summary '{}'", token));
        }
    }
    (score, reasons)
}

fn contains_token(haystack: &str, token: &str) -> bool {
    haystack.to_lowercase().contains(&token.to_lowercase())
}

fn tokenize(input: &str) -> Vec<String> {
    input
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-')
        .filter(|segment| !segment.trim().is_empty())
        .map(|segment| segment.trim().to_lowercase())
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CompatCatalog {
    #[serde(default)]
    locals: Vec<CompatLocalSkill>,
    #[serde(default)]
    sources: Vec<CompatRemoteSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CompatLocalSkill {
    name: String,
    relative_path: String,
    #[serde(default)]
    description: String,
    metadata: Option<ToonMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CompatRemoteSource {
    name: String,
    repo_url: String,
    #[serde(rename = "ref")]
    git_ref: String,
    #[serde(default)]
    skills: Vec<CompatRemoteSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CompatRemoteSkill {
    name: String,
    path: String,
    #[serde(default)]
    description: String,
    metadata: Option<ToonMetadata>,
}

impl CompatCatalog {
    fn into_catalog(self) -> Result<Catalog, AppError> {
        Ok(Catalog {
            locals: self
                .locals
                .into_iter()
                .map(|skill| {
                    Ok(LocalSkill {
                        name: skill.name.clone(),
                        relative_path: skill.relative_path,
                        metadata: skill.metadata.unwrap_or_else(|| {
                            migrate_description(&skill.name, &skill.description)
                        }),
                    })
                })
                .collect::<Result<Vec<_>, AppError>>()?,
            sources: self
                .sources
                .into_iter()
                .map(|source| {
                    Ok(RemoteSource {
                        name: source.name,
                        repo_url: source.repo_url,
                        git_ref: source.git_ref,
                        skills: source
                            .skills
                            .into_iter()
                            .map(|skill| {
                                let name = skill.name.clone();
                                Ok(RemoteSkill {
                                    name,
                                    path: skill.path,
                                    metadata: skill.metadata.unwrap_or_else(|| {
                                        migrate_description(&skill.name, &skill.description)
                                    }),
                                })
                            })
                            .collect::<Result<Vec<_>, AppError>>()?,
                    })
                })
                .collect::<Result<Vec<_>, AppError>>()?,
        })
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

#[cfg(test)]
mod tests {
    use super::*;

    fn metadata(summary: &str, tags: &[&str]) -> ToonMetadata {
        ToonMetadata {
            trigger: ToonTrigger {
                summary: summary.to_string(),
                intent_tags: tags.iter().map(|tag| tag.to_string()).collect(),
                when_to_use: vec![summary.to_string()],
            },
            objective: ToonText {
                summary: "objective".to_string(),
            },
            output: ToonText {
                summary: "output".to_string(),
            },
            navigation: ToonNavigation {
                summary: "navigation".to_string(),
                priority: 10,
            },
        }
    }

    fn valid_catalog() -> Catalog {
        Catalog {
            locals: vec![LocalSkill {
                name: "repo-scout".to_string(),
                relative_path: "skills/repo-scout".to_string(),
                metadata: metadata("Inspect repositories", &["repo", "analysis"]),
            }],
            sources: vec![RemoteSource {
                name: "source-a".to_string(),
                repo_url: "https://example.com/repo.git".to_string(),
                git_ref: "main".to_string(),
                skills: vec![RemoteSkill {
                    name: "remote-skill".to_string(),
                    path: "skills/remote-skill".to_string(),
                    metadata: metadata("Review migrations", &["migration"]),
                }],
            }],
        }
    }

    #[test]
    fn validates_successful_catalog() {
        let catalog = valid_catalog();
        assert!(catalog.validate().is_ok());
    }

    #[test]
    fn rejects_duplicate_local_names() {
        let mut catalog = valid_catalog();
        catalog.locals.push(LocalSkill {
            name: "repo-scout".to_string(),
            relative_path: "skills/repo-scout-2".to_string(),
            metadata: metadata("dup", &["dup"]),
        });
        assert!(catalog.validate().is_err());
    }

    #[test]
    fn rejects_parent_traversal() {
        let mut catalog = valid_catalog();
        catalog.locals[0].relative_path = "../bad".to_string();
        assert!(catalog.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_remote_name_in_source() {
        let mut catalog = valid_catalog();
        catalog.sources[0].skills.push(RemoteSkill {
            name: "remote-skill".to_string(),
            path: "skills/other".to_string(),
            metadata: metadata("other", &["other"]),
        });
        assert!(catalog.validate().is_err());
    }

    #[test]
    fn migrates_old_schema_description() {
        let input = r#"
[[locals]]
name = "repo-scout"
description = "Inspect repos"
relative_path = "skills/repo-scout"
"#;
        let compat: CompatCatalog = toml::from_str(input).expect("parse old schema");
        let catalog = compat.into_catalog().expect("migrate");
        assert_eq!(catalog.locals[0].metadata.trigger.summary, "Inspect repos");
        assert!(
            catalog.locals[0]
                .metadata
                .trigger
                .intent_tags
                .contains(&"repo-scout".to_string())
        );
    }

    #[test]
    fn intent_matching_prefers_tags() {
        let catalog = valid_catalog();
        let matches = catalog.matches_for_intent("migration rollback");
        assert_eq!(matches[0].skill_name, "remote-skill");
    }

    #[test]
    fn intent_matching_prefers_creational_skill_for_generic_builder_query() {
        let catalog = Catalog {
            locals: vec![
                LocalSkill {
                    name: "creational-pattern-architect".to_string(),
                    relative_path: "skills/creational-pattern-architect".to_string(),
                    metadata: metadata(
                        "Choose creational design patterns for object construction",
                        &["creational", "builder", "factory", "construction"],
                    ),
                },
                LocalSkill {
                    name: "rust-patterns-architecture".to_string(),
                    relative_path: "skills/rust-patterns-architecture".to_string(),
                    metadata: metadata(
                        "Rust-specific architecture and API guidance",
                        &["rust", "architecture", "ownership", "api"],
                    ),
                },
            ],
            sources: Vec::new(),
        };

        let matches = catalog.matches_for_intent("builder pattern");
        assert_eq!(matches[0].skill_name, "creational-pattern-architect");
    }

    #[test]
    fn intent_matching_prefers_rust_skill_for_rust_specific_query() {
        let catalog = Catalog {
            locals: vec![
                LocalSkill {
                    name: "creational-pattern-architect".to_string(),
                    relative_path: "skills/creational-pattern-architect".to_string(),
                    metadata: metadata(
                        "Choose creational design patterns for object construction",
                        &["creational", "builder", "factory", "construction"],
                    ),
                },
                LocalSkill {
                    name: "rust-patterns-architecture".to_string(),
                    relative_path: "skills/rust-patterns-architecture".to_string(),
                    metadata: metadata(
                        "Rust-specific architecture and API guidance",
                        &["rust", "architecture", "ownership", "api"],
                    ),
                },
            ],
            sources: Vec::new(),
        };

        let matches = catalog.matches_for_intent("Rust ownership-safe API design");
        assert_eq!(matches[0].skill_name, "rust-patterns-architecture");
    }
}
