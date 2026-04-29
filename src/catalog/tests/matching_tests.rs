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
