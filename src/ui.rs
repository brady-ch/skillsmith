use std::path::{Path, PathBuf};

use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};

use crate::catalog::{
    Catalog, CatalogCache, LocalSkill, RemoteSkill, RemoteSource, explain_skill_selection,
    health_check_local_skill,
};
use crate::error::AppError;
use crate::installer::{InstallRequest, check_git_installed, install_skill, is_git_repo_url};

pub struct UiConfig {
    pub catalog_path: PathBuf,
    pub repo_root: PathBuf,
    pub initial_target: PathBuf,
}

pub fn run_menu(config: UiConfig) -> Result<(), AppError> {
    let theme = ColorfulTheme::default();
    let mut install_target = config.initial_target.clone();
    let mut cache = CatalogCache::new(Catalog::load_from_file(&config.catalog_path)?);

    loop {
        let health_summary = current_health_summary(cache.catalog(), &config.repo_root)?;
        let prompt = format!("Skillsmith [{}]", health_summary);
        let action = Select::with_theme(&theme)
            .with_prompt(prompt)
            .items(&[
                "Browse local skills",
                "Browse linked GitHub repos",
                "Explain routing",
                "Validate local skills",
                "Add GitHub repo reference",
                "Set install target",
                "Reload catalog",
                "Exit",
            ])
            .default(0)
            .interact()
            .map_err(|err| AppError::InputError(err.to_string()))?;

        match action {
            0 => browse_local(&theme, &mut cache, &config.repo_root, &install_target)?,
            1 => browse_remote(&theme, &mut cache, &config.repo_root, &install_target)?,
            2 => explain_routing(&theme, &mut cache, &config.repo_root)?,
            3 => print_validation(&cache, &config.repo_root)?,
            4 => {
                add_source_prompt(&theme, &config.catalog_path)?;
                cache = CatalogCache::new(Catalog::load_from_file(&config.catalog_path)?);
            }
            5 => {
                let value: String = Input::with_theme(&theme)
                    .with_prompt("Install target directory")
                    .with_initial_text(install_target.to_string_lossy().to_string())
                    .interact_text()
                    .map_err(|err| AppError::InputError(err.to_string()))?;
                install_target = PathBuf::from(value.trim());
            }
            6 => {
                cache = CatalogCache::new(Catalog::load_from_file(&config.catalog_path)?);
            }
            _ => return Ok(()),
        }
    }
}

fn browse_local(
    theme: &ColorfulTheme,
    cache: &mut CatalogCache,
    repo_root: &Path,
    install_target: &Path,
) -> Result<(), AppError> {
    if cache.catalog().locals.is_empty() {
        println!("No local skills in catalog.");
        return Ok(());
    }

    let intent = prompt_intent(theme)?;
    let matches = if let Some(intent) = &intent {
        cache
            .catalog()
            .matches_for_intent(intent)
            .into_iter()
            .filter(|entry| entry.source_name.is_none())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let locals: Vec<&LocalSkill> = if intent.is_some() {
        matches
            .iter()
            .filter_map(|matched| cache.catalog().find_local_skill(&matched.skill_name))
            .collect()
    } else {
        cache.catalog().locals.iter().collect()
    };

    if locals.is_empty() {
        println!("No local skills matched that intent.");
        return Ok(());
    }

    let items: Vec<String> = locals
        .iter()
        .map(|entry| {
            format!(
                "{} - {} [{}]",
                entry.name,
                entry.short_description(),
                entry.metadata.trigger.intent_tags.join(",")
            )
        })
        .collect();
    let selection = MultiSelect::with_theme(theme)
        .with_prompt("Local skills (Space: toggle, Enter: confirm, Esc: cancel)")
        .items(&items)
        .interact_opt()
        .map_err(|err| AppError::InputError(err.to_string()))?;

    let Some(indices) = selection else {
        return Ok(());
    };
    if indices.is_empty() {
        return Ok(());
    }

    let selected: Vec<&LocalSkill> = indices.iter().map(|&i| locals[i]).collect();
    install_batch_local(theme, cache.catalog(), repo_root, install_target, &selected)
}

fn browse_remote(
    theme: &ColorfulTheme,
    cache: &mut CatalogCache,
    repo_root: &Path,
    install_target: &Path,
) -> Result<(), AppError> {
    if cache.catalog().sources.is_empty() {
        println!("No linked sources in catalog.");
        return Ok(());
    }
    check_git_installed()?;

    let intent = prompt_intent(theme)?;
    let source_items: Vec<&RemoteSource> = if let Some(intent) = &intent {
        let mut filtered = Vec::new();
        for source in &cache.catalog().sources {
            if source.skills.iter().any(|skill| {
                skill
                    .metadata
                    .trigger
                    .intent_tags
                    .iter()
                    .any(|tag| intent.contains(tag))
            }) {
                filtered.push(source);
            }
        }
        if filtered.is_empty() {
            cache.catalog().sources.iter().collect()
        } else {
            filtered
        }
    } else {
        cache.catalog().sources.iter().collect()
    };

    let mut source_labels: Vec<String> = source_items
        .iter()
        .map(|entry| format!("{} ({})", entry.name, entry.repo_url))
        .collect();
    source_labels.push("Back".to_string());
    let source_idx = Select::with_theme(theme)
        .with_prompt("Linked GitHub repos")
        .items(&source_labels)
        .default(0)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    if source_idx >= source_items.len() {
        return Ok(());
    }

    let source = source_items[source_idx];
    let skills: Vec<&RemoteSkill> = if let Some(intent) = &intent {
        let filtered: Vec<&RemoteSkill> = source
            .skills
            .iter()
            .filter(|skill| {
                skill
                    .metadata
                    .trigger
                    .intent_tags
                    .iter()
                    .any(|tag| intent.contains(tag))
            })
            .collect();
        if filtered.is_empty() {
            source.skills.iter().collect()
        } else {
            filtered
        }
    } else {
        source.skills.iter().collect()
    };

    if skills.is_empty() {
        println!("No remote skills listed for source '{}'.", source.name);
        return Ok(());
    }

    let labels: Vec<String> = skills
        .iter()
        .map(|entry| {
            format!(
                "{} - {} [{}]",
                entry.name,
                entry.short_description(),
                entry.metadata.trigger.intent_tags.join(",")
            )
        })
        .collect();
    let selection = MultiSelect::with_theme(theme)
        .with_prompt(format!(
            "Remote skills in {} (Space: toggle, Enter: confirm, Esc: cancel)",
            source.name
        ))
        .items(&labels)
        .interact_opt()
        .map_err(|err| AppError::InputError(err.to_string()))?;

    let Some(indices) = selection else {
        return Ok(());
    };
    if indices.is_empty() {
        return Ok(());
    }

    let selected: Vec<&RemoteSkill> = indices.iter().map(|&i| skills[i]).collect();
    install_batch_remote(
        theme,
        cache.catalog(),
        repo_root,
        install_target,
        source,
        &selected,
    )
}

fn install_batch_local(
    theme: &ColorfulTheme,
    catalog: &Catalog,
    repo_root: &Path,
    install_target: &Path,
    skills: &[&LocalSkill],
) -> Result<(), AppError> {
    if skills.is_empty() {
        return Ok(());
    }

    let confirm_prompt = if skills.len() == 1 {
        format!(
            "Install '{}' into {} ?",
            skills[0].name,
            install_target.to_string_lossy()
        )
    } else {
        let names = skills
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "Install {} skills ({}) into {} ?",
            skills.len(),
            names,
            install_target.to_string_lossy()
        )
    };

    let confirm = Confirm::with_theme(theme)
        .with_prompt(confirm_prompt)
        .default(false)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    if !confirm {
        return Ok(());
    }

    let force = Confirm::with_theme(theme)
        .with_prompt("Allow overwrite with --force if target exists?")
        .default(false)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;

    for skill in skills {
        let request = InstallRequest {
            skill_name: skill.name.clone(),
            source_name: None,
            target_root: install_target.to_path_buf(),
            force,
            link: false,
        };
        let outcome = install_skill(catalog, &request, repo_root)?;
        println!(
            "Installed '{}' from {} into {}",
            outcome.skill_name,
            outcome.source_kind,
            outcome.installed_path.to_string_lossy()
        );
    }
    Ok(())
}

fn install_batch_remote(
    theme: &ColorfulTheme,
    catalog: &Catalog,
    repo_root: &Path,
    install_target: &Path,
    source: &RemoteSource,
    skills: &[&RemoteSkill],
) -> Result<(), AppError> {
    if skills.is_empty() {
        return Ok(());
    }

    let confirm_prompt = if skills.len() == 1 {
        format!(
            "Install '{}' from '{}' into {} ?",
            skills[0].name,
            source.name,
            install_target.to_string_lossy()
        )
    } else {
        let names = skills
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "Install {} skills from '{}' ({}) into {} ?",
            skills.len(),
            source.name,
            names,
            install_target.to_string_lossy()
        )
    };

    let confirm = Confirm::with_theme(theme)
        .with_prompt(confirm_prompt)
        .default(false)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    if !confirm {
        return Ok(());
    }
    let force = Confirm::with_theme(theme)
        .with_prompt("Allow overwrite with --force if target exists?")
        .default(false)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;

    for skill in skills {
        let request = InstallRequest {
            skill_name: skill.name.clone(),
            source_name: Some(source.name.clone()),
            target_root: install_target.to_path_buf(),
            force,
            link: false,
        };
        let outcome = install_skill(catalog, &request, repo_root)?;
        println!(
            "Installed '{}' from {} into {}",
            outcome.skill_name,
            outcome.source_kind,
            outcome.installed_path.to_string_lossy()
        );
    }
    Ok(())
}

fn explain_routing(
    theme: &ColorfulTheme,
    cache: &mut CatalogCache,
    repo_root: &Path,
) -> Result<(), AppError> {
    let skill: String = Input::with_theme(theme)
        .with_prompt("Skill name (leave blank to use intent)")
        .allow_empty(true)
        .interact_text()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    let intent = if skill.trim().is_empty() {
        let raw: String = Input::with_theme(theme)
            .with_prompt("Intent text")
            .interact_text()
            .map_err(|err| AppError::InputError(err.to_string()))?;
        Some(raw)
    } else {
        None
    };

    let explain = explain_skill_selection(
        cache,
        repo_root,
        if skill.trim().is_empty() {
            None
        } else {
            Some(skill.trim())
        },
        intent.as_deref(),
        None,
    )?;
    println!("Skill: {}", explain.skill_name);
    println!("Reference: {}", explain.reference.file);
    println!(
        "Intent tags: {}",
        explain.skill_metadata.trigger.intent_tags.join(",")
    );
    for reason in explain.match_reasons {
        println!("Reason: {}", reason);
    }
    Ok(())
}

fn print_validation(cache: &CatalogCache, repo_root: &Path) -> Result<(), AppError> {
    let mut issues = 0usize;
    for skill in &cache.catalog().locals {
        let report = health_check_local_skill(repo_root, skill)?;
        if report.issues.is_empty() {
            println!("OK {}", skill.name);
        } else {
            for issue in report.issues {
                issues += 1;
                println!("ISSUE {}", issue);
            }
        }
    }
    if issues == 0 {
        println!("Local skill validation passed.");
    }
    Ok(())
}

fn add_source_prompt(theme: &ColorfulTheme, catalog_path: &Path) -> Result<(), AppError> {
    let name: String = Input::with_theme(theme)
        .with_prompt("Source name")
        .interact_text()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    let repo_url: String = Input::with_theme(theme)
        .with_prompt("GitHub repo URL")
        .interact_text()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    if !is_git_repo_url(&repo_url) {
        return Err(AppError::ValidationError(
            "repo URL looks invalid; expected GitHub HTTPS/git URL".to_string(),
        ));
    }
    let git_ref: String = Input::with_theme(theme)
        .with_prompt("Git ref")
        .with_initial_text("main")
        .interact_text()
        .map_err(|err| AppError::InputError(err.to_string()))?;

    let mut catalog = Catalog::load_from_file(catalog_path)?;
    catalog.add_source(RemoteSource {
        name: name.trim().to_string(),
        repo_url: repo_url.trim().to_string(),
        git_ref: git_ref.trim().to_string(),
        skills: Vec::new(),
    })?;
    catalog.save_to_file(catalog_path)?;
    println!("Added source '{}'", name.trim());
    Ok(())
}

fn prompt_intent(theme: &ColorfulTheme) -> Result<Option<String>, AppError> {
    let value: String = Input::with_theme(theme)
        .with_prompt("Intent filter (blank for all skills)")
        .allow_empty(true)
        .interact_text()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Ok(None)
    } else {
        Ok(Some(trimmed.to_string()))
    }
}

fn current_health_summary(catalog: &Catalog, repo_root: &Path) -> Result<String, AppError> {
    let mut issue_count = 0usize;
    for skill in &catalog.locals {
        issue_count += health_check_local_skill(repo_root, skill)?.issues.len();
    }
    if issue_count == 0 {
        Ok("healthy".to_string())
    } else {
        Ok(format!("{} issue(s)", issue_count))
    }
}
