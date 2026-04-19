use crate::catalog::compat::CompatCatalog;

#[test]
fn migrates_old_schema_description() {
    let input = r#"
[[locals]]
name = "repo-scout"
description = "Inspect repos"
relative_path = "skills/repo-scout"
"#;
    let compat: CompatCatalog = toml::from_str(input).expect("parse old schema");
    let catalog = compat.into_catalog();
    assert_eq!(catalog.locals[0].metadata.trigger.summary, "Inspect repos");
    assert!(
        catalog.locals[0]
            .metadata
            .trigger
            .intent_tags
            .contains(&"repo-scout".to_string())
    );
}
