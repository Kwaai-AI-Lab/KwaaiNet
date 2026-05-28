use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SectionDef {
    /// Case-insensitive substring matched against paragraph text to detect this section heading.
    pub pattern: String,
    /// When true, chunks in this section are skipped during graph/entity extraction.
    #[serde(default)]
    pub skip: bool,
    /// Injected into the extraction prompt and dream evidence when this section is active.
    pub narrator_note: Option<String>,
    /// When true, lines in this section are parsed as index entries (Name, page...) and
    /// injected into the graph as entity name seeds. Still skips LLM extraction.
    #[serde(default)]
    pub index_seeds: bool,
}

/// A document schema, optionally derived from schema.org typing.
///
/// Well-known `metadata` keys for `schema_type = "Book"` (schema.org Book):
///   author, isbn, publisher, datePublished, inLanguage, numberOfPages,
///   genre, bookEdition, about (topic/subject), copyrightHolder, copyrightYear
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocSchema {
    /// The exact title of the source document — prevents treating it as a location entity.
    pub document_title: Option<String>,
    /// Name of the default narrator / first-person voice in the main body.
    pub default_narrator: Option<String>,
    /// Ordered list of section definitions; first pattern to match wins.
    #[serde(default)]
    pub sections: Vec<SectionDef>,
    /// Document-level metadata persisted into the KB and injected into every query.
    /// Common keys: author, subject, year, language, publisher, isbn, datePublished.
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
    /// schema.org @type (e.g. "Book", "Article"). Auto-detected when absent.
    pub schema_type: Option<String>,
}

impl DocSchema {
    /// One-line context string injected into LLM prompts.
    pub fn context_line(&self) -> Option<String> {
        if self.metadata.is_empty() && self.document_title.is_none() {
            return None;
        }
        let mut parts: Vec<String> = Vec::new();
        let title = self
            .document_title
            .as_deref()
            .or_else(|| self.metadata.get("title").map(|s| s.as_str()))
            .unwrap_or("(untitled document)");
        if let Some(author) = self.metadata.get("author") {
            parts.push(format!("\"{}\" by {}", title, author));
        } else {
            parts.push(format!("\"{}\"", title));
        }
        if let Some(year) = self
            .metadata
            .get("year")
            .or_else(|| self.metadata.get("datePublished"))
        {
            parts.push(format!("({})", year));
        }
        if let Some(isbn) = self.metadata.get("isbn") {
            parts.push(format!("ISBN: {}", isbn));
        }
        if let Some(pub_) = self.metadata.get("publisher") {
            parts.push(format!("Publisher: {}", pub_));
        }
        if let Some(subj) = self
            .metadata
            .get("subject")
            .or_else(|| self.metadata.get("about"))
        {
            parts.push(format!("Subject: {}", subj));
        }
        Some(parts.join(" "))
    }

    /// True if any section has `index_seeds: true`.
    pub fn has_index_seeds(&self) -> bool {
        self.sections.iter().any(|s| s.index_seeds)
    }
}

pub fn load_doc_schema(path: &Path) -> Result<DocSchema> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("reading doc schema: {}", path.display()))?;
    let schema: DocSchema =
        serde_yaml::from_str(&text).with_context(|| "parsing doc schema YAML")?;
    Ok(schema)
}

/// Returns the first `SectionDef` whose pattern is a case-insensitive substring of `heading`.
pub fn match_section<'a>(heading: &str, schema: &'a DocSchema) -> Option<&'a SectionDef> {
    let lower = heading.to_lowercase();
    schema
        .sections
        .iter()
        .find(|s| lower.contains(&s.pattern.to_lowercase()))
}

/// Heuristic auto-detection of document type and metadata from the first ~3000 chars
/// of a document (title page + copyright page). No LLM required. Returns a minimal
/// DocSchema with whatever fields could be reliably extracted.
pub fn auto_detect_schema(header_text: &str) -> DocSchema {
    let mut schema = DocSchema::default();

    if let Some(isbn) = find_isbn(header_text) {
        schema.schema_type = Some("Book".to_string());
        schema.metadata.insert("isbn".to_string(), isbn);
    }
    if let Some(pub_) = find_publisher(header_text) {
        schema.metadata.insert("publisher".to_string(), pub_);
    }
    if let Some(year) = find_copyright_year(header_text) {
        schema
            .metadata
            .entry("copyrightYear".to_string())
            .or_insert(year.clone());
        schema
            .metadata
            .entry("datePublished".to_string())
            .or_insert(year.clone());
        schema.metadata.entry("year".to_string()).or_insert(year);
    }
    if let Some(holder) = find_copyright_holder(header_text) {
        schema
            .metadata
            .insert("copyrightHolder".to_string(), holder);
    }
    if let Some(author) = find_by_author(header_text) {
        schema
            .metadata
            .entry("author".to_string())
            .or_insert(author);
    }

    schema
}

/// Parse an index section's raw text into (entity_name, type_hint) pairs.
///
/// Handles index formats like:
///   Rassool, Yousuf, 15, 23, 45–67       → ("Yousuf Rassool", Some("Person"))
///   Cape Town, 12, 34                      → ("Cape Town", None)
///   African National Congress (ANC), 72   → ("African National Congress", Some("Organization"))
pub fn parse_index_seeds(index_text: &str) -> Vec<(String, Option<String>)> {
    let mut seeds = Vec::new();
    for line in index_text.lines() {
        let line = line.trim();
        // Skip blank, pure-numeric, or very short lines (page headers etc.)
        if line.is_empty() || line.len() < 3 {
            continue;
        }
        if line.chars().next().is_none_or(|c| c.is_ascii_digit()) {
            continue;
        }

        let name_part = strip_page_refs(line);
        if name_part.len() < 2 {
            continue;
        }

        let (name, type_hint) = if let Some((surname, rest)) = split_surname_first(&name_part) {
            // "Surname, Firstname" → "Firstname Surname"
            (
                format!("{} {}", rest.trim(), surname.trim()),
                Some("Person".to_string()),
            )
        } else if is_likely_org(&name_part) {
            (name_part, Some("Organization".to_string()))
        } else if is_likely_person(&name_part) {
            (name_part, Some("Person".to_string()))
        } else {
            (name_part, None)
        };

        let name = name.trim().to_string();
        if !name.is_empty() {
            seeds.push((name, type_hint));
        }
    }
    seeds
}

// ── private helpers ───────────────────────────────────────────────────────────

fn find_isbn(text: &str) -> Option<String> {
    for line in text.lines() {
        if line.contains("ISBN") {
            let clean: String = line
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == 'X')
                .collect();
            if clean.len() == 13 || clean.len() == 10 {
                return Some(clean);
            }
        }
    }
    None
}

fn find_publisher(text: &str) -> Option<String> {
    for line in text.lines() {
        let lower = line.to_lowercase();
        if let Some(pos) = lower.find("published by") {
            let after = line[pos + 12..].trim();
            // Take up to the first comma or period, but not too long
            let name = after.split([',', '.']).next().unwrap_or("").trim();
            if !name.is_empty() && name.len() < 80 {
                return Some(name.to_string());
            }
        }
    }
    None
}

fn find_copyright_year(text: &str) -> Option<String> {
    for line in text.lines() {
        if line.to_lowercase().contains("copyright") || line.contains('©') {
            let mut buf = String::new();
            for ch in line.chars() {
                if ch.is_ascii_digit() {
                    buf.push(ch);
                    if buf.len() == 4 {
                        let y: u32 = buf.parse().unwrap_or(0);
                        if (1800..=2100).contains(&y) {
                            return Some(buf);
                        }
                        buf.clear();
                    }
                } else {
                    buf.clear();
                }
            }
        }
    }
    None
}

fn find_copyright_holder(text: &str) -> Option<String> {
    for line in text.lines() {
        if line.contains('©') {
            // "Copyright © Name YEAR" — take text between © and first digit year
            if let Some(sym_pos) = line.find('©') {
                let after = line[sym_pos + '©'.len_utf8()..].trim();
                // Drop leading "Copyright" word if present
                let after = after.strip_prefix("opyright").unwrap_or(after).trim();
                // Take until we hit a digit (year)
                let holder: String = after.chars().take_while(|c| !c.is_ascii_digit()).collect();
                let holder = holder.trim().trim_end_matches(',').to_string();
                if holder.len() > 2 {
                    return Some(holder);
                }
            }
        }
    }
    None
}

fn find_by_author(text: &str) -> Option<String> {
    for line in text.lines() {
        let t = line.trim();
        if let Some(rest) = t.strip_prefix("By ") {
            let name = rest.trim();
            let words: Vec<&str> = name.split_whitespace().collect();
            // 2–5 words, first word capitalised
            if (2..=5).contains(&words.len())
                && words[0].chars().next().is_some_and(|c| c.is_uppercase())
            {
                return Some(name.to_string());
            }
        }
    }
    None
}

/// Strip trailing page reference from an index entry line.
/// "Rassool, Yousuf, 15, 23" → "Rassool, Yousuf"
/// "Gandhi, x, 4, 7" → "Gandhi"  (handles roman numeral pages)
fn strip_page_refs(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut best_cut = chars.len();
    for (i, &c) in chars.iter().enumerate() {
        if c == ',' {
            let rest: String = chars[i + 1..].iter().collect();
            let rest_t = rest.trim();
            // Everything after comma is page-number-like (digits, roman numerals, separators)
            if !rest_t.is_empty()
                && rest_t.chars().all(|c| {
                    c.is_ascii_digit()
                        || matches!(c, 'i' | 'v' | 'x' | 'l' | 'c' | 'I' | 'V' | 'X' | 'L' | 'C')
                        || c == ','
                        || c == ' '
                        || c == '-'
                        || c == '\u{2013}'
                })
            {
                best_cut = i;
                break;
            }
        }
    }
    chars[..best_cut]
        .iter()
        .collect::<String>()
        .trim()
        .to_string()
}

/// Detect "Surname, Firstname [Middle]" pattern. Returns (surname, rest).
fn split_surname_first(name: &str) -> Option<(&str, &str)> {
    if let Some(pos) = name.find(',') {
        let surname = name[..pos].trim();
        let rest = name[pos + 1..].trim();
        let surname_words = surname.split_whitespace().count();
        if surname_words == 1
            && surname.chars().next().is_some_and(|c| c.is_uppercase())
            && rest.chars().next().is_some_and(|c| c.is_uppercase())
        {
            return Some((surname, rest));
        }
    }
    None
}

/// Heuristic: 2 or 3 capitalised words likely = person name.
/// Excludes entries with place-like suffixes (Street, Road, etc.) or org keywords.
fn is_likely_person(name: &str) -> bool {
    let words: Vec<&str> = name.split_whitespace().collect();
    if !(2..=4).contains(&words.len()) {
        return false;
    }
    // All words start with uppercase (title-case name)
    if !words
        .iter()
        .all(|w| w.chars().next().is_some_and(|c| c.is_uppercase()))
    {
        return false;
    }
    // Exclude place/address words
    const PLACE_WORDS: &[&str] = &[
        "street", "road", "avenue", "drive", "lane", "place", "square", "district", "city", "cape",
        "town", "province", "mountain", "river", "hall", "house", "building", "school", "museum",
        "station",
    ];
    let lower = name.to_lowercase();
    if PLACE_WORDS.iter().any(|w| lower.contains(w)) {
        return false;
    }
    true
}

fn is_likely_org(name: &str) -> bool {
    const ORG_KEYWORDS: &[&str] = &[
        "congress",
        "committee",
        "association",
        "league",
        "party",
        "union",
        "council",
        "institute",
        "museum",
        "school",
        "university",
        "college",
        "company",
        "corporation",
        "limited",
        "ltd",
        "inc",
        "foundation",
        "movement",
        "government",
        "ministry",
        "department",
        "church",
        "mosque",
    ];
    let lower = name.to_lowercase();
    ORG_KEYWORDS.iter().any(|k| lower.contains(k)) || (name.contains('(') && name.contains(')'))
}
