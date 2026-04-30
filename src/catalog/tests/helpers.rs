use crate::catalog::types::{
    Catalog, LocalSkill, RemoteSkill, RemoteSource, SkillRole, ToonMetadata, ToonNavigation,
    ToonText, ToonTrigger,
};

pub(super) fn test_metadata(summary: &str, tags: &[&str]) -> ToonMetadata {
    ToonMetadata {
        trigger: ToonTrigger {
            summary: summary.to_string(),
            intent_tags: tags.iter().map(|tag| tag.to_string()).collect(),
            when_to_use: vec![summary.to_string()],
            skill_role: SkillRole::Implementation,
            order_weight: 0,
        },
        objective: ToonText {
            summary: "objective".to_string(),
        },
        output: ToonText {
            summary: "output".to_string(),
        },
        navigation: ToonNavigation {
            summary: "navigation".to_string(),
            priority: 10,
        },
        deprecated: false,
        token_hint: None,
        tier: None,
    }
}

pub(super) fn valid_catalog() -> Catalog {
    Catalog {
        locals: vec![LocalSkill {
            name: "repo-scout".to_string(),
            relative_path: "skills/repo-scout".to_string(),
            metadata: test_metadata("Inspect repositories", &["repo", "analysis"]),
        }],
        sources: vec![RemoteSource {
            name: "source-a".to_string(),
            repo_url: "https://example.com/repo.git".to_string(),
            git_ref: "main".to_string(),
            skills: vec![RemoteSkill {
                name: "remote-skill".to_string(),
                path: "skills/remote-skill".to_string(),
                metadata: test_metadata("Review migrations", &["migration"]),
            }],
        }],
    }
}
