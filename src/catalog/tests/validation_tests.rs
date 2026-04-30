use crate::catalog::tests::helpers::{test_metadata, valid_catalog};
use crate::catalog::types::{LocalSkill, RemoteSkill};

#[test]
fn validates_successful_catalog() {
    let catalog = valid_catalog();
    assert!(catalog.validate().is_ok());
}

#[test]
fn rejects_duplicate_local_names() {
    let mut catalog = valid_catalog();
    catalog.locals.push(LocalSkill {
        name: "fixture-skill".to_string(),
        relative_path: "skills/fixture-skill-2".to_string(),
        metadata: test_metadata("dup", &["dup"]),
    });
    assert!(catalog.validate().is_err());
}

#[test]
fn rejects_parent_traversal() {
    let mut catalog = valid_catalog();
    catalog.locals[0].relative_path = "../bad".to_string();
    assert!(catalog.validate().is_err());
}

#[test]
fn rejects_empty_and_current_dir_paths() {
    let mut catalog = valid_catalog();
    catalog.locals[0].relative_path = "".to_string();
    assert!(catalog.validate().is_err());

    let mut catalog = valid_catalog();
    catalog.locals[0].relative_path = ".".to_string();
    assert!(catalog.validate().is_err());
}

#[test]
fn rejects_duplicate_remote_name_in_source() {
    let mut catalog = valid_catalog();
    catalog.sources[0].skills.push(RemoteSkill {
        name: "remote-skill".to_string(),
        path: "skills/other".to_string(),
        metadata: test_metadata("other", &["other"]),
    });
    assert!(catalog.validate().is_err());
}
