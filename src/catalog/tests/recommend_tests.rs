use std::path::PathBuf;

use crate::catalog::types::Catalog;
use crate::catalog::{CatalogCache, recommend_for_intent};

#[test]
fn recommend_skips_remote_skills_without_local_reference_index() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let catalog_path = repo_root.join("catalog/catalog.toml");
    let catalog = Catalog::load_from_file(&catalog_path).expect("load catalog");
    let mut cache = CatalogCache::new(catalog);
    let intent = "refactor codebase to align with skillsmith catalog, CLI workflows, and skill authoring conventions";
    let res = recommend_for_intent(&mut cache, &repo_root, intent, 15, None, None, false)
        .expect("recommend should not fail when remote paths are absent");

    assert!(
        !res.recommendations
            .iter()
            .any(|r| r.skill_name == "example-skill"),
        "unmaterialized remote skills must not appear in recommendations"
    );
    assert!(
        !res.recommendations.is_empty(),
        "expected local skills to fill recommendations after skipping remote"
    );
}

#[test]
fn recommend_prefers_umbrella_architecture_skill_for_broad_query() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let catalog_path = repo_root.join("catalog/catalog.toml");
    let catalog = Catalog::load_from_file(&catalog_path).expect("load catalog");
    let mut cache = CatalogCache::new(catalog);
    let intent = "system architecture tradeoffs and module boundaries";
    let res = recommend_for_intent(&mut cache, &repo_root, intent, 5, None, None, false)
        .expect("recommend should succeed for architecture query");

    assert_eq!(
        res.recommendations
            .first()
            .map(|entry| entry.skill_name.as_str()),
        Some("software-architecture-architect")
    );
    assert_eq!(
        res.recommendations
            .first()
            .map(|entry| entry.suggested_reference_file.as_str()),
        Some("architecture-decision-framing-wenyan.md")
    );
}

#[test]
fn recommend_principles_intent_does_not_suggest_english_companion_for_architecture_skill() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let catalog_path = repo_root.join("catalog/catalog.toml");
    let catalog = Catalog::load_from_file(&catalog_path).expect("load catalog");
    let mut cache = CatalogCache::new(catalog);
    let intent = "engineering principles and wenyan rewrite workflow";
    let res = recommend_for_intent(
        &mut cache,
        &repo_root,
        intent,
        15,
        Some("software-architecture-architect"),
        None,
        false,
    )
    .expect("recommend should succeed for architecture skill");
    let first = res
        .recommendations
        .first()
        .expect("expected one recommendation");
    assert!(!first.suggested_reference_file.ends_with("-english.md"));
    assert!(
        first.suggested_reference_file.ends_with("-wenyan.md")
            || first.suggested_reference_file == "reference-router.md",
        "expected Wenyan slice or router, got {}",
        first.suggested_reference_file
    );
}

#[test]
fn recommend_skips_deprecated_locals_by_default() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let catalog_path = repo_root.join("catalog/catalog.toml");
    let mut catalog = Catalog::load_from_file(&catalog_path).expect("load catalog");
    catalog
        .locals
        .iter_mut()
        .find(|s| s.name == "using-skillsmith")
        .expect("using-skillsmith in catalog")
        .metadata
        .deprecated = true;
    let mut cache = CatalogCache::new(catalog);
    let intent = "skillsmith bootstrap install workflow agent rules validate";

    let res = recommend_for_intent(&mut cache, &repo_root, intent, 20, None, None, false)
        .expect("recommend default");
    assert!(
        !res.recommendations
            .iter()
            .any(|r| r.skill_name == "using-skillsmith"),
        "deprecated locals should be hidden from default recommend output"
    );

    let res_inc = recommend_for_intent(&mut cache, &repo_root, intent, 20, None, None, true)
        .expect("recommend include deprecated");
    assert!(
        res_inc
            .recommendations
            .iter()
            .any(|r| r.skill_name == "using-skillsmith"),
        "include_deprecated should restore deprecated entries"
    );
}
