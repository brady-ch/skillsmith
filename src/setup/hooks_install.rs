use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::error::AppError;

const PORTABLE_SESSION: &str = include_str!("templates/portable-session-start.sh");
const INJECT_PROJECT: &str = include_str!("templates/inject-project-bootstrap.sh");
const CURSOR_HOOKS_JSON: &str = include_str!("templates/cursor-hooks.json");
const SESSION_BOOTSTRAP: &str = include_str!("templates/session-bootstrap.md");

fn set_executable(path: &Path) -> Result<(), AppError> {
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

/// Writes portable Cursor hook files under `project_root` (consumer project layout).
pub fn install_cursor_hooks(project_root: &Path, replace_hooks_json: bool) -> Result<(), AppError> {
    let cursor_hooks = project_root.join(".cursor/hooks");
    let smith_hooks = project_root.join(".skillsmith/hooks");
    fs::create_dir_all(&cursor_hooks)?;
    fs::create_dir_all(&smith_hooks)?;

    let portable = smith_hooks.join("portable-session-start.sh");
    let inject = cursor_hooks.join("inject-project-bootstrap.sh");
    fs::write(&portable, PORTABLE_SESSION)?;
    fs::write(&inject, INJECT_PROJECT)?;
    set_executable(&portable)?;
    set_executable(&inject)?;

    let hooks_json = project_root.join(".cursor/hooks.json");
    if replace_hooks_json {
        fs::create_dir_all(project_root.join(".cursor"))?;
        fs::write(&hooks_json, CURSOR_HOOKS_JSON)?;
    }

    let bootstrap = project_root.join(".skillsmith/session-bootstrap.md");
    fs::create_dir_all(bootstrap.parent().unwrap())?;
    if !bootstrap.is_file() {
        fs::write(&bootstrap, SESSION_BOOTSTRAP)?;
    }

    Ok(())
}
