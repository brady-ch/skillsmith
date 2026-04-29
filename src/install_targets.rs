//! Preset install roots for Codex, Claude Code, and agents-style layouts.
use std::path::{Path, PathBuf};

use clap::ValueEnum;

/// Which tool’s skills directory layout to use under the chosen [`InstallScope`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum InstallInto {
    /// OpenAI Codex: `.codex/skills`
    Codex,
    /// Claude Code: `.claude/skills`
    Claude,
    /// Superpowers-style agents: `.agents/skills`
    Agents,
}

/// Global (home) vs project (current working directory).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum InstallScope {
    /// User home, e.g. ~/.codex/skills (without HOME, uses ./.codex/skills)
    #[default]
    Global,
    /// Current working directory, e.g. ./.codex/skills
    Project,
}

/// Resolve the directory that receives installed skill folders.
///
/// Precedence: explicit `--target` wins; otherwise `into` + `scope`. If `into` is omitted,
/// defaults to Codex (matching historical `default_install_root` behavior).
pub fn resolve_install_root(
    explicit_target: Option<PathBuf>,
    into: Option<InstallInto>,
    scope: InstallScope,
) -> PathBuf {
    if let Some(p) = explicit_target {
        return p;
    }
    let into = into.unwrap_or(InstallInto::Codex);
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    preset_root(into, scope, home.as_deref(), &cwd)
}

fn preset_root(into: InstallInto, scope: InstallScope, home: Option<&Path>, cwd: &Path) -> PathBuf {
    let (a, b) = match into {
        InstallInto::Codex => (".codex", "skills"),
        InstallInto::Claude => (".claude", "skills"),
        InstallInto::Agents => (".agents", "skills"),
    };
    match scope {
        InstallScope::Global => {
            let base = home.unwrap_or_else(|| Path::new("."));
            base.join(a).join(b)
        }
        InstallScope::Project => cwd.join(a).join(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preset_codex_global_uses_home() {
        let got = preset_root(
            InstallInto::Codex,
            InstallScope::Global,
            Some(Path::new("/home/u")),
            Path::new("/proj"),
        );
        assert_eq!(got, PathBuf::from("/home/u/.codex/skills"));
    }

    #[test]
    fn preset_claude_project_uses_cwd() {
        let got = preset_root(
            InstallInto::Claude,
            InstallScope::Project,
            Some(Path::new("/home/u")),
            Path::new("/proj/sub"),
        );
        assert_eq!(got, PathBuf::from("/proj/sub/.claude/skills"));
    }
}
