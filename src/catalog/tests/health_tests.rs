//! Health checks for on-disk skill layout.

use std::fs;
use std::path::Path;

use tempfile::TempDir;

use crate::catalog::health::health_check_local_skill;
use crate::catalog::tests::helpers::test_metadata;
use crate::catalog::types::LocalSkill;

fn write_minimal_skill(skill_dir: &Path) {
    fs::create_dir_all(skill_dir.join("references")).expect("references");
    fs::write(
        skill_dir.join("references/reference-router.md"),
        "# Router\n\nSee index.\n",
    )
    .expect("router");
    fs::write(skill_dir.join("references/guide.md"), "# Guide\n\nBody.\n").expect("guide");
    fs::write(
        skill_dir.join("references/index.toml"),
        r#"[[references]]
file = "guide.md"

[references.metadata.trigger]
summary = "guide"
intent_tags = ["guide"]
when_to_use = ["guide"]

[references.metadata.objective]
summary = "o"

[references.metadata.output]
summary = "out"

[references.metadata.navigation]
summary = "nav"
priority = 10
"#,
    )
    .expect("index");
}

#[test]
fn strict_health_notice_when_token_hint_missing() {
    let tmp = TempDir::new().expect("tempdir");
    let rel = "skills/token-hint-test";
    let skill_dir = tmp.path().join(rel);
    write_minimal_skill(&skill_dir);

    let mut meta = test_metadata("test", &["guide"]);
    meta.token_hint = None;
    let skill = LocalSkill {
        name: "token-hint-test".to_string(),
        relative_path: rel.to_string(),
        metadata: meta,
    };

    let report = health_check_local_skill(tmp.path(), &skill).expect("health");
    assert!(report.issues.is_empty());
    assert!(
        report.notices.iter().any(|n| n.contains("token_hint")),
        "notices={:?}",
        report.notices
    );
}

#[test]
fn strict_health_no_token_hint_notice_when_set() {
    let tmp = TempDir::new().expect("tempdir");
    let rel = "skills/token-hint-set";
    let skill_dir = tmp.path().join(rel);
    write_minimal_skill(&skill_dir);

    let mut meta = test_metadata("test", &["guide"]);
    meta.token_hint = Some(1200);
    let skill = LocalSkill {
        name: "token-hint-set".to_string(),
        relative_path: rel.to_string(),
        metadata: meta,
    };

    let report = health_check_local_skill(tmp.path(), &skill).expect("health");
    assert!(report.issues.is_empty());
    assert!(report.notices.is_empty(), "notices={:?}", report.notices);
}
