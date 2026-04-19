use std::fs;
use std::path::Path;
use std::process::Command;

use skillsmith::catalog::{
    Catalog, LocalSkill, ReferenceIndex, RemoteSkill, RemoteSource, SkillRole, ToonMetadata,
    ToonNavigation, ToonText, ToonTrigger, explain_skill_selection,
};
use skillsmith::installer::{InstallRequest, install_skill};
use tempfile::TempDir;

fn metadata(summary: &str, tags: &[&str]) -> ToonMetadata {
    ToonMetadata {
        trigger: ToonTrigger {
            summary: summary.to_string(),
            intent_tags: tags.iter().map(|tag| tag.to_string()).collect(),
            when_to_use: vec![summary.to_string()],
            skill_role: SkillRole::Implementation,
            order_weight: 0,
        },
        objective: ToonText {
            summary: format!("objective: {summary}"),
        },
        output: ToonText {
            summary: format!("output: {summary}"),
        },
        navigation: ToonNavigation {
            summary: "Load the router, then one indexed reference.".to_string(),
            priority: 10,
        },
    }
}

fn write_skill(dir: &Path, name: &str, description: &str) {
    fs::create_dir_all(dir.join("agents")).expect("create agents dir");
    fs::create_dir_all(dir.join("references")).expect("create references dir");
    let skill_md = format!(
        "---\nname: {name}\ndescription: {description}\n---\n\n# {name}\n\n## Skill Inventory Note\n\n- example\n",
    );
    fs::write(dir.join("SKILL.md"), skill_md).expect("write SKILL.md");
    fs::write(
        dir.join("references/reference-router.md"),
        "# Reference Router\n\nRoute to one reference file as needed.\n",
    )
    .expect("write reference-router.md");
    fs::write(
        dir.join("references/guide.md"),
        "# Guide\n\nDetailed content.\n",
    )
    .expect("write guide.md");
    fs::write(
        dir.join("references/index.toml"),
        r#"[[references]]
file = "guide.md"

[references.metadata.trigger]
summary = "Use when a guide is needed."
intent_tags = ["guide", "test"]
when_to_use = ["Use for test guidance."]

[references.metadata.objective]
summary = "Provide detailed test guidance."

[references.metadata.output]
summary = "Return test guidance."

[references.metadata.navigation]
summary = "Select the guide for test requests."
priority = 10
"#,
    )
    .expect("write index.toml");
    fs::write(
        dir.join("agents/openai.yaml"),
        "version: 1\ndisplay_name: Test\nshort_description: Test\ndefault_prompt: Test\n",
    )
    .expect("write openai.yaml");
}

fn local_catalog() -> Catalog {
    Catalog {
        locals: vec![LocalSkill {
            name: "repo-scout".to_string(),
            relative_path: "skills/repo-scout".to_string(),
            metadata: metadata("Inspect repositories", &["repo", "analysis"]),
        }],
        sources: Vec::new(),
    }
}

#[test]
fn installs_local_skill_successfully() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    write_skill(&repo.path().join("skills/repo-scout"), "repo-scout", "desc");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: false,
        link: false,
    };

    let outcome = install_skill(&catalog, &req, repo.path()).expect("install local");
    assert!(outcome.installed_path.join("SKILL.md").exists());
}

#[test]
fn blocks_conflict_without_force() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    write_skill(&repo.path().join("skills/repo-scout"), "repo-scout", "desc");
    write_skill(&target.path().join("repo-scout"), "repo-scout", "old");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: false,
        link: false,
    };

    let err = install_skill(&catalog, &req, repo.path()).expect_err("conflict expected");
    assert_eq!(err.code(), "install_conflict_error");
}

#[test]
fn replaces_conflict_with_force() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    write_skill(&repo.path().join("skills/repo-scout"), "repo-scout", "new");
    write_skill(&target.path().join("repo-scout"), "repo-scout", "old");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: true,
        link: false,
    };

    let outcome = install_skill(&catalog, &req, repo.path()).expect("forced install");
    let skill_md = fs::read_to_string(outcome.installed_path.join("SKILL.md")).expect("read");
    assert!(skill_md.contains("description: new"));
}

#[test]
fn fails_when_skill_md_missing() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    fs::create_dir_all(repo.path().join("skills/repo-scout")).expect("create skill folder");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: false,
        link: false,
    };
    let err = install_skill(&catalog, &req, repo.path()).expect_err("expected missing SKILL.md");
    assert_eq!(err.code(), "validation_error");
}

#[test]
fn fails_when_reference_router_missing() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    let skill_dir = repo.path().join("skills/repo-scout");
    write_skill(&skill_dir, "repo-scout", "desc");
    fs::remove_file(skill_dir.join("references/reference-router.md")).expect("remove router");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: false,
        link: false,
    };
    let err = install_skill(&catalog, &req, repo.path()).expect_err("expected router failure");
    assert_eq!(err.code(), "validation_error");
}

#[test]
fn fails_when_no_additional_reference_file_exists() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    let skill_dir = repo.path().join("skills/repo-scout");
    write_skill(&skill_dir, "repo-scout", "desc");
    fs::remove_file(skill_dir.join("references/guide.md")).expect("remove guide");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: false,
        link: false,
    };
    let err = install_skill(&catalog, &req, repo.path())
        .expect_err("expected additional reference requirement failure");
    assert_eq!(err.code(), "validation_error");
}

#[test]
fn fails_when_reference_index_missing() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    let skill_dir = repo.path().join("skills/repo-scout");
    write_skill(&skill_dir, "repo-scout", "desc");
    fs::remove_file(skill_dir.join("references/index.toml")).expect("remove index");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: false,
        link: false,
    };
    let err = install_skill(&catalog, &req, repo.path()).expect_err("expected index failure");
    assert_eq!(err.code(), "validation_error");
}

#[cfg(unix)]
#[test]
fn installs_local_skill_via_symlink() {
    let repo = TempDir::new().expect("temp repo");
    let target = TempDir::new().expect("temp target");
    write_skill(&repo.path().join("skills/repo-scout"), "repo-scout", "desc");

    let catalog = local_catalog();
    let req = InstallRequest {
        skill_name: "repo-scout".to_string(),
        source_name: None,
        target_root: target.path().to_path_buf(),
        force: false,
        link: true,
    };

    let outcome = install_skill(&catalog, &req, repo.path()).expect("symlink install");
    assert!(
        outcome.installed_path.is_symlink(),
        "expected symlink at {}",
        outcome.installed_path.display()
    );
    assert!(outcome.installed_path.join("SKILL.md").exists());
}

#[test]
fn installs_remote_skill_from_pinned_ref() {
    let remote = TempDir::new().expect("temp remote");
    let target = TempDir::new().expect("temp target");
    run(remote.path(), ["git", "init", "-b", "main", "."].as_slice());
    write_skill(
        &remote.path().join("skills/example-remote"),
        "example-remote",
        "desc",
    );
    run(remote.path(), ["git", "add", "."].as_slice());
    run(
        remote.path(),
        [
            "git",
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.com",
            "commit",
            "-m",
            "init",
        ]
        .as_slice(),
    );

    let catalog = Catalog {
        locals: Vec::new(),
        sources: vec![RemoteSource {
            name: "fixture-source".to_string(),
            repo_url: remote.path().to_string_lossy().to_string(),
            git_ref: "main".to_string(),
            skills: vec![RemoteSkill {
                name: "example-remote".to_string(),
                path: "skills/example-remote".to_string(),
                metadata: metadata("Install a remote example skill", &["remote", "install"]),
            }],
        }],
    };

    let req = InstallRequest {
        skill_name: "example-remote".to_string(),
        source_name: Some("fixture-source".to_string()),
        target_root: target.path().to_path_buf(),
        force: false,
        link: false,
    };
    let outcome = install_skill(&catalog, &req, remote.path()).expect("remote install");
    assert!(outcome.installed_path.join("SKILL.md").exists());
}

#[test]
fn explain_selects_indexed_reference_without_loading_all() {
    let repo = TempDir::new().expect("temp repo");
    let skill_dir = repo.path().join("skills/repo-scout");
    write_skill(&skill_dir, "repo-scout", "desc");

    let mut cache = skillsmith::catalog::CatalogCache::new(local_catalog());
    let explain = explain_skill_selection(
        &mut cache,
        repo.path(),
        Some("repo-scout"),
        Some("guide"),
        None,
    )
    .expect("explain selection");

    assert_eq!(explain.reference.file, "guide.md");
}

#[test]
fn explain_prefers_creational_skill_for_generic_builder_intent() {
    let repo = TempDir::new().expect("temp repo");
    let creational_dir = repo.path().join("skills/creational-pattern-architect");
    let rust_dir = repo.path().join("skills/rust-patterns-architecture");
    write_skill(
        &creational_dir,
        "creational-pattern-architect",
        "language-agnostic creational guidance",
    );
    write_skill(
        &rust_dir,
        "rust-patterns-architecture",
        "rust-specific design guidance",
    );

    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "creational-pattern-architect".to_string(),
                relative_path: "skills/creational-pattern-architect".to_string(),
                metadata: metadata(
                    "Choose creational design patterns for builders and factories",
                    &["creational", "builder", "factory", "construction"],
                ),
            },
            LocalSkill {
                name: "rust-patterns-architecture".to_string(),
                relative_path: "skills/rust-patterns-architecture".to_string(),
                metadata: metadata(
                    "Rust-specific architecture and ownership guidance",
                    &["rust", "architecture", "ownership", "api"],
                ),
            },
        ],
        sources: Vec::new(),
    };

    let mut cache = skillsmith::catalog::CatalogCache::new(catalog);
    let explain =
        explain_skill_selection(&mut cache, repo.path(), None, Some("builder pattern"), None)
            .expect("generic builder explain");

    assert_eq!(explain.skill_name, "creational-pattern-architect");
}

#[test]
fn explain_prefers_rust_skill_for_rust_specific_api_intent() {
    let repo = TempDir::new().expect("temp repo");
    let creational_dir = repo.path().join("skills/creational-pattern-architect");
    let rust_dir = repo.path().join("skills/rust-patterns-architecture");
    write_skill(
        &creational_dir,
        "creational-pattern-architect",
        "language-agnostic creational guidance",
    );
    write_skill(
        &rust_dir,
        "rust-patterns-architecture",
        "rust-specific design guidance",
    );

    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "creational-pattern-architect".to_string(),
                relative_path: "skills/creational-pattern-architect".to_string(),
                metadata: metadata(
                    "Choose creational design patterns for builders and factories",
                    &["creational", "builder", "factory", "construction"],
                ),
            },
            LocalSkill {
                name: "rust-patterns-architecture".to_string(),
                relative_path: "skills/rust-patterns-architecture".to_string(),
                metadata: metadata(
                    "Rust-specific architecture and ownership guidance",
                    &["rust", "architecture", "ownership", "api"],
                ),
            },
        ],
        sources: Vec::new(),
    };

    let mut cache = skillsmith::catalog::CatalogCache::new(catalog);
    let explain = explain_skill_selection(
        &mut cache,
        repo.path(),
        None,
        Some("Rust ownership-safe API design"),
        None,
    )
    .expect("rust explain");

    assert_eq!(explain.skill_name, "rust-patterns-architecture");
}

#[test]
fn explain_routes_behavioral_intent_to_article_file() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog =
        Catalog::load_from_file(&repo_root.join("catalog/catalog.toml")).expect("load catalog");
    let mut cache = skillsmith::catalog::CatalogCache::new(catalog);

    let explain =
        explain_skill_selection(&mut cache, repo_root, None, Some("observer pattern"), None)
            .expect("behavioral explain");

    assert_eq!(explain.skill_name, "behavioral-pattern-architect");
    assert_eq!(explain.reference.file, "observer.md");
}

#[test]
fn explain_routes_creational_intent_to_article_file() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog =
        Catalog::load_from_file(&repo_root.join("catalog/catalog.toml")).expect("load catalog");
    let mut cache = skillsmith::catalog::CatalogCache::new(catalog);

    let explain =
        explain_skill_selection(&mut cache, repo_root, None, Some("builder pattern"), None)
            .expect("creational explain");

    assert_eq!(explain.skill_name, "creational-pattern-architect");
    assert_eq!(explain.reference.file, "builder.md");
}

#[test]
fn explain_routes_concurrency_intent_to_article_file() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog =
        Catalog::load_from_file(&repo_root.join("catalog/catalog.toml")).expect("load catalog");
    let mut cache = skillsmith::catalog::CatalogCache::new(catalog);

    let explain = explain_skill_selection(
        &mut cache,
        repo_root,
        None,
        Some("thread pool pattern"),
        None,
    )
    .expect("concurrency explain");

    assert_eq!(explain.skill_name, "concurrency-pattern-architect");
    assert_eq!(explain.reference.file, "thread-pool.md");
}

#[test]
fn explain_routes_structural_intent_to_article_file() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let catalog =
        Catalog::load_from_file(&repo_root.join("catalog/catalog.toml")).expect("load catalog");
    let mut cache = skillsmith::catalog::CatalogCache::new(catalog);

    let explain =
        explain_skill_selection(&mut cache, repo_root, None, Some("adapter pattern"), None)
            .expect("structural explain");

    assert_eq!(explain.skill_name, "structural-pattern-architect");
    assert_eq!(explain.reference.file, "adapter.md");
}

#[test]
fn reference_index_validates() {
    let index: ReferenceIndex = toml::from_str(
        r#"[[references]]
file = "guide.md"

[references.metadata.trigger]
summary = "guide"
intent_tags = ["guide"]
when_to_use = ["Use guide."]

[references.metadata.objective]
summary = "objective"

[references.metadata.output]
summary = "output"

[references.metadata.navigation]
summary = "navigation"
priority = 10
"#,
    )
    .expect("parse index");
    assert!(index.validate(Path::new("index.toml")).is_ok());
}

fn run(cwd: &Path, args: &[&str]) {
    let (cmd, rest) = args.split_first().expect("command");
    let status = Command::new(cmd)
        .args(rest)
        .current_dir(cwd)
        .status()
        .expect("run command");
    assert!(status.success(), "command failed: {args:?}");
}
