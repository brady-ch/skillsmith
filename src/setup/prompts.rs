use std::path::{Path, PathBuf};

use dialoguer::{Confirm, Input, theme::ColorfulTheme};

use crate::error::AppError;

pub fn prompt_project_root(theme: &ColorfulTheme) -> Result<PathBuf, AppError> {
    let default_proj = std::env::current_dir()
        .map_err(|e| AppError::FilesystemError(e.to_string()))?
        .to_string_lossy()
        .to_string();
    let proj: String = Input::with_theme(theme)
        .with_prompt("Project root directory")
        .default(default_proj)
        .interact_text()
        .map_err(|e| AppError::InputError(e.to_string()))?;
    Ok(PathBuf::from(proj.trim()))
}

pub fn confirm_replace_hooks(theme: &ColorfulTheme, project_root: &Path) -> Result<bool, AppError> {
    let hooks_json = project_root.join(".cursor/hooks.json");
    let replace = !hooks_json.is_file()
        || Confirm::with_theme(theme)
            .with_prompt(
                "Replace existing .cursor/hooks.json? (No still writes hook scripts and bootstrap)",
            )
            .default(false)
            .interact()
            .map_err(|e| AppError::InputError(e.to_string()))?;
    Ok(replace)
}
