use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;
use tempfile::TempDir;
use walkdir::WalkDir;

use crate::catalog::{
    Catalog, LocalSkill, ReferenceIndex, RemoteSkill, RemoteSource, validate_relative_path,
};
use crate::error::{AppError, fmt_path};

#[derive(Debug, Clone)]
pub struct InstallRequest {
    pub skill_name: String,
    pub source_name: Option<String>,
    pub target_root: PathBuf,
    pub force: bool,
    /// Symlink to the source skill directory instead of copying (local skills only).
    pub link: bool,
}

#[derive(Debug, Clone)]
pub struct InstallOutcome {
    pub skill_name: String,
    pub source_kind: String,
    pub installed_path: PathBuf,
}

pub fn install_skill(
    catalog: &Catalog,
    request: &InstallRequest,
    repo_root: &Path,
) -> Result<InstallOutcome, AppError> {
    if let Some(source_name) = &request.source_name {
        let source = catalog
            .find_source(source_name)
            .ok_or_else(|| AppError::SourceNotFound(source_name.clone()))?;
        let skill = source
            .skills
            .iter()
            .find(|entry| entry.name == request.skill_name)
            .ok_or_else(|| AppError::SkillNotFound(request.skill_name.clone()))?;
        return install_remote(source, skill, request);
    }

    if let Some(local) = catalog.find_local_skill(&request.skill_name) {
        return install_local(local, request, repo_root);
    }

    if let Some((source, skill)) = catalog.find_remote_skill(&request.skill_name, None) {
        return install_remote(source, skill, request);
    }

    Err(AppError::SkillNotFound(request.skill_name.clone()))
}

fn install_local(
    local: &LocalSkill,
    request: &InstallRequest,
    repo_root: &Path,
) -> Result<InstallOutcome, AppError> {
    validate_relative_path(&local.relative_path)?;
    let source = repo_root.join(&local.relative_path);
    ensure_valid_skill_directory(&source, &local.name)?;
    let installed_path = if request.link {
        symlink_skill_install(&source, &request.target_root, &local.name, request.force)?
    } else {
        stage_and_install(&source, &request.target_root, &local.name, request.force)?
    };
    Ok(InstallOutcome {
        skill_name: local.name.clone(),
        source_kind: "local".to_string(),
        installed_path,
    })
}

fn install_remote(
    source: &RemoteSource,
    skill: &RemoteSkill,
    request: &InstallRequest,
) -> Result<InstallOutcome, AppError> {
    if request.link {
        return Err(AppError::ValidationError(
            "--link is only supported for local catalog skills (not remote installs)".to_string(),
        ));
    }
    let scratch = clone_remote_source(source)?;
    install_remote_from_checkout(source, skill, request, scratch.path())
}

pub(crate) fn clone_remote_source(source: &RemoteSource) -> Result<TempDir, AppError> {
    let scratch = TempDir::new()?;
    clone_repo(source, scratch.path())?;
    Ok(scratch)
}

pub(crate) fn install_remote_from_checkout(
    source: &RemoteSource,
    skill: &RemoteSkill,
    request: &InstallRequest,
    checkout_root: &Path,
) -> Result<InstallOutcome, AppError> {
    validate_relative_path(&skill.path)?;
    let source_dir = checkout_root.join(&skill.path);
    ensure_valid_skill_directory(&source_dir, &skill.name)?;
    let installed_path = stage_and_install(
        &source_dir,
        &request.target_root,
        &skill.name,
        request.force,
    )?;

    Ok(InstallOutcome {
        skill_name: skill.name.clone(),
        source_kind: format!("remote:{}", source.name),
        installed_path,
    })
}

fn clone_repo(source: &RemoteSource, clone_dir: &Path) -> Result<(), AppError> {
    let status = Command::new("git")
        .arg("clone")
        .arg("--depth")
        .arg("1")
        .arg("--branch")
        .arg(&source.git_ref)
        .arg(&source.repo_url)
        .arg(clone_dir)
        .status()
        .map_err(|err| AppError::SourceFetchError(err.to_string()))?;

    if !status.success() {
        return Err(AppError::SourceFetchError(format!(
            "failed to clone {} at ref {}",
            source.repo_url, source.git_ref
        )));
    }

    Ok(())
}

fn ensure_valid_skill_directory(path: &Path, skill_name: &str) -> Result<(), AppError> {
    if !path.is_dir() {
        return Err(AppError::ValidationError(format!(
            "skill directory not found for {}: {}",
            skill_name,
            path.to_string_lossy()
        )));
    }

    let skill_md = path.join("SKILL.md");
    if !skill_md.is_file() {
        return Err(AppError::ValidationError(format!(
            "SKILL.md missing for {}: {}",
            skill_name,
            skill_md.to_string_lossy()
        )));
    }
    validate_skill_frontmatter(&skill_md)?;
    validate_references_layout(path, skill_name)?;
    Ok(())
}

fn validate_skill_frontmatter(skill_md_path: &Path) -> Result<(), AppError> {
    let content = fs::read_to_string(skill_md_path)?;
    let mut lines = content.lines();
    if lines.next() != Some("---") {
        return Err(AppError::ValidationError(format!(
            "SKILL.md missing frontmatter start: {}",
            skill_md_path.to_string_lossy()
        )));
    }

    let mut frontmatter = String::new();
    let mut found_end = false;
    for line in lines {
        if line == "---" {
            found_end = true;
            break;
        }
        frontmatter.push_str(line);
        frontmatter.push('\n');
    }

    if !found_end {
        return Err(AppError::ValidationError(format!(
            "SKILL.md missing frontmatter end: {}",
            skill_md_path.to_string_lossy()
        )));
    }

    #[derive(Deserialize)]
    struct SkillFrontmatter {
        name: String,
        description: String,
    }

    let parsed: SkillFrontmatter = serde_yaml::from_str(&frontmatter)?;
    if parsed.name.trim().is_empty() || parsed.description.trim().is_empty() {
        return Err(AppError::ValidationError(format!(
            "SKILL.md frontmatter requires non-empty name and description: {}",
            skill_md_path.to_string_lossy()
        )));
    }

    Ok(())
}

fn validate_references_layout(skill_dir: &Path, skill_name: &str) -> Result<(), AppError> {
    let references_dir = skill_dir.join("references");
    if !references_dir.is_dir() {
        return Err(AppError::ValidationError(format!(
            "references directory missing for {}: {}",
            skill_name,
            references_dir.to_string_lossy()
        )));
    }

    let router = references_dir.join("reference-router.md");
    if !router.is_file() {
        return Err(AppError::ValidationError(format!(
            "references/reference-router.md missing for {}: {}",
            skill_name,
            router.to_string_lossy()
        )));
    }

    let index_path = references_dir.join("index.toml");
    if !index_path.is_file() {
        return Err(AppError::ValidationError(format!(
            "references/index.toml missing for {}: {}",
            skill_name,
            index_path.to_string_lossy()
        )));
    }
    let index_content = fs::read_to_string(&index_path)?;
    let index: ReferenceIndex = toml::from_str(&index_content)?;
    index.validate(&index_path)?;

    let mut has_additional_reference = false;
    for entry in WalkDir::new(&references_dir) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(&references_dir)
            .map_err(|err| AppError::FilesystemError(err.to_string()))?;
        if relative != Path::new("reference-router.md") && relative != Path::new("index.toml") {
            has_additional_reference = true;
            break;
        }
    }

    if !has_additional_reference {
        return Err(AppError::ValidationError(format!(
            "at least one additional file is required under references/ for {}",
            skill_name
        )));
    }

    Ok(())
}

fn stage_and_install(
    source_dir: &Path,
    target_root: &Path,
    skill_name: &str,
    force: bool,
) -> Result<PathBuf, AppError> {
    fs::create_dir_all(target_root)?;
    let target_dir = target_root.join(skill_name);

    if target_dir.exists() && !force {
        return Err(AppError::InstallConflictError(format!(
            "target exists for {} at {}. rerun with --force",
            skill_name,
            target_dir.to_string_lossy()
        )));
    }

    let staging_parent = TempDir::new_in(target_root).map_err(|err| {
        AppError::FilesystemError(format!(
            "failed to create staging dir under {}: {}",
            target_root.to_string_lossy(),
            err
        ))
    })?;
    let staged_skill_dir = staging_parent.path().join(skill_name);
    copy_dir_recursive(source_dir, &staged_skill_dir)?;

    if target_dir.exists() {
        fs::remove_dir_all(&target_dir)?;
    }
    fs::rename(&staged_skill_dir, &target_dir)?;

    Ok(target_dir)
}

fn symlink_skill_install(
    source_dir: &Path,
    target_root: &Path,
    skill_name: &str,
    force: bool,
) -> Result<PathBuf, AppError> {
    fs::create_dir_all(target_root)?;
    let target_dir = target_root.join(skill_name);

    if target_dir.exists() {
        if !force {
            return Err(AppError::InstallConflictError(format!(
                "target exists for {} at {}. rerun with --force",
                skill_name,
                target_dir.to_string_lossy()
            )));
        }
        if target_dir.is_symlink() {
            fs::remove_file(&target_dir)?;
        } else {
            fs::remove_dir_all(&target_dir)?;
        }
    }

    let abs_source = fs::canonicalize(source_dir).map_err(|err| {
        AppError::FilesystemError(format!(
            "could not canonicalize {}: {}",
            source_dir.to_string_lossy(),
            err
        ))
    })?;

    create_symlink(&abs_source, &target_dir)?;
    Ok(target_dir)
}

fn create_symlink(source: &Path, dest: &Path) -> Result<(), AppError> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(source, dest)?;
        return Ok(());
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_dir(source, dest)?;
        return Ok(());
    }
    #[cfg(not(any(unix, windows)))]
    {
        let _ = (source, dest);
        Err(AppError::FilesystemError(
            "symlink install is not supported on this platform".to_string(),
        ))
    }
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<(), AppError> {
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let relative = entry
            .path()
            .strip_prefix(source)
            .map_err(|err| AppError::FilesystemError(err.to_string()))?;
        let target = destination.join(relative);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
            continue;
        }
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(entry.path(), &target)?;
        preserve_permissions(entry.path(), &target)?;
    }
    Ok(())
}

fn preserve_permissions(source: &Path, destination: &Path) -> Result<(), AppError> {
    let metadata = fs::metadata(source)?;
    fs::set_permissions(destination, metadata.permissions())?;
    Ok(())
}

pub fn summarize_install(outcome: &InstallOutcome) -> String {
    format!(
        "installed '{}' from {} into {}",
        outcome.skill_name,
        outcome.source_kind,
        fmt_path(&outcome.installed_path)
    )
}

pub fn check_git_installed() -> Result<(), AppError> {
    let output = Command::new("git")
        .arg("--version")
        .output()
        .map_err(|err| AppError::CommandError(err.to_string()))?;
    if output.status.success() {
        return Ok(());
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(AppError::CommandError(format!(
        "git is not available: {}",
        stderr.trim()
    )))
}

pub fn is_git_repo_url(url: &str) -> bool {
    let trimmed = url.trim();
    if trimmed.is_empty() || trimmed.contains(char::is_whitespace) {
        return false;
    }

    trimmed.starts_with("https://github.com/")
        || trimmed.starts_with("https://www.github.com/")
        || trimmed.starts_with("git@github.com:")
        || trimmed.starts_with("ssh://git@github.com/")
        || trimmed.starts_with("git://github.com/")
}

pub fn trim_to_owned<T: AsRef<OsStr>>(input: T) -> String {
    input.as_ref().to_string_lossy().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{
        RemoteSkill, RemoteSource, ToonMetadata, ToonNavigation, ToonText, ToonTrigger,
    };
    use tempfile::TempDir;

    fn metadata(summary: &str, tags: &[&str]) -> ToonMetadata {
        ToonMetadata {
            trigger: ToonTrigger {
                summary: summary.to_string(),
                intent_tags: tags.iter().map(|tag| tag.to_string()).collect(),
                when_to_use: vec![summary.to_string()],
                skill_role: Default::default(),
                order_weight: 0,
            },
            objective: ToonText {
                summary: summary.to_string(),
            },
            output: ToonText {
                summary: summary.to_string(),
            },
            navigation: ToonNavigation {
                summary: summary.to_string(),
                priority: 10,
            },
        }
    }

    fn write_skill(dir: &Path, name: &str, description: &str) {
        fs::create_dir_all(dir.join("agents")).expect("create agents dir");
        fs::create_dir_all(dir.join("references")).expect("create references dir");
        fs::write(
            dir.join("SKILL.md"),
            format!(
                "---\nname: {name}\ndescription: {description}\n---\n\n# {name}\n\n## Skill Inventory Note\n\n- example\n"
            ),
        )
        .expect("write skill");
        fs::write(
            dir.join("references/reference-router.md"),
            "# Reference Router\n",
        )
        .expect("write router");
        fs::write(dir.join("references/guide.md"), "# Guide\n").expect("write guide");
        fs::write(
            dir.join("references/index.toml"),
            r#"[[references]]
file = "guide.md"

[references.metadata.trigger]
summary = "Use the guide."
intent_tags = ["guide"]
when_to_use = ["Use the guide."]

[references.metadata.objective]
summary = "Guide objective."

[references.metadata.output]
summary = "Guide output."

[references.metadata.navigation]
summary = "Guide navigation."
priority = 10
"#,
        )
        .expect("write index");
        fs::write(
            dir.join("agents/openai.yaml"),
            "version: 1\ndisplay_name: Test\nshort_description: Test\ndefault_prompt: Test\n",
        )
        .expect("write agent");
    }

    fn init_git_repo(root: &Path) {
        let status = Command::new("git")
            .current_dir(root)
            .args(["init", "-b", "main", "."])
            .status()
            .expect("git init");
        assert!(status.success());
    }

    fn git_add_commit(root: &Path, message: &str) {
        for args in [vec!["add", "."], vec!["commit", "-m", message]] {
            let status = Command::new("git")
                .current_dir(root)
                .args(args)
                .env("GIT_AUTHOR_NAME", "Test")
                .env("GIT_AUTHOR_EMAIL", "test@example.com")
                .env("GIT_COMMITTER_NAME", "Test")
                .env("GIT_COMMITTER_EMAIL", "test@example.com")
                .status()
                .expect("git command");
            assert!(status.success());
        }
    }

    #[test]
    fn git_url_helper_accepts_common_github_forms() {
        assert!(is_git_repo_url("https://github.com/org/repo"));
        assert!(is_git_repo_url("git@github.com:org/repo.git"));
        assert!(is_git_repo_url("ssh://git@github.com/org/repo.git"));
    }

    #[test]
    fn git_url_helper_rejects_non_repo_strings() {
        assert!(!is_git_repo_url(""));
        assert!(!is_git_repo_url("not a url"));
        assert!(!is_git_repo_url("https://example.com/repo.git"));
        assert!(!is_git_repo_url("https://github.com/org/repo with spaces"));
    }

    #[test]
    fn installs_multiple_remote_skills_from_one_shared_checkout() {
        let remote = TempDir::new().expect("temp remote");
        let target = TempDir::new().expect("temp target");
        fs::create_dir_all(remote.path().join("skills/remote-a")).expect("create remote-a");
        fs::create_dir_all(remote.path().join("skills/remote-b")).expect("create remote-b");
        write_skill(&remote.path().join("skills/remote-a"), "remote-a", "desc a");
        write_skill(&remote.path().join("skills/remote-b"), "remote-b", "desc b");
        init_git_repo(remote.path());
        git_add_commit(remote.path(), "init");

        let source = RemoteSource {
            name: "remote".to_string(),
            repo_url: remote.path().to_string_lossy().to_string(),
            git_ref: "main".to_string(),
            skills: vec![
                RemoteSkill {
                    name: "remote-a".to_string(),
                    path: "skills/remote-a".to_string(),
                    metadata: metadata("A", &["a"]),
                },
                RemoteSkill {
                    name: "remote-b".to_string(),
                    path: "skills/remote-b".to_string(),
                    metadata: metadata("B", &["b"]),
                },
            ],
        };
        let checkout = clone_remote_source(&source).expect("clone once");

        let request_a = InstallRequest {
            skill_name: "remote-a".to_string(),
            source_name: Some(source.name.clone()),
            target_root: target.path().to_path_buf(),
            force: false,
            link: false,
        };
        let request_b = InstallRequest {
            skill_name: "remote-b".to_string(),
            source_name: Some(source.name.clone()),
            target_root: target.path().to_path_buf(),
            force: false,
            link: false,
        };

        let outcome_a =
            install_remote_from_checkout(&source, &source.skills[0], &request_a, checkout.path())
                .expect("install first skill");
        let outcome_b =
            install_remote_from_checkout(&source, &source.skills[1], &request_b, checkout.path())
                .expect("install second skill");

        assert!(outcome_a.installed_path.join("SKILL.md").exists());
        assert!(outcome_b.installed_path.join("SKILL.md").exists());
    }
}
