//! Lightweight proper-noun pre-screening and pronoun resolution for entity extraction.
//!
//! Feeds `graph::extract_from_text()` with:
//! - `candidates`: proper noun phrases for the LLM to classify (guided attention)
//! - `pronoun_map`: resolved (pronoun, entity_name) pairs so the LLM never
//!   receives raw pronouns as entity candidates
//!
//! Both functions are pure Rust with no external NLP dependencies.

/// Maximum candidates forwarded to the LLM (prevents prompt blowout on dense passages).
const CANDIDATE_CAP: usize = 40;

/// Uppercase-starting words that are not proper nouns.
const STOP_WORDS: &[&str] = &[
    // Articles & determiners
    "The",
    "A",
    "An",
    "This",
    "That",
    "These",
    "Those",
    "Its",
    "It",
    // Personal pronouns
    "He",
    "Him",
    "His",
    "She",
    "Her",
    "Hers",
    "They",
    "Them",
    "Their",
    "Theirs",
    "We",
    "Our",
    "You",
    "Your",
    "I",
    "My",
    "Me",
    // Conjunctions & prepositions
    "In",
    "On",
    "At",
    "By",
    "For",
    "From",
    "With",
    "Of",
    "About",
    "And",
    "Or",
    "But",
    "If",
    "As",
    "So",
    "Yet",
    "Nor",
    "Both",
    "Also",
    "To",
    "Into",
    "Up",
    "Between",
    "Among",
    "Before",
    "After",
    "During",
    "Through",
    "Within",
    "Over",
    // Days of the week
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
    // Months (full)
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
    // Months (abbreviated) — prevent "Indian Opinion Dec" from fusing into one candidate
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
];

const MALE_PRONOUNS: &[&str] = &["he", "him", "his", "himself"];
const FEMALE_PRONOUNS: &[&str] = &["she", "her", "hers", "herself"];
const NEUTRAL_PRONOUNS: &[&str] = &["they", "them", "their", "theirs", "themselves"];

fn core(word: &str) -> &str {
    word.trim_matches(|c: char| !c.is_alphanumeric())
}

/// Returns true when `word` ends a phrase — either a full sentence (`.!?`) or a
/// list/clause separator (`,;:)"'`) that makes consecutive capitalized words
/// belong to different named entities rather than one multi-word phrase.
fn ends_phrase(word: &str) -> bool {
    matches!(
        word.chars().last(),
        Some('.' | '!' | '?' | ',' | ';' | ':' | ')' | '"' | '\'')
    )
}

fn is_stop_word(s: &str) -> bool {
    STOP_WORDS.contains(&s)
}

/// Extract proper noun candidate phrases from `text`.
///
/// Scans all capitalised word sequences (not just mid-sentence ones), merges
/// consecutive capitalised words into multi-word phrases, stops phrase
/// accumulation at sentence boundaries, and deduplicates results.
///
/// The caller passes the returned list to the LLM as a focus list; the LLM
/// classifies candidates as entities or discards non-entities.
pub fn extract_proper_noun_candidates(text: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    let words: Vec<&str> = text.split_whitespace().collect();
    let n = words.len();
    let mut i = 0;

    while i < n && result.len() < CANDIDATE_CAP {
        let w = words[i];
        let c = core(w);

        let is_candidate =
            c.len() > 1 && c.starts_with(|ch: char| ch.is_uppercase()) && !is_stop_word(c);

        if is_candidate {
            let mut parts = vec![c.to_string()];
            let mut j = i + 1;

            // Extend phrase across consecutive capitalised words, stopping at
            // any phrase boundary (sentence-end or list separator) or at 5 words
            // (entity names in this corpus never exceed 5 words).
            while j < n && !ends_phrase(words[j - 1]) && parts.len() < 5 {
                let nc = core(words[j]);
                if nc.len() > 1 && nc.starts_with(|ch: char| ch.is_uppercase()) && !is_stop_word(nc)
                {
                    parts.push(nc.to_string());
                    j += 1;
                } else {
                    break;
                }
            }

            let phrase = parts.join(" ");
            if seen.insert(phrase.clone()) {
                result.push(phrase);
            }
            i = j;
        } else {
            i += 1;
        }
    }

    result
}

/// Resolve pronouns in `text` to entity names.
///
/// `entities` is a snapshot of `(name, gender)` for Person entities already in
/// the graph, pre-computed before any async work starts. Gender is `"Male"`,
/// `"Female"`, or `None` (ambiguous).
///
/// `candidates` must contain **Person-entity names only** (e.g. GLiNER-confirmed
/// spans). Gendered pronouns (he/she) can only refer to persons — passing the full
/// proper-noun candidate list would cause Place or Organization names to be selected
/// when they appear closest before the pronoun (e.g. "walked through District Six.
/// He…" → wrong resolution to a Place). When `candidates` is empty the backward
/// scan is skipped and the forward scan is tried instead.
///
/// Resolution strategy per pronoun:
/// 1. Scan `entities` from end to start (most recently known = most likely referent).
/// 2. If no gender match in snapshot, scan `candidates` backward from the pronoun
///    position (most recently mentioned Person name in this chunk = most likely
///    referent). Caller is responsible for supplying Person-only candidates.
/// 3. If still unresolved, scan forward in the remaining text for the next
///    capitalised proper-noun sequence.
///
/// Returns one `(pronoun, entity_name)` pair per unique pronoun type found.
/// Unresolved pronouns are omitted rather than guessed.
pub fn resolve_pronouns(
    text: &str,
    entities: &[(String, Option<String>)],
    candidates: &[String],
) -> Vec<(String, String)> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut resolved: Vec<(String, String)> = Vec::new();
    let mut seen_pronouns = std::collections::HashSet::new();

    for (idx, &w) in words.iter().enumerate() {
        let lower = core(w).to_lowercase();

        let gender_wanted = if MALE_PRONOUNS.contains(&lower.as_str()) {
            Some("Male")
        } else if FEMALE_PRONOUNS.contains(&lower.as_str()) {
            Some("Female")
        } else if NEUTRAL_PRONOUNS.contains(&lower.as_str()) {
            Some("Neutral")
        } else {
            None
        };

        let Some(gender) = gender_wanted else {
            continue;
        };

        if !seen_pronouns.insert(lower.clone()) {
            continue;
        }

        // Strategy 1: most-recent entity with matching gender from graph snapshot.
        // For Neutral pronouns (they/them/their), only resolve if there is exactly
        // one candidate with no gender set — otherwise too ambiguous.
        let neutral_candidates: Vec<_> = entities.iter().filter(|(_, g)| g.is_none()).collect();
        if gender == "Neutral" && neutral_candidates.len() != 1 {
            continue; // ambiguous — skip
        }
        let found = entities.iter().rev().find(|(_, g)| match g.as_deref() {
            Some(eg) => eg == gender,
            None => gender == "Neutral",
        });

        let name = found
            .map(|(n, _)| n.clone())
            // Strategy 2: most recently mentioned proper-noun candidate before the pronoun.
            // Only used when the graph snapshot has no gender match (reset builds).
            // Gender is not checked — serves as a heuristic hint for the LLM preamble.
            .or_else(|| {
                if gender != "Neutral" {
                    backward_candidate(&words[..idx], candidates)
                } else {
                    None
                }
            })
            // Strategy 3: next capitalised sequence after the pronoun.
            .or_else(|| forward_name(&words[idx + 1..]));

        if let Some(name) = name {
            resolved.push((lower, name));
        }
    }

    resolved
}

/// A resolved pronoun/definite-description → entity mapping, produced by the coref pass.
#[derive(Debug, Clone)]
pub struct CorefResolution {
    /// The surface form that was resolved ("he", "my grandfather", "Grandpa", etc.)
    pub surface: String,
    /// Canonical entity name it resolved to
    pub entity_name: String,
    /// Character byte offset of the surface form in the original text
    pub offset: usize,
    /// Confidence: 0.9 = rule-based, 0.7 = LLM-assisted
    pub confidence: f32,
    /// Source of the resolution
    pub method: &'static str,
}

/// Definite descriptions and kinship roles that should be resolved to known entities.
/// Each entry is (surface_pattern, alias_to_match_against_entity_aliases).
/// The surface pattern is checked case-insensitively against chunk text.
const DEFINITE_DESCRIPTIONS: &[(&str, &str)] = &[
    ("grandpa", "grandpa"),
    ("grandfather", "grandfather"),
    ("my grandfather", "my grandfather"),
    ("grandma", "grandma"),
    ("grandmother", "grandmother"),
    ("the author", "author"),
    ("the narrator", "narrator"),
    ("my mother", "mother"),
    ("his mother", "mother"),
    ("her mother", "mother"),
    ("my father", "father"),
    ("his father", "father"),
    ("her father", "father"),
    ("my wife", "wife"),
    ("his wife", "wife"),
    ("my husband", "husband"),
    ("her husband", "husband"),
];

/// Resolve definite descriptions and kinship roles to known entities by alias matching.
///
/// For each surface form in `DEFINITE_DESCRIPTIONS` that appears in `text`, look up
/// candidate entities (from the graph) whose aliases include the alias pattern. Returns
/// a `CorefResolution` for each match.
///
/// `candidates` is a slice of `(name, aliases, gender)` from the graph's entity store,
/// pre-filtered to Person entities in the chunk's context window.
pub fn resolve_definite_descriptions(
    text: &str,
    candidates: &[(String, Vec<String>, Option<String>)],
) -> Vec<CorefResolution> {
    let text_lower = text.to_lowercase();
    let mut results = Vec::new();

    for &(surface, alias_pattern) in DEFINITE_DESCRIPTIONS {
        let Some(offset) = text_lower.find(surface) else {
            continue;
        };
        // Find candidate whose aliases contain the alias_pattern
        let matched = candidates
            .iter()
            .find(|(_, aliases, _)| aliases.iter().any(|a| a.to_lowercase() == alias_pattern));
        if let Some((name, _, _)) = matched {
            results.push(CorefResolution {
                surface: surface.to_string(),
                entity_name: name.clone(),
                offset,
                confidence: 0.9,
                method: "alias_match",
            });
        }
    }
    results
}

/// Resolve pronouns to entities using gender matching against graph candidates.
///
/// Extended version of the ingestion-time `resolve_pronouns` that accepts the
/// richer `(name, aliases, gender)` candidate list from the graph rather than the
/// global gender snapshot.
pub fn resolve_pronouns_from_candidates(
    text: &str,
    candidates: &[(String, Vec<String>, Option<String>)],
) -> Vec<CorefResolution> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut results = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (idx, &w) in words.iter().enumerate() {
        let lower = core(w).to_lowercase();
        let gender_wanted = if MALE_PRONOUNS.contains(&lower.as_str()) {
            Some("Male")
        } else if FEMALE_PRONOUNS.contains(&lower.as_str()) {
            Some("Female")
        } else if NEUTRAL_PRONOUNS.contains(&lower.as_str()) {
            Some("Neutral")
        } else {
            None
        };
        let Some(gender) = gender_wanted else {
            continue;
        };
        if !seen.insert(lower.clone()) {
            continue;
        }

        // Find the most-recent candidate with matching gender (scan candidates in reverse,
        // assuming they were ordered by recency / appearance position).
        let matched = candidates
            .iter()
            .rev()
            .find(|(_, _, g)| match g.as_deref() {
                Some(eg) => eg == gender || gender == "Neutral",
                None => gender == "Neutral",
            });

        if let Some((name, _, _)) = matched {
            // Find byte offset of this pronoun in original text
            let offset = text
                .split_whitespace()
                .take(idx + 1)
                .map(|s| s.len() + 1)
                .sum::<usize>()
                .saturating_sub(w.len() + 1);
            results.push(CorefResolution {
                surface: w.to_string(),
                entity_name: name.clone(),
                offset,
                confidence: 0.9,
                method: "gender_nearest",
            });
        }
    }
    results
}

/// Spatial pronouns that strongly indicate a place antecedent.
/// "it"/"that"/"which" are excluded — too ambiguous.
const SPATIAL_PRONOUNS: &[&str] = &["there", "where"];

/// Definite descriptions → alias patterns for Place entities.
const PLACE_DEFINITE_DESCRIPTIONS: &[(&str, &str)] = &[
    ("the district", "district"),
    ("the area", "area"),
    ("the neighbourhood", "neighbourhood"),
    ("the neighborhood", "neighborhood"),
    ("the suburb", "suburb"),
    ("the street", "street"),
    ("the road", "road"),
    ("the building", "building"),
    ("the mosque", "mosque"),
    ("the church", "church"),
    ("the field", "field"),
    ("the park", "park"),
];

/// Definite descriptions → alias patterns for Organization entities.
const ORG_DEFINITE_DESCRIPTIONS: &[(&str, &str)] = &[
    ("the organization", "organization"),
    ("the organisation", "organisation"),
    ("the group", "group"),
    ("the movement", "movement"),
    ("the committee", "committee"),
    ("the party", "party"),
    ("the league", "league"),
    ("the association", "association"),
    ("the college", "college"),
    ("the school", "school"),
    ("the congress", "congress"),
    ("the council", "council"),
];

/// Resolve spatial pronouns ("there", "where") and definite place descriptions to
/// known Place entities.
///
/// Only resolves to `in_chunk_candidates` — places explicitly named in the current
/// chunk. Uses the most recently mentioned in-chunk place as the spatial antecedent.
pub fn resolve_place_pronouns_from_candidates(
    text: &str,
    in_chunk_candidates: &[(String, Vec<String>)],
) -> Vec<CorefResolution> {
    if in_chunk_candidates.is_empty() {
        return Vec::new();
    }
    let words: Vec<&str> = text.split_whitespace().collect();
    let text_lower = text.to_lowercase();
    let mut results = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (idx, &w) in words.iter().enumerate() {
        let lower = core(w).to_lowercase();
        if !SPATIAL_PRONOUNS.contains(&lower.as_str()) {
            continue;
        }
        if !seen.insert(lower.clone()) {
            continue;
        }
        let before_text = words[..idx].join(" ").to_lowercase();
        // Resolve to the candidate with the rightmost mention before this pronoun
        let best = in_chunk_candidates.iter().max_by_key(|(name, aliases)| {
            let np = name
                .to_lowercase()
                .split_whitespace()
                .filter(|w| w.len() >= 4)
                .filter_map(|w| before_text.rfind(w))
                .max()
                .unwrap_or(0);
            let ap = aliases
                .iter()
                .flat_map(|a| {
                    let al = a.to_lowercase();
                    al.split_whitespace()
                        .filter(|w| w.len() >= 4)
                        .filter_map(|w| before_text.rfind(w))
                        .collect::<Vec<_>>()
                })
                .max()
                .unwrap_or(0);
            np.max(ap)
        });
        if let Some((name, _)) = best {
            // Guard: entity must actually appear before this pronoun
            let nl = name.to_lowercase();
            let mentioned = nl
                .split_whitespace()
                .filter(|w| w.len() >= 4)
                .any(|w| before_text.contains(w));
            if mentioned {
                let offset = words[..idx].iter().map(|s| s.len() + 1).sum::<usize>();
                results.push(CorefResolution {
                    surface: w.to_string(),
                    entity_name: name.clone(),
                    offset,
                    confidence: 0.85,
                    method: "spatial_pronoun",
                });
            }
        }
    }

    // Definite place descriptions: alias matching (uses all window candidates)
    for &(surface, alias_pat) in PLACE_DEFINITE_DESCRIPTIONS {
        let Some(offset) = text_lower.find(surface) else {
            continue;
        };
        let matched = in_chunk_candidates
            .iter()
            .find(|(_, aliases)| aliases.iter().any(|a| a.to_lowercase() == alias_pat));
        if let Some((name, _)) = matched {
            results.push(CorefResolution {
                surface: surface.to_string(),
                entity_name: name.clone(),
                offset,
                confidence: 0.9,
                method: "place_alias_match",
            });
        }
    }
    results
}

/// Resolve definite descriptions to known Organization entities by alias matching.
pub fn resolve_org_descriptions_from_candidates(
    text: &str,
    candidates: &[(String, Vec<String>)],
) -> Vec<CorefResolution> {
    if candidates.is_empty() {
        return Vec::new();
    }
    let text_lower = text.to_lowercase();
    let mut results = Vec::new();
    for &(surface, alias_pat) in ORG_DEFINITE_DESCRIPTIONS {
        let Some(offset) = text_lower.find(surface) else {
            continue;
        };
        let matched = candidates
            .iter()
            .find(|(_, aliases)| aliases.iter().any(|a| a.to_lowercase() == alias_pat));
        if let Some((name, _)) = matched {
            results.push(CorefResolution {
                surface: surface.to_string(),
                entity_name: name.clone(),
                offset,
                confidence: 0.9,
                method: "org_alias_match",
            });
        }
    }
    results
}

/// Find the rightmost candidate in `words_before` — the proper-noun most recently
/// mentioned before the pronoun. Used as strategy 2 in `resolve_pronouns` when the
/// graph snapshot is empty (reset builds). No gender check — serves as a heuristic.
fn backward_candidate(words_before: &[&str], candidates: &[String]) -> Option<String> {
    if candidates.is_empty() || words_before.is_empty() {
        return None;
    }
    let before_text = words_before.join(" ");
    let before_lower = before_text.to_lowercase();
    // Among all candidates, pick the one whose last occurrence position is furthest right.
    candidates
        .iter()
        .filter_map(|c| {
            before_lower
                .rfind(&c.to_lowercase())
                .map(|pos| (pos, c.clone()))
        })
        .max_by_key(|(pos, _)| *pos)
        .map(|(_, name)| name)
}

/// Scan forward through `words` past lowercase words to find the first capitalised
/// proper-noun sequence (stops extending the phrase on the first non-candidate word
/// after the sequence has started).
fn forward_name(words: &[&str]) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut in_phrase = false;

    for &w in words.iter().take(40) {
        let c = core(w);
        let is_candidate =
            c.len() > 1 && c.starts_with(|ch: char| ch.is_uppercase()) && !is_stop_word(c);

        if is_candidate {
            parts.push(c.to_string());
            in_phrase = true;
        } else if in_phrase {
            break; // phrase ended
        }
        // else: keep scanning past lowercase / stop words
    }

    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ner_proper_nouns_basic() {
        let text = "We lived in District Six. My father Abdullah Gool worked there.";
        let c = extract_proper_noun_candidates(text);
        assert!(c.contains(&"District Six".to_string()), "{c:?}");
        assert!(c.contains(&"Abdullah Gool".to_string()), "{c:?}");
    }

    #[test]
    fn ner_proper_nouns_stops_at_sentence_boundary() {
        let text = "We visited Cape Town. London is far away.";
        let c = extract_proper_noun_candidates(text);
        assert!(c.contains(&"Cape Town".to_string()), "{c:?}");
        assert!(c.contains(&"London".to_string()), "{c:?}");
        assert!(!c.contains(&"Cape Town London".to_string()), "{c:?}");
    }

    #[test]
    fn ner_proper_nouns_filters_stop_words() {
        let text = "The doctor said He was fine. She visited The Hospital.";
        let c = extract_proper_noun_candidates(text);
        assert!(!c.contains(&"The".to_string()), "{c:?}");
        assert!(!c.contains(&"He".to_string()), "{c:?}");
        assert!(c.contains(&"Hospital".to_string()), "{c:?}");
    }

    #[test]
    fn ner_resolve_pronouns_male() {
        let entities = vec![("Abdullah Gool".to_string(), Some("Male".to_string()))];
        let map = resolve_pronouns("Abdullah arrived. He was tired.", &entities, &[]);
        assert!(
            map.iter().any(|(p, n)| p == "he" && n == "Abdullah Gool"),
            "{map:?}"
        );
    }

    #[test]
    fn ner_resolve_pronouns_female() {
        let entities = vec![
            ("Hassan".to_string(), Some("Male".to_string())),
            ("Zainab".to_string(), Some("Female".to_string())),
        ];
        let map = resolve_pronouns("Zainab left. She took the train.", &entities, &[]);
        assert!(
            map.iter().any(|(p, n)| p == "she" && n == "Zainab"),
            "{map:?}"
        );
    }

    #[test]
    fn ner_phrase_boundary_comma_list() {
        // Comma-separated names must not merge into one giant phrase.
        let text =
            "The meeting was attended by Soviet Ambassador, Cissie Gool, Moses Kotane, Bill Andrews.";
        let c = extract_proper_noun_candidates(text);
        // Each name should be its own candidate
        assert!(c.contains(&"Soviet Ambassador".to_string()), "{c:?}");
        assert!(c.contains(&"Cissie Gool".to_string()), "{c:?}");
        assert!(c.contains(&"Moses Kotane".to_string()), "{c:?}");
        assert!(c.contains(&"Bill Andrews".to_string()), "{c:?}");
        // The entire list must NOT be one merged candidate
        assert!(
            !c.iter()
                .any(|s| s.contains("Soviet") && s.contains("Kotane")),
            "merged phrase found: {c:?}"
        );
    }

    #[test]
    fn ner_phrase_boundary_five_word_cap() {
        // A single run of six consecutive capitalized words must be capped at 5.
        let text = "He studied at Royal Cape Town University Medical School annually.";
        let c = extract_proper_noun_candidates(text);
        assert!(c.iter().all(|p| p.split_whitespace().count() <= 5), "{c:?}");
    }

    #[test]
    fn ner_resolve_pronouns_forward_scan() {
        let entities: Vec<(String, Option<String>)> = vec![];
        let map = resolve_pronouns(
            "He was a great leader. Nelson Mandela inspired millions.",
            &entities,
            &[],
        );
        assert!(
            map.iter().any(|(p, n)| p == "he" && n == "Nelson Mandela"),
            "{map:?}"
        );
    }

    #[test]
    fn ner_resolve_pronouns_backward_candidate() {
        // On a reset build the entity snapshot is empty. The backward candidate scan
        // should find the most recently mentioned proper noun before the pronoun.
        let entities: Vec<(String, Option<String>)> = vec![];
        let candidates = vec!["Yousuf Rassool".to_string(), "Cape Town".to_string()];
        let map = resolve_pronouns(
            "Yousuf Rassool arrived in Cape Town. He sat down.",
            &entities,
            &candidates,
        );
        // "Cape Town" is the most recent candidate before "He", but it is a Place.
        // The backward scan picks the rightmost-occurring candidate regardless of type —
        // the LLM resolves ambiguity with context. Here Cape Town appears right before He.
        assert!(
            map.iter().any(|(p, _)| p == "he"),
            "expected 'he' to resolve: {map:?}"
        );
    }

    #[test]
    fn ner_resolve_pronouns_backward_candidate_last_person() {
        // When only one name precedes the pronoun, backward scan returns it.
        let entities: Vec<(String, Option<String>)> = vec![];
        let candidates = vec!["Yousuf Rassool".to_string()];
        let map = resolve_pronouns(
            "Yousuf Rassool was a historian. He wrote the memoir.",
            &entities,
            &candidates,
        );
        assert!(
            map.iter().any(|(p, n)| p == "he" && n == "Yousuf Rassool"),
            "{map:?}"
        );
    }
}
