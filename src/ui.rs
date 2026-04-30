use std::path::{Path, PathBuf};

use dialoguer::{Confirm, Input, MultiSelect, Select, theme::ColorfulTheme};

use crate::catalog::{
    Catalog, CatalogCache, LocalSkill, RemoteSkill, RemoteSource, explain_skill_selection,
    health_check_local_skill,
};
use crate::error::AppError;
use crate::install_targets::{InstallInto, InstallScope, resolve_install_root};
use crate::installer::{
    InstallRequest, check_git_installed, clone_remote_source, install_remote_from_checkout,
    install_skill, is_git_repo_url,
};
use crate::setup::{
    confirm_replace_hooks, install_cursor_hooks, install_project_agent_rules, prompt_project_root,
};

pub struct UiConfig {
    pub catalog_path: PathBuf,
    pub repo_root: PathBuf,
    pub initial_target: PathBuf,
}

pub fn run_menu(config: UiConfig) -> Result<(), AppError> {
    let theme = ColorfulTheme::default();
    let mut install_targets = vec![config.initial_target.clone()];
    let mut cache = CatalogCache::new(Catalog::load_from_file(&config.catalog_path)?);

    loop {
        let health_summary = current_health_summary(cache.catalog(), &config.repo_root)?;
        let prompt = format!("Skillsmith [{}]", health_summary);
        let action = Select::with_theme(&theme)
            .with_prompt(prompt)
            .items(&[
                "Install skills from this package",
                "Install Cursor session hooks and agent rules",
                "Advanced…",
                "Exit",
            ])
            .default(0)
            .interact()
            .map_err(|err| AppError::InputError(err.to_string()))?;

        match action {
            0 => {
                install_targets = pick_install_destinations(&theme, &install_targets)?;
                browse_install_local(&theme, &mut cache, &config.repo_root, &install_targets)?;
            }
            1 => install_hooks_interactive(&theme, cache.catalog(), &config.repo_root)?,
            2 => advanced_menu(
                &theme,
                &mut cache,
                &config.catalog_path,
                &config.repo_root,
                &mut install_targets,
            )?,
            _ => return Ok(()),
        }
    }
}

/// Returns one or two roots (global + project) depending on user choice.
fn pick_install_destinations(
    theme: &ColorfulTheme,
    previous: &[PathBuf],
) -> Result<Vec<PathBuf>, AppError> {
    let preset = Select::with_theme(theme)
        .with_prompt("Where to install skills")
        .items(&[
            "Global — Codex (~/.codex/skills)",
            "Global — Claude (~/.claude/skills)",
            "Global — Agents (~/.agents/skills)",
            "This project — Codex (./.codex/skills)",
            "This project — Claude (./.claude/skills)",
            "This project — Agents (./.agents/skills)",
            "Global + this project — Codex (both)",
            "Global + this project — Claude (both)",
            "Global + this project — Agents (both)",
            "Custom directory…",
        ])
        .default(0)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;

    Ok(match preset {
        0 => vec![resolve_install_root(
            None,
            Some(InstallInto::Codex),
            InstallScope::Global,
        )],
        1 => vec![resolve_install_root(
            None,
            Some(InstallInto::Claude),
            InstallScope::Global,
        )],
        2 => vec![resolve_install_root(
            None,
            Some(InstallInto::Agents),
            InstallScope::Global,
        )],
        3 => vec![resolve_install_root(
            None,
            Some(InstallInto::Codex),
            InstallScope::Project,
        )],
        4 => vec![resolve_install_root(
            None,
            Some(InstallInto::Claude),
            InstallScope::Project,
        )],
        5 => vec![resolve_install_root(
            None,
            Some(InstallInto::Agents),
            InstallScope::Project,
        )],
        6 => vec![
            resolve_install_root(None, Some(InstallInto::Codex), InstallScope::Global),
            resolve_install_root(None, Some(InstallInto::Codex), InstallScope::Project),
        ],
        7 => vec![
            resolve_install_root(None, Some(InstallInto::Claude), InstallScope::Global),
            resolve_install_root(None, Some(InstallInto::Claude), InstallScope::Project),
        ],
        8 => vec![
            resolve_install_root(None, Some(InstallInto::Agents), InstallScope::Global),
            resolve_install_root(None, Some(InstallInto::Agents), InstallScope::Project),
        ],
        9 => {
            let initial = previous
                .first()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            let value: String = Input::with_theme(theme)
                .with_prompt("Install target directory")
                .with_initial_text(initial)
                .interact_text()
                .map_err(|err| AppError::InputError(err.to_string()))?;
            vec![PathBuf::from(value.trim())]
        }
        _ => unreachable!(),
    })
}

fn install_hooks_interactive(
    theme: &ColorfulTheme,
    catalog: &Catalog,
    repo_root: &Path,
) -> Result<(), AppError> {
    let proj_root = prompt_project_root(theme)?;
    let replace = confirm_replace_hooks(theme, &proj_root)?;

    install_project_agent_rules(&proj_root, catalog, repo_root)?;
    install_cursor_hooks(&proj_root, replace)?;
    println!(
        "Cursor hooks installed under {} (.cursor/ and .skillsmith/)",
        proj_root.display()
    );
    Ok(())
}

fn advanced_menu(
    theme: &ColorfulTheme,
    cache: &mut CatalogCache,
    catalog_path: &Path,
    repo_root: &Path,
    install_targets: &mut Vec<PathBuf>,
) -> Result<(), AppError> {
    let action = Select::with_theme(theme)
        .with_prompt("Advanced")
        .items(&[
            "Browse linked GitHub repos (remote skills)",
            "Explain routing",
            "Validate local skills",
            "Add GitHub repo reference",
            "Reload catalog",
            "Back",
        ])
        .default(0)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;

    match action {
        0 => {
            *install_targets = pick_install_destinations(theme, install_targets)?;
            browse_remote(theme, cache, install_targets)?;
        }
        1 => explain_routing(theme, cache, repo_root)?,
        2 => print_validation(cache, repo_root)?,
        3 => {
            add_source_prompt(theme, catalog_path)?;
            *cache = CatalogCache::new(Catalog::load_from_file(catalog_path)?);
        }
        4 => {
            *cache = CatalogCache::new(Catalog::load_from_file(catalog_path)?);
        }
        _ => {}
    }
    Ok(())
}

fn browse_install_local(
    theme: &ColorfulTheme,
    cache: &mut CatalogCache,
    repo_root: &Path,
    install_targets: &[PathBuf],
) -> Result<(), AppError> {
    if cache.catalog().locals.is_empty() {
        println!("No local skills in catalog.");
        return Ok(());
    }

    let locals: Vec<&LocalSkill> = cache.catalog().locals.iter().collect();

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
        .with_prompt("Skills to install (Space: toggle, Enter: confirm, Esc: cancel)")
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

    let dest_list = install_targets
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let names = selected
        .iter()
        .map(|s| s.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let confirm = Confirm::with_theme(theme)
        .with_prompt(format!(
            "Install {} skill(s) ({}) into [{}] ?",
            selected.len(),
            names,
            dest_list
        ))
        .default(true)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    if !confirm {
        return Ok(());
    }

    for target in install_targets {
        install_batch_local_no_prompts(cache.catalog(), repo_root, target, &selected)?;
    }
    Ok(())
}

fn browse_remote(
    theme: &ColorfulTheme,
    cache: &mut CatalogCache,
    install_targets: &[PathBuf],
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

    let dest_list = install_targets
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let names = selected
        .iter()
        .map(|s| s.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    let confirm = Confirm::with_theme(theme)
        .with_prompt(format!(
            "Install {} skill(s) from '{}' ({}) into [{}] ?",
            selected.len(),
            source.name,
            names,
            dest_list
        ))
        .default(true)
        .interact()
        .map_err(|err| AppError::InputError(err.to_string()))?;
    if !confirm {
        return Ok(());
    }

    for target in install_targets {
        install_batch_remote_no_prompts(target, source, &selected)?;
    }
    Ok(())
}

fn install_batch_local_no_prompts(
    catalog: &Catalog,
    repo_root: &Path,
    install_target: &Path,
    skills: &[&LocalSkill],
) -> Result<(), AppError> {
    if skills.is_empty() {
        return Ok(());
    }

    let force = false;
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

fn install_batch_remote_no_prompts(
    install_target: &Path,
    source: &RemoteSource,
    skills: &[&RemoteSkill],
) -> Result<(), AppError> {
    if skills.is_empty() {
        return Ok(());
    }

    let force = false;
    let checkout = clone_remote_source(source)?;
    for skill in skills {
        let request = InstallRequest {
            skill_name: skill.name.clone(),
            source_name: Some(source.name.clone()),
            target_root: install_target.to_path_buf(),
            force,
            link: false,
        };
        let outcome = install_remote_from_checkout(source, skill, &request, checkout.path())?;
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
