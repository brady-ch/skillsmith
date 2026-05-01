use super::types::{Catalog, ToonMetadata};

#[derive(Debug, Clone)]
pub struct SkillMatch<'a> {
    pub source_name: Option<String>,
    pub skill_name: String,
    pub skill_path: String,
    pub metadata: ToonMetadata,
    pub score: usize,
    pub reasons: Vec<String>,
    #[allow(dead_code)]
    marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> SkillMatch<'a> {
    pub fn new(
        source_name: Option<String>,
        skill_name: String,
        skill_path: String,
        metadata: ToonMetadata,
        score: usize,
        reasons: Vec<String>,
    ) -> Self {
        Self {
            source_name,
            skill_name,
            skill_path,
            metadata,
            score,
            reasons,
            marker: std::marker::PhantomData,
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        Option<String>,
        String,
        String,
        ToonMetadata,
        usize,
        Vec<String>,
    ) {
        (
            self.source_name,
            self.skill_name,
            self.skill_path,
            self.metadata,
            self.score,
            self.reasons,
        )
    }
}

pub(crate) fn sort_skill_matches(matches: &mut Vec<SkillMatch<'_>>) {
    matches.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| {
                left.metadata
                    .trigger
                    .skill_role
                    .sort_key()
                    .cmp(&right.metadata.trigger.skill_role.sort_key())
            })
            .then_with(|| {
                left.metadata
                    .trigger
                    .order_weight
                    .cmp(&right.metadata.trigger.order_weight)
            })
            .then_with(|| {
                token_hint_sort_key(left.metadata.token_hint)
                    .cmp(&token_hint_sort_key(right.metadata.token_hint))
            })
            .then_with(|| left.skill_name.cmp(&right.skill_name))
    });
}

/// Lower hints sort first; missing hint sorts last (unknown cost).
fn token_hint_sort_key(hint: Option<u32>) -> u64 {
    hint.map(|h| h as u64).unwrap_or(u64::MAX)
}

pub(crate) fn metadata_match_score(
    metadata: &ToonMetadata,
    tokens: &[String],
) -> (usize, Vec<String>) {
    let mut score = 0;
    let mut reasons = Vec::new();
    for token in tokens {
        if metadata
            .trigger
            .intent_tags
            .iter()
            .any(|tag| tag_matches_token(tag, token))
        {
            score += 3;
            reasons.push(format!("matched intent tag '{}'", token));
            continue;
        }
        if metadata
            .trigger
            .when_to_use
            .iter()
            .any(|entry| contains_token(entry, token))
        {
            score += 2;
            reasons.push(format!("matched trigger phrase '{}'", token));
            continue;
        }
        if contains_token(&metadata.trigger.summary, token)
            || contains_token(&metadata.objective.summary, token)
            || contains_token(&metadata.output.summary, token)
            || contains_token(&metadata.navigation.summary, token)
        {
            score += 1;
            reasons.push(format!("matched TOON summary '{}'", token));
        }
    }
    (score, reasons)
}

fn contains_token(haystack: &str, token: &str) -> bool {
    tokenize(haystack)
        .iter()
        .any(|segment| token_matches(segment, token))
}

fn tag_matches_token(tag: &str, token: &str) -> bool {
    tokenize(tag)
        .iter()
        .any(|segment| token_matches(segment, token))
}

fn token_matches(left: &str, right: &str) -> bool {
    normalize_token(left) == normalize_token(right)
}

fn normalize_token(token: &str) -> std::borrow::Cow<'_, str> {
    if let Some(base) = token.strip_suffix("ies") {
        return std::borrow::Cow::Owned(format!("{base}y"));
    }
    if token.len() > 3 && token.ends_with('s') {
        return std::borrow::Cow::Owned(token[..token.len() - 1].to_string());
    }
    std::borrow::Cow::Borrowed(token)
}

pub(crate) fn tokenize(input: &str) -> Vec<String> {
    const STOPWORDS: &[&str] = &[
        "a", "an", "and", "as", "at", "by", "for", "from", "in", "into", "is", "of", "on", "or",
        "the", "to", "with",
    ];
    input
        .split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '-')
        .filter(|segment| !segment.trim().is_empty())
        .map(|segment| segment.trim().to_lowercase())
        .filter(|segment| !STOPWORDS.contains(&segment.as_str()))
        .collect()
}

impl Catalog {
    pub fn matches_for_intent<'a>(&'a self, intent: &str) -> Vec<SkillMatch<'a>> {
        let tokens = tokenize(intent);
        let mut matches = Vec::new();

        for local in &self.locals {
            let (score, reasons) = metadata_match_score(&local.metadata, &tokens);
            if score > 0 {
                matches.push(SkillMatch::new(
                    None,
                    local.name.clone(),
                    local.relative_path.clone(),
                    local.metadata.clone(),
                    score,
                    reasons,
                ));
            }
        }

        for source in &self.sources {
            for skill in &source.skills {
                let (score, reasons) = metadata_match_score(&skill.metadata, &tokens);
                if score > 0 {
                    matches.push(SkillMatch::new(
                        Some(source.name.clone()),
                        skill.name.clone(),
                        skill.path.clone(),
                        skill.metadata.clone(),
                        score,
                        reasons,
                    ));
                }
            }
        }

        sort_skill_matches(&mut matches);
        matches
    }
}
