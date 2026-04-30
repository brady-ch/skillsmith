use crate::catalog::tests::helpers::{test_metadata, valid_catalog};
use crate::catalog::types::{Catalog, LocalSkill, SkillRole};

#[test]
fn intent_matching_prefers_tags() {
    let catalog = valid_catalog();
    let matches = catalog.matches_for_intent("migration rollback");
    assert_eq!(matches[0].skill_name, "remote-skill");
}

#[test]
fn intent_matching_does_not_use_substring_false_positives() {
    let mut metadata = test_metadata("Capabilities and workflows", &["platform"]);
    metadata.trigger.summary = "Capabilities and workflows".to_string();
    metadata.objective.summary = "Capabilities and workflows".to_string();
    metadata.output.summary = "Capabilities and workflows".to_string();
    metadata.navigation.summary = "Capabilities and workflows".to_string();

    let catalog = Catalog {
        locals: vec![LocalSkill {
            name: "capability-skill".to_string(),
            relative_path: "skills/capability".to_string(),
            metadata,
        }],
        sources: Vec::new(),
    };

    assert!(catalog.matches_for_intent("api").is_empty());
}

#[test]
fn intent_matching_prefers_creational_skill_for_generic_builder_query() {
    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "creational-pattern-architect".to_string(),
                relative_path: "skills/creational-pattern-architect".to_string(),
                metadata: test_metadata(
                    "Choose creational design patterns for object construction",
                    &["creational", "builder", "factory", "construction"],
                ),
            },
            LocalSkill {
                name: "rust-patterns-architecture".to_string(),
                relative_path: "skills/rust-patterns-architecture".to_string(),
                metadata: test_metadata(
                    "Rust-specific architecture and API guidance",
                    &["rust", "architecture", "ownership", "api"],
                ),
            },
        ],
        sources: Vec::new(),
    };

    let matches = catalog.matches_for_intent("builder pattern");
    assert_eq!(matches[0].skill_name, "creational-pattern-architect");
}

#[test]
fn intent_matching_prefers_rust_skill_for_rust_specific_query() {
    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "creational-pattern-architect".to_string(),
                relative_path: "skills/creational-pattern-architect".to_string(),
                metadata: test_metadata(
                    "Choose creational design patterns for object construction",
                    &["creational", "builder", "factory", "construction"],
                ),
            },
            LocalSkill {
                name: "rust-patterns-architecture".to_string(),
                relative_path: "skills/rust-patterns-architecture".to_string(),
                metadata: test_metadata(
                    "Rust-specific architecture and API guidance",
                    &["rust", "architecture", "ownership", "api"],
                ),
            },
        ],
        sources: Vec::new(),
    };

    let matches = catalog.matches_for_intent("Rust ownership-safe API design");
    assert_eq!(matches[0].skill_name, "rust-patterns-architecture");
}

#[test]
fn intent_matching_prefers_umbrella_architecture_skill_for_broad_query() {
    let mut umbrella_meta = test_metadata(
        "Language-agnostic system architecture framing and decomposition",
        &[
            "architecture",
            "system",
            "design",
            "decomposition",
            "boundaries",
        ],
    );
    umbrella_meta.trigger.skill_role = SkillRole::Process;
    umbrella_meta.trigger.order_weight = -5;

    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "repo-scout".to_string(),
                relative_path: "skills/repo-scout".to_string(),
                metadata: test_metadata(
                    "Inspect repositories and architecture gaps",
                    &["repo", "analysis", "brief", "architecture"],
                ),
            },
            LocalSkill {
                name: "software-architecture-architect".to_string(),
                relative_path: "skills/software-architecture-architect".to_string(),
                metadata: umbrella_meta,
            },
        ],
        sources: Vec::new(),
    };

    let matches = catalog.matches_for_intent("architecture");
    assert_eq!(matches[0].skill_name, "software-architecture-architect");
}

#[test]
fn intent_matching_orders_process_before_implementation_on_score_tie() {
    let mut process_meta = test_metadata("Process guidance", &["api"]);
    process_meta.trigger.skill_role = SkillRole::Process;
    let mut impl_meta = test_metadata("Implementation guidance", &["api"]);
    impl_meta.trigger.skill_role = SkillRole::Implementation;

    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "z-impl-skill".to_string(),
                relative_path: "skills/z".to_string(),
                metadata: impl_meta,
            },
            LocalSkill {
                name: "a-process-skill".to_string(),
                relative_path: "skills/a".to_string(),
                metadata: process_meta,
            },
        ],
        sources: Vec::new(),
    };

    let matches = catalog.matches_for_intent("api");
    assert_eq!(matches[0].skill_name, "a-process-skill");
    assert_eq!(matches[1].skill_name, "z-impl-skill");
}

#[test]
fn intent_matching_orders_lower_token_hint_when_score_role_and_weight_tie() {
    let mut heavy = test_metadata("Same tags", &["api"]);
    heavy.token_hint = Some(900);
    let mut light = test_metadata("Same tags", &["api"]);
    light.token_hint = Some(100);

    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "heavy-skill".to_string(),
                relative_path: "skills/heavy".to_string(),
                metadata: heavy,
            },
            LocalSkill {
                name: "light-skill".to_string(),
                relative_path: "skills/light".to_string(),
                metadata: light,
            },
        ],
        sources: Vec::new(),
    };

    let matches = catalog.matches_for_intent("api");
    assert_eq!(matches[0].skill_name, "light-skill");
    assert_eq!(matches[1].skill_name, "heavy-skill");
}

#[test]
fn intent_matching_orders_known_token_hint_before_missing_when_other_keys_tie() {
    let mut known = test_metadata("Same tags", &["api"]);
    known.token_hint = Some(5_000);
    let mut unknown = test_metadata("Same tags", &["api"]);
    unknown.token_hint = None;

    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "unknown-hint".to_string(),
                relative_path: "skills/u".to_string(),
                metadata: unknown,
            },
            LocalSkill {
                name: "known-hint".to_string(),
                relative_path: "skills/k".to_string(),
                metadata: known,
            },
        ],
        sources: Vec::new(),
    };

    let matches = catalog.matches_for_intent("api");
    assert_eq!(matches[0].skill_name, "known-hint");
    assert_eq!(matches[1].skill_name, "unknown-hint");
}

#[test]
fn intent_matching_score_still_dominates_token_hint() {
    let mut weaker = test_metadata("Weaker match", &["api"]);
    weaker.token_hint = Some(1);
    let mut stronger = test_metadata("Stronger match", &["api", "versioning"]);
    stronger.token_hint = Some(10_000);

    let catalog = Catalog {
        locals: vec![
            LocalSkill {
                name: "cheap-weaker".to_string(),
                relative_path: "skills/cheap".to_string(),
                metadata: weaker,
            },
            LocalSkill {
                name: "expensive-stronger".to_string(),
                relative_path: "skills/expensive".to_string(),
                metadata: stronger,
            },
        ],
        sources: Vec::new(),
    };

    let matches = catalog.matches_for_intent("api versioning");
    assert_eq!(matches[0].skill_name, "expensive-stronger");
    assert_eq!(matches[1].skill_name, "cheap-weaker");
}
