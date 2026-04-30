//! Remove caveman-style fluff from prose (see caveman `SKILL.md` Rules). Code fences stay unchanged.

use std::sync::LazyLock;

use regex::Regex;

/// Longest multi-word phrases first.
const CAVE_PHRASES: &[&str] = &[
    "i would be happy to help you with that",
    "i'd be happy to help you with that",
    "i would be happy to",
    "i'd be happy to",
    "of course",
    "happy to help",
    "happy to",
    "it seems like",
    "it seems",
    "i believe that",
    "i believe",
    "i think that",
    "i think",
    "kind of",
    "sort of",
];

const CAVE_FILLER: &[&str] = &["just", "really", "basically", "actually", "simply"];

const CAVE_PLEASANTRIES: &[&str] = &["sure", "certainly"];

const CAVE_HEDGE: &[&str] = &[
    "probably",
    "perhaps",
    "maybe",
    "might",
    "arguably",
    "generally",
    "typically",
    "usually",
    "likely",
];

static PHRASE_RES: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    CAVE_PHRASES
        .iter()
        .map(|p| {
            let e = regex::escape(p);
            Regex::new(&format!(r"(?i)\s*\b{e}\b\s*")).expect("phrase regex")
        })
        .collect()
});

static HEDGE_RE: LazyLock<Regex> = LazyLock::new(|| word_alt_regex(CAVE_HEDGE));
static FILLER_RE: LazyLock<Regex> = LazyLock::new(|| word_alt_regex(CAVE_FILLER));
static PLEASANTRY_RE: LazyLock<Regex> = LazyLock::new(|| word_alt_regex(CAVE_PLEASANTRIES));

static COLLAPSE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[ \t]{2,}").unwrap());
static STRIP_SPACE_BEFORE_NL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[ \t]+\n").unwrap());
static STRIP_SPACE_AFTER_NL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\n[ \t]+").unwrap());
static LINE_LEAD_PUNCT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*[!.,?]+(?:\s+|$)").unwrap());
static FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?ms)^```.*?^```").unwrap());
static ARTICLES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)\b(a|an|the)\b").unwrap());

fn word_alt_regex(words: &[&str]) -> Regex {
    let mut v: Vec<&str> = words.to_vec();
    v.sort_by_key(|w| std::cmp::Reverse(w.len()));
    let body = v
        .iter()
        .map(|w| regex::escape(w))
        .collect::<Vec<_>>()
        .join("|");
    Regex::new(&format!(r"(?i)\b(?:{body})\b")).expect("word alt regex")
}

fn collapse_ws(s: &str) -> String {
    let s = COLLAPSE_SPACES.replace_all(s, " ");
    let s = STRIP_SPACE_BEFORE_NL.replace_all(&s, "\n");
    STRIP_SPACE_AFTER_NL.replace_all(&s, "\n").into_owned()
}

fn strip_articles(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for m in ARTICLES.find_iter(text) {
        out.push_str(&text[last..m.start()]);
        let matched = m.as_str();
        let lowered = matched.to_lowercase();
        let after_first = text[m.end()..].chars().next();
        let keep = matches!(lowered.as_str(), "a" | "an") && after_first == Some('.');
        if keep {
            out.push_str(matched);
        }
        last = m.end();
    }
    out.push_str(&text[last..]);
    out
}

fn trim_horizontal_ws_end(s: &str) -> String {
    s.trim_matches(|c: char| [' ', '\t', '\r', '\u{000b}', '\u{000c}'].contains(&c))
        .to_string()
}

fn defluff_segment(text: &str, strip_articles_flag: bool) -> String {
    let mut s = text.to_string();
    for re in PHRASE_RES.iter() {
        s = re.replace_all(&s, " ").into_owned();
    }
    s = HEDGE_RE.replace_all(&s, "").into_owned();
    s = FILLER_RE.replace_all(&s, "").into_owned();
    s = PLEASANTRY_RE.replace_all(&s, "").into_owned();
    if strip_articles_flag {
        s = strip_articles(&s);
    }
    trim_horizontal_ws_end(&collapse_ws(&s))
}

fn strip_leading_line_punct(s: &str) -> String {
    LINE_LEAD_PUNCT.replace_all(s, "").into_owned()
}

/// Strip fluff; preserve ```-fenced regions. If `strip_articles` is false, keep a/an/the.
pub fn defluff(text: &str, strip_articles: bool) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pos = 0;
    for m in FENCE.find_iter(text) {
        out.push_str(&defluff_segment(&text[pos..m.start()], strip_articles));
        out.push_str(m.as_str());
        pos = m.end();
    }
    out.push_str(&defluff_segment(&text[pos..], strip_articles));
    strip_leading_line_punct(&out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_filler_and_phrases() {
        let s = "Sure! I would be happy to help. The issue is basically just that the token really uses <= wrong.";
        let got = defluff(s, true);
        assert!(!got.to_lowercase().contains("sure"));
        assert!(!got.contains("basically"));
        assert!(got.contains("<="));
    }

    #[test]
    fn preserves_code_fence() {
        let s = "Sure! Basically the bug is in foo.\n\n```js\nconst a = really just 1;\n```\n\nActually it seems fine.\n";
        let got = defluff(s, true);
        assert!(got.contains("really just"));
        assert!(!got.to_lowercase().contains("basically"));
    }

    #[test]
    fn keeps_a_before_extension_dot() {
        assert_eq!(defluff("Use a.json path.", true), "Use a.json path.");
    }
}
