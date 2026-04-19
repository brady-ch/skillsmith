use std::fs;
use std::path::PathBuf;
use std::process::Command;

use clap::{Parser, Subcommand, ValueEnum};
use skillsmith::catalog::{
    Catalog, CatalogCache, ExplainMatch, LocalSkill, RecommendResponse, RemoteSource, SkillRole,
    explain_skill_selection, health_check_local_skill, health_check_local_skill_minimal,
    recommend_for_intent,
};
use skillsmith::error::AppError;
use skillsmith::installer::{InstallRequest, install_skill, summarize_install, trim_to_owned};
use skillsmith::setup::{resolve_catalog_paths, run_setup};
use skillsmith::ui::{UiConfig, run_menu};

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum ValidationProfile {
    /// Full skillsmith layout (reference router, index.toml, inventory note, etc.).
    #[default]
    Strict,
    /// Only `SKILL.md` present under each catalog skill path (for mixed / external trees).
    Minimal,
}

#[derive(Debug, Parser)]
#[command(name = "skillsmith", about = "Catalog and installer for Codex skills.")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Ui {
        #[arg(long)]
        target: Option<PathBuf>,
    },
    List {
        #[arg(long)]
        intent: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    Sources,
    Install {
        /// Skill name(s); repeat `--name` for each skill to install in one run
        #[arg(long, required = true, action = clap::ArgAction::Append)]
        name: Vec<String>,
        #[arg(long)]
        target: Option<PathBuf>,
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = false)]
        force: bool,
        /// Symlink instead of copy (local skills only; live updates when the repo changes).
        #[arg(long, default_value_t = false)]
        link: bool,
    },
    AddSource {
        #[arg(long)]
        name: String,
        #[arg(long)]
        repo: String,
        #[arg(long = "ref", default_value = "main")]
        git_ref: String,
    },
    Validate {
        #[arg(long)]
        source: Option<String>,
        #[arg(long, default_value_t = false)]
        remote: bool,
        #[arg(long, value_enum, default_value_t = ValidationProfile::Strict)]
        profile: ValidationProfile,
    },
    Explain {
        #[arg(long)]
        skill: Option<String>,
        #[arg(long)]
        intent: Option<String>,
        #[arg(long)]
        source: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Rank catalog skills and suggested reference files for an intent (agent-friendly JSON).
    Recommend {
        #[arg(long)]
        intent: String,
        #[arg(long, default_value_t = 10)]
        limit: usize,
        #[arg(long)]
        skill: Option<String>,
        #[arg(long)]
        source: Option<String>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Interactive install: clone catalog to data dir, print SKILLSMITH_REPO_ROOT, optional Cursor hooks.
    Setup,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("[{}] {}", err.code(), err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let cli = Cli::parse();
    let default_target = default_install_root();

    match cli.command {
        Some(Commands::Setup) => run_setup(),
        cmd => {
            let (repo_root, catalog_path) = resolve_catalog_paths()?;
            run_with_catalog(cmd, repo_root, catalog_path, default_target)
        }
    }
}

fn run_with_catalog(
    cmd: Option<Commands>,
    repo_root: PathBuf,
    catalog_path: PathBuf,
    default_target: PathBuf,
) -> Result<(), AppError> {
    match cmd {
        None => run_menu(UiConfig {
            catalog_path,
            repo_root,
            initial_target: default_target,
        }),
        Some(Commands::Ui { target }) => run_menu(UiConfig {
            catalog_path,
            repo_root,
            initial_target: target.unwrap_or(default_target),
        }),
        Some(Commands::List { intent, format }) => {
            let catalog = Catalog::load_from_file(&catalog_path)?;
            match format {
                OutputFormat::Text => {
                    if let Some(intent) = intent {
                        for matched in catalog.matches_for_intent(&intent) {
                            println!(
                                "{}\t{}\t{}",
                                matched.skill_name,
                                matched
                                    .source_name
                                    .clone()
                                    .unwrap_or_else(|| "local".to_string()),
                                matched.metadata.trigger.intent_tags.join(",")
                            );
                        }
                    } else {
                        for local in &catalog.locals {
                            println!(
                                "{}\tlocal\t{}",
                                local.name,
                                local.metadata.trigger.intent_tags.join(",")
                            );
                        }
                    }
                }
                OutputFormat::Json => {
                    if let Some(intent) = intent {
                        let rows: Vec<serde_json::Value> = catalog
                            .matches_for_intent(&intent)
                            .into_iter()
                            .map(|m| {
                                serde_json::json!({
                                    "skill_name": m.skill_name,
                                    "source": m.source_name.unwrap_or_else(|| "local".to_string()),
                                    "skill_path": m.skill_path,
                                    "score": m.score,
                                    "skill_role": m.metadata.trigger.skill_role,
                                    "order_weight": m.metadata.trigger.order_weight,
                                    "intent_tags": m.metadata.trigger.intent_tags,
                                    "reasons": m.reasons,
                                })
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&rows)?);
                    } else {
                        let rows: Vec<serde_json::Value> = catalog
                            .locals
                            .iter()
                            .map(|local| {
                                serde_json::json!({
                                    "skill_name": local.name,
                                    "source": "local",
                                    "skill_path": local.relative_path,
                                    "skill_role": local.metadata.trigger.skill_role,
                                    "order_weight": local.metadata.trigger.order_weight,
                                    "intent_tags": local.metadata.trigger.intent_tags,
                                })
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&rows)?);
                    }
                }
            }
            Ok(())
        }
        Some(Commands::Sources) => {
            let catalog = Catalog::load_from_file(&catalog_path)?;
            for source in catalog.sources {
                println!("{}\t{}\t{}", source.name, source.repo_url, source.git_ref);
                for skill in source.skills {
                    println!(
                        "  - {}\t{}\t{}",
                        skill.name,
                        skill.path,
                        skill.metadata.trigger.intent_tags.join(",")
                    );
                }
            }
            Ok(())
        }
        Some(Commands::Install {
            name,
            target,
            source,
            force,
            link,
        }) => {
            let catalog = Catalog::load_from_file(&catalog_path)?;
            let target_root = target.unwrap_or(default_target);
            let source_name = source.map(trim_to_owned);
            for skill_name in name {
                let request = InstallRequest {
                    skill_name: trim_to_owned(skill_name),
                    source_name: source_name.clone(),
                    target_root: target_root.clone(),
                    force,
                    link,
                };
                let outcome = install_skill(&catalog, &request, &repo_root)?;
                println!("{}", summarize_install(&outcome));
            }
            Ok(())
        }
        Some(Commands::AddSource {
            name,
            repo,
            git_ref,
        }) => {
            let mut catalog = Catalog::load_from_file(&catalog_path)?;
            catalog.add_source(RemoteSource {
                name: name.trim().to_string(),
                repo_url: repo.trim().to_string(),
                git_ref: git_ref.trim().to_string(),
                skills: Vec::new(),
            })?;
            catalog.save_to_file(&catalog_path)?;
            println!("added source '{}'", name.trim());
            Ok(())
        }
        Some(Commands::Validate {
            source,
            remote,
            profile,
        }) => {
            let catalog = Catalog::load_from_file(&catalog_path)?;
            let issues = validate_catalog_and_skills(
                &catalog,
                &repo_root,
                source.as_deref(),
                remote,
                profile,
            )?;
            if issues.is_empty() {
                println!("validation passed");
            } else {
                for issue in &issues {
                    println!("issue\t{}", issue);
                }
                return Err(AppError::ValidationError(format!(
                    "{} validation issue(s) found",
                    issues.len()
                )));
            }
            Ok(())
        }
        Some(Commands::Explain {
            skill,
            intent,
            source,
            format,
        }) => {
            let catalog = Catalog::load_from_file(&catalog_path)?;
            let mut cache = CatalogCache::new(catalog);
            let explain = explain_skill_selection(
                &mut cache,
                &repo_root,
                skill.as_deref(),
                intent.as_deref(),
                source.as_deref(),
            )?;
            match format {
                OutputFormat::Text => print_explain(explain),
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&explain)?),
            }
            Ok(())
        }
        Some(Commands::Recommend {
            intent,
            limit,
            skill,
            source,
            format,
        }) => {
            let catalog = Catalog::load_from_file(&catalog_path)?;
            let mut cache = CatalogCache::new(catalog);
            let response = recommend_for_intent(
                &mut cache,
                &repo_root,
                &intent,
                limit,
                skill.as_deref(),
                source.as_deref(),
            )?;
            match format {
                OutputFormat::Text => print_recommend_text(&response),
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&response)?),
            }
            Ok(())
        }
        Some(Commands::Setup) => unreachable!("setup is handled before resolve_catalog_paths"),
    }
}

fn skill_role_as_str(role: SkillRole) -> &'static str {
    match role {
        SkillRole::Process => "process",
        SkillRole::Meta => "meta",
        SkillRole::Implementation => "implementation",
    }
}

fn print_recommend_text(response: &RecommendResponse) {
    println!("intent\t{}", response.intent);
    for (i, rec) in response.recommendations.iter().enumerate() {
        println!("\n#{}", i + 1);
        println!("skill\t{}", rec.skill_name);
        println!(
            "source\t{}",
            rec.source.clone().unwrap_or_else(|| "local".to_string())
        );
        println!("path\t{}", rec.skill_path);
        println!("score\t{}", rec.score);
        println!("skill_role\t{}", skill_role_as_str(rec.skill_role));
        println!("order_weight\t{}", rec.order_weight);
        println!("suggested_reference\t{}", rec.suggested_reference_file);
        for r in &rec.reasons {
            println!("reason\t{}", r);
        }
        for r in &rec.reference_reasons {
            println!("reference_reason\t{}", r);
        }
    }
}

fn print_explain(explain: ExplainMatch) {
    println!("skill\t{}", explain.skill_name);
    println!(
        "source\t{}",
        explain.source_name.unwrap_or_else(|| "local".to_string())
    );
    println!("path\t{}", explain.skill_path);
    println!("selected_reference\t{}", explain.reference.file);
    println!(
        "intent_tags\t{}",
        explain.skill_metadata.trigger.intent_tags.join(",")
    );
    println!("trigger\t{}", explain.skill_metadata.trigger.summary);
    println!("objective\t{}", explain.skill_metadata.objective.summary);
    println!("navigation\t{}", explain.skill_metadata.navigation.summary);
    for reason in explain.match_reasons {
        println!("reason\t{}", reason);
    }
}

fn validate_catalog_and_skills(
    catalog: &Catalog,
    repo_root: &std::path::Path,
    source_filter: Option<&str>,
    remote: bool,
    profile: ValidationProfile,
) -> Result<Vec<String>, AppError> {
    let mut issues = Vec::new();

    for local in &catalog.locals {
        let report = match profile {
            ValidationProfile::Strict => health_check_local_skill(repo_root, local)?,
            ValidationProfile::Minimal => health_check_local_skill_minimal(repo_root, local)?,
        };
        issues.extend(report.issues);
        if matches!(profile, ValidationProfile::Strict) {
            issues.extend(validate_skill_inventory_note(repo_root, local)?);
        }
    }

    if remote {
        for source in &catalog.sources {
            if source_filter.is_some() && source_filter != Some(source.name.as_str()) {
                continue;
            }
            issues.extend(validate_remote_source_health(source)?);
        }
    }

    Ok(issues)
}

fn validate_skill_inventory_note(
    repo_root: &std::path::Path,
    skill: &LocalSkill,
) -> Result<Vec<String>, AppError> {
    let mut issues = Vec::new();
    let skill_md_path = repo_root.join(&skill.relative_path).join("SKILL.md");
    let content = fs::read_to_string(&skill_md_path)?;
    if !content.contains("## Skill Inventory Note") {
        issues.push(format!(
            "missing '## Skill Inventory Note' in {}",
            skill_md_path.to_string_lossy()
        ));
    }
    Ok(issues)
}

fn validate_remote_source_health(source: &RemoteSource) -> Result<Vec<String>, AppError> {
    let mut issues = Vec::new();
    let output = Command::new("git")
        .arg("ls-remote")
        .arg("--exit-code")
        .arg(&source.repo_url)
        .arg(&source.git_ref)
        .output();
    match output {
        Ok(output) if output.status.success() => {}
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            issues.push(format!(
                "remote source '{}' not reachable at ref '{}': {}",
                source.name,
                source.git_ref,
                stderr.trim()
            ));
        }
        Err(err) => {
            issues.push(format!(
                "remote source '{}' health check could not run: {}",
                source.name, err
            ));
        }
    }
    Ok(issues)
}

fn default_install_root() -> PathBuf {
    match std::env::var("HOME") {
        Ok(home) => PathBuf::from(home).join(".codex").join("skills"),
        Err(_) => PathBuf::from(".codex/skills"),
    }
}
