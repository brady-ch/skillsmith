//! Cursor `postToolUse` helper: follow up a Shell `skillsmith recommend` with compact routing text.

use std::path::Path;

use serde::Deserialize;
use serde_json::Value;

use crate::catalog::RecommendResponse;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
struct CursorPostToolEnvelope {
    tool_name: Option<String>,
    tool_input: Option<Value>,
    tool_output: Option<Value>,
}

fn shell_command_line(input: &Value) -> Option<String> {
    match input {
        Value::Object(m) => m
            .get("command")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        _ => None,
    }
}

fn stringify_tool_stdout(tool_output: &Value) -> Option<String> {
    match tool_output {
        Value::String(blob) => Some(blob.clone()),
        Value::Object(m) => {
            if let Some(Value::String(stdout)) = m.get("stdout") {
                return Some(stdout.clone());
            }
            serde_json::to_string(&Value::Object(m.clone())).ok()
        }
        _ => None,
    }
}

fn parse_recommend_stdout(text: &str) -> Option<RecommendResponse> {
    let trimmed = text.trim();
    serde_json::from_str::<RecommendResponse>(trimmed)
        .ok()
        .or_else(|| {
            let v: Value = serde_json::from_str(trimmed).ok()?;
            let inner = v.get("stdout")?.as_str()?;
            serde_json::from_str(inner).ok()
        })
}

/// Parse Cursor `postToolUse` JSON from stdin and emit `{"additional_context": "..."}` when the
/// shell command was a `skillsmith recommend` with parseable JSON output.
pub fn cursor_post_shell_recommend_followup(
    stdin: &[u8],
    _repo_root: &Path,
    _catalog_path: &Path,
    max_route_chars: usize,
) -> Result<Vec<u8>, AppError> {
    let envelope: CursorPostToolEnvelope = match serde_json::from_slice(stdin) {
        Ok(v) => v,
        Err(_) => return Ok(b"{}\n".to_vec()),
    };
    if envelope.tool_name.as_deref() != Some("Shell") {
        return Ok(b"{}\n".to_vec());
    }
    let cmd = envelope
        .tool_input
        .as_ref()
        .and_then(shell_command_line)
        .unwrap_or_default();
    let lc = cmd.to_lowercase();
    if !lc.contains("skillsmith") || !lc.contains("recommend") {
        return Ok(b"{}\n".to_vec());
    }
    let Some(blob) = envelope
        .tool_output
        .as_ref()
        .and_then(stringify_tool_stdout)
    else {
        return Ok(b"{}\n".to_vec());
    };
    let Some(parsed) = parse_recommend_stdout(&blob) else {
        return Ok(b"{}\n".to_vec());
    };
    let Some(top) = parsed.recommendations.first() else {
        return Ok(b"{}\n".to_vec());
    };
    let mut line = format!(
        "Skillsmith routing (shell recommend): skill={} path={}/SKILL.md ref={}/references/{}. Load SKILL.md → reference-router.md → ref only.",
        top.skill_name, top.skill_path, top.skill_path, top.suggested_reference_file,
    );
    if line.len() > max_route_chars {
        line.truncate(max_route_chars);
        line.push('…');
    }
    serde_json::to_vec(&serde_json::json!({ "additional_context": line })).map_err(AppError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn sample_recommend_json() -> String {
        serde_json::json!({
            "schema_version": 1,
            "intent": "test",
            "recommendations": [{
                "skill_name": "repo-scout",
                "source": null,
                "skill_path": "skills/repo-scout",
                "score": 3,
                "skill_role": "implementation",
                "order_weight": 0,
                "reasons": [],
                "suggested_reference_file": "repo-briefing-wenyan.md",
                "reference_reasons": []
            }]
        })
        .to_string()
    }

    #[test]
    fn post_shell_non_shell_tool_emits_empty_object() {
        let hook = serde_json::json!({
            "tool_name": "Read",
            "tool_input": {},
            "tool_output": ""
        });
        let out = cursor_post_shell_recommend_followup(
            hook.to_string().as_bytes(),
            Path::new("/tmp"),
            Path::new("/tmp"),
            400,
        )
        .expect("hook");
        let s = std::str::from_utf8(&out).unwrap().trim();
        assert_eq!(s, "{}");
    }

    #[test]
    fn post_shell_shell_recommend_string_stdout_injects_context() {
        let stdout = sample_recommend_json();
        let hook = serde_json::json!({
            "tool_name": "Shell",
            "tool_input": { "command": "skillsmith recommend --intent x --format json" },
            "tool_output": stdout
        });
        let out = cursor_post_shell_recommend_followup(
            hook.to_string().as_bytes(),
            Path::new("/tmp"),
            Path::new("/tmp"),
            500,
        )
        .expect("hook");
        let v: serde_json::Value = serde_json::from_slice(&out).expect("json");
        let ctx = v
            .get("additional_context")
            .and_then(|x| x.as_str())
            .expect("context");
        assert!(ctx.contains("repo-scout"));
        assert!(ctx.contains("skills/repo-scout"));
        assert!(ctx.contains("repo-briefing-wenyan.md"));
    }

    #[test]
    fn post_shell_parses_wrapped_shell_json_envelope() {
        let inner = sample_recommend_json();
        let hook = serde_json::json!({
            "tool_name": "Shell",
            "tool_input": { "command": "skillsmith recommend --intent x --format json" },
            "tool_output": { "stdout": inner, "stderr": "" }
        });
        let out = cursor_post_shell_recommend_followup(
            hook.to_string().as_bytes(),
            Path::new("/tmp"),
            Path::new("/tmp"),
            400,
        )
        .expect("hook");
        let v: serde_json::Value = serde_json::from_slice(&out).expect("json");
        assert!(v.get("additional_context").is_some());
    }
}
