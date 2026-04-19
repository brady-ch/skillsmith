use std::path::{Path, PathBuf};
use std::process::Command;

use dialoguer::{Confirm, Input, theme::ColorfulTheme};

use crate::error::AppError;

use super::hooks_install::install_cursor_hooks;
use super::paths::{
    default_git_ref, default_git_url, ensure_data_dir, setup_state_path,
    skillsmith_env_snippet_path, upstream_checkout_dir,
};
use super::state::{SetupState, load_state, save_state};

fn run_git(repo: &Path, args: &[&str]) -> Result<(), AppError> {
    let st = Command::new("git")
        .current_dir(repo)
        .args(args)
        .status()
        .map_err(|e| AppError::CommandError(format!("git {}: {}", args.join(" "), e)))?;
    if !st.success() {
        return Err(AppError::CommandError(format!(
            "git {} exited with status {}",
            args.join(" "),
            st
        )));
    }
    Ok(())
}

fn sync_upstream(path: &Path, url: &str, git_ref: &str) -> Result<(), AppError> {
    if path.join(".git").is_dir() {
        run_git(path, &["fetch", "--depth", "1", "origin", git_ref])?;
        run_git(path, &["checkout", "FETCH_HEAD"])?;
        return Ok(());
    }

    if path.exists() {
        return Err(AppError::FilesystemError(format!(
            "{} exists and is not a git repository; remove it to clone again",
            path.display()
        )));
    }

    std::fs::create_dir_all(
        path.parent()
            .ok_or_else(|| AppError::FilesystemError("invalid upstream path".into()))?,
    )?;

    let st = Command::new("git")
        .args([
            "clone",
            "--depth",
            "1",
            "--branch",
            git_ref,
            url,
            path.to_str()
                .ok_or_else(|| AppError::FilesystemError("upstream path not UTF-8".into()))?,
        ])
        .status()
        .map_err(|e| AppError::CommandError(format!("git clone: {e}")))?;
    if !st.success() {
        return Err(AppError::CommandError(
            "git clone failed (check URL, branch, and network). For some tags you may need a full clone."
                .into(),
        ));
    }
    Ok(())
}

pub fn run_setup() -> Result<(), AppError> {
    let theme = ColorfulTheme::default();
    ensure_data_dir()?;

    let state_path = setup_state_path()
        .ok_or_else(|| AppError::FilesystemError("could not resolve setup state path".into()))?;
    let previous = load_state(&state_path).ok();

    let default_url = previous
        .as_ref()
        .map(|s| s.git_url.clone())
        .unwrap_or_else(default_git_url);
    let url: String = Input::with_theme(&theme)
        .with_prompt("Skillsmith git URL")
        .default(default_url)
        .interact_text()
        .map_err(|e| AppError::InputError(e.to_string()))?;

    let default_ref = previous
        .as_ref()
        .map(|s| s.git_ref.clone())
        .unwrap_or_else(default_git_ref);
    let git_ref: String = Input::with_theme(&theme)
        .with_prompt("Git branch or tag")
        .default(default_ref)
        .interact_text()
        .map_err(|e| AppError::InputError(e.to_string()))?;

    let upstream = upstream_checkout_dir()
        .ok_or_else(|| AppError::FilesystemError("could not resolve upstream path".into()))?;
    let url_trim = url.trim();
    let ref_trim = git_ref.trim();
    sync_upstream(&upstream, url_trim, ref_trim)?;

    save_state(
        &state_path,
        &SetupState {
            git_url: url_trim.to_string(),
            git_ref: ref_trim.to_string(),
        },
    )?;

    let env_path = skillsmith_env_snippet_path()
        .ok_or_else(|| AppError::FilesystemError("could not resolve skillsmith.env path".into()))?;
    let env_contents = format!(
        "# Source or paste into your shell profile:\nexport SKILLSMITH_REPO_ROOT=\"{}\"\n",
        upstream.display()
    );
    std::fs::write(&env_path, &env_contents)?;

    println!("\nCatalog checkout: {}", upstream.display());
    println!("{}", env_contents);
    println!("Wrote: {}", env_path.display());

    if !Confirm::with_theme(&theme)
        .with_prompt("Install Cursor session hook in a project?")
        .default(false)
        .interact()
        .map_err(|e| AppError::InputError(e.to_string()))?
    {
        return Ok(());
    }

    let default_proj = std::env::current_dir()
        .map_err(|e| AppError::FilesystemError(e.to_string()))?
        .to_string_lossy()
        .to_string();
    let proj: String = Input::with_theme(&theme)
        .with_prompt("Project root directory")
        .default(default_proj)
        .interact_text()
        .map_err(|e| AppError::InputError(e.to_string()))?;
    let proj_root = PathBuf::from(proj.trim());

    let hooks_json = proj_root.join(".cursor/hooks.json");
    let replace = if hooks_json.is_file() {
        Confirm::with_theme(&theme)
            .with_prompt(
                ".cursor/hooks.json already exists. Replace the entire file? (No still writes scripts and bootstrap if missing)",
            )
            .default(false)
            .interact()
            .map_err(|e| AppError::InputError(e.to_string()))?
    } else {
        true
    };

    install_cursor_hooks(&proj_root, replace)?;
    println!(
        "Cursor hooks installed under {} (.cursor/ and .skillsmith/)",
        proj_root.display()
    );

    Ok(())
}
