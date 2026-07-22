//! Per-chunk mention resolution — locates every noun/alias/pronoun/definite-description
//! span in a chunk's text and resolves it to a known entity, once, as a byproduct of
//! entity extraction. Relation extraction (`relation_extract.rs`) previously re-derived
//! this same literal/surname-drop/coref matching independently every time it ran,
//! against its own snapshot of `known_entities`/`person_candidates` — this module is
//! that logic relocated (not reinvented) so it runs once and gets persisted
//! (`GraphStore::write_chunk_mentions`/`get_chunk_mentions`), with every consumer
//! (relation extraction today; Phase 5's planned classifiers later) reading the same
//! resolution instead of each approximating their own.

use serde::{Deserialize, Serialize};

/// How a mention's span was resolved to its entity.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MentionKind {
    /// Exact literal match of the entity's canonical name or an alias.
    Literal,
    /// Truncated-name fallback (e.g. "Abdul Hamid Gool" -> "Abdul Hamid") — this
    /// corpus commonly drops surnames on repeat mentions.
    SurnameDropped,
    /// Third-person pronoun ("he"/"she"), gender-matched to a candidate.
    Pronoun,
    /// Definite description ("my wife", "the author").
    DefiniteDescription,
    /// Bare first-person pronoun ("I"/"my"/"me") resolved to the narrator.
    NarratorPronoun,
}

impl MentionKind {
    /// Whether this resolution required an inference step beyond an exact literal
    /// match — used to cap confidence lower, same reasoning `relation_extract.rs`
    /// already applied via its `via_coref` flag.
    pub fn is_approximate(self) -> bool {
        !matches!(self, MentionKind::Literal)
    }
}

/// One resolved mention within a sentence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentionSpan {
    pub entity_id: i64,
    pub entity_name: String,
    /// Byte offsets into the *sentence* string this mention belongs to
    /// (`SentenceMentions::sentence`), not the chunk as a whole — every consumer
    /// so far (trigger-phrase before/after matching) only ever needs sentence-local
    /// positions, so chunk-global offset tracking isn't worth the complexity.
    pub start: usize,
    pub end: usize,
    pub kind: MentionKind,
}

/// One sentence from a chunk, with every mention resolved within it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceMentions {
    pub sentence: String,
    pub mentions: Vec<MentionSpan>,
}

/// Resolve every mention in `chunk_text`, sentence by sentence — the same
/// sentence-splitting, literal/surname-drop matching, and coref resolution
/// `relation_extract.rs::classify_relation_candidates` used to do inline, extracted
/// so it can run once (during/after entity extraction) and be persisted instead of
/// re-derived on every relation-extraction pass.
///
/// `known_entities` is `(entity_id, canonical_name, aliases)`. `person_candidates` is
/// `(name, aliases, gender)`, as produced by `GraphStore::coref_candidates_for_chunk()`.
/// `narrator` additionally resolves bare first-person pronouns.
pub fn resolve_chunk_mentions(
    chunk_text: &str,
    known_entities: &[(i64, String, Vec<String>)],
    person_candidates: &[(String, Vec<String>, Option<String>)],
    narrator: Option<(i64, &str)>,
) -> Vec<SentenceMentions> {
    let name_by_id: std::collections::HashMap<i64, &str> = known_entities
        .iter()
        .map(|(id, name, _)| (*id, name.as_str()))
        .chain(narrator)
        .collect();
    let id_by_name_lower: std::collections::HashMap<String, i64> = known_entities
        .iter()
        .map(|(id, name, _)| (name.to_lowercase(), *id))
        .collect();

    let sentences: Vec<&str> = chunk_text
        .split(['.', '?', '!', ';'])
        .map(str::trim)
        .filter(|s| s.len() >= 12)
        .collect();

    let mut result = Vec::with_capacity(sentences.len());
    for sentence in sentences {
        let s_lower = sentence.to_lowercase();

        // (start, end, id, kind)
        let mut positions: Vec<(usize, usize, i64, MentionKind)> = Vec::new();
        for (id, name, aliases) in known_entities {
            let candidates =
                std::iter::once(name.as_str()).chain(aliases.iter().map(String::as_str));
            let mut matched = false;
            for tok in candidates {
                if tok.len() < 3 || tok.eq_ignore_ascii_case("I") {
                    continue;
                }
                if let Some(pos) = s_lower.find(&tok.to_lowercase()) {
                    positions.push((pos, pos + tok.len(), *id, MentionKind::Literal));
                    matched = true;
                    break;
                }
            }
            // Fall back to a surname-dropped variant of the canonical name when
            // nothing matched exactly — e.g. "Abdul Hamid Gool" -> "Abdul Hamid",
            // or "Cissie Gool" -> "Cissie". Collision risk (two known entities
            // sharing a truncated form) is caught downstream by the ambiguous-window
            // axiom, which demotes ties within the tie margin regardless of how the
            // position was matched.
            if !matched {
                let words: Vec<&str> = name.split_whitespace().collect();
                if words.len() >= 2 {
                    let truncated = words[..words.len() - 1].join(" ");
                    if truncated.len() >= 3 {
                        if let Some(pos) = s_lower.find(&truncated.to_lowercase()) {
                            positions.push((
                                pos,
                                pos + truncated.len(),
                                *id,
                                MentionKind::SurnameDropped,
                            ));
                        }
                    }
                }
            }
        }

        // Coref: definite descriptions ("my wife", "the author") + 3rd-person pronouns
        // ("he"/"she", gender-matched to the nearest candidate).
        for res in crate::ner::resolve_definite_descriptions(sentence, person_candidates) {
            if let Some(&id) = id_by_name_lower.get(&res.entity_name.to_lowercase()) {
                positions.push((
                    res.offset,
                    res.offset + res.surface.len(),
                    id,
                    MentionKind::DefiniteDescription,
                ));
            }
        }
        for res in crate::ner::resolve_pronouns_from_candidates(sentence, person_candidates) {
            if let Some(&id) = id_by_name_lower.get(&res.entity_name.to_lowercase()) {
                positions.push((
                    res.offset,
                    res.offset + res.surface.len(),
                    id,
                    MentionKind::Pronoun,
                ));
            }
        }

        // Coref: bare first-person pronouns ("I", "my", "me") -> the narrator.
        if let Some((narrator_id, _)) = narrator {
            positions.extend(
                find_narrator_pronoun_positions(sentence, narrator_id)
                    .into_iter()
                    .map(|(s, e, id)| (s, e, id, MentionKind::NarratorPronoun)),
            );
        }

        // Resolve overlapping spans by keeping the longest/most-specific match —
        // e.g. "my wife" (a definite-description resolution) fully contains "my"
        // (a bare first-person-pronoun match); without this, whichever happened to
        // be pushed last would win regardless of specificity.
        //
        // Critical distinction: an EXACT-same-span match from a *different* entity
        // (e.g. three distinct people all truncating to the identical "Cissie" text
        // via the surname-drop fallback) is genuine ambiguity, not a specificity
        // relationship — it must survive so the ambiguous-window axiom can see the
        // collision and demote it. Only a *properly contained* shorter span
        // (different start/end, not an exact tie) gets dropped in favor of the
        // longer, already-kept one.
        positions.sort_by_key(|(start, end, _, _)| std::cmp::Reverse(end - start));
        let mut deduped: Vec<(usize, usize, i64, MentionKind)> =
            Vec::with_capacity(positions.len());
        for pos @ (start, end, id, _) in positions {
            let overlaps = deduped.iter().any(|&(rs, re, rid, _)| {
                if rid == id {
                    // Same entity matched twice (e.g. literal + coref) — redundant.
                    start < re && rs < end
                } else if start == rs && end == re {
                    // Same span, different entity — keep both; this is ambiguity,
                    // not one match being more specific than the other.
                    false
                } else {
                    start < re && rs < end
                }
            });
            if !overlaps {
                deduped.push(pos);
            }
        }

        let mentions = deduped
            .into_iter()
            .map(|(start, end, id, kind)| MentionSpan {
                entity_id: id,
                entity_name: name_by_id
                    .get(&id)
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
                start,
                end,
                kind,
            })
            .collect();

        result.push(SentenceMentions {
            sentence: sentence.to_string(),
            mentions,
        });
    }

    result
}

/// Whole-word first-person pronouns, resolved to a fixed narrator entity regardless
/// of local context (unlike 3rd-person pronouns, which need gender/recency matching).
const FIRST_PERSON_PRONOUNS: &[&str] = &["i", "my", "me", "myself"];

/// Locate whole-word first-person pronoun occurrences in `text`, each resolved to
/// `narrator_id`. Byte offsets computed the same way as
/// `ner::resolve_pronouns_from_candidates` (cumulative preceding-word lengths).
fn find_narrator_pronoun_positions(text: &str, narrator_id: i64) -> Vec<(usize, usize, i64)> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut results = Vec::new();
    let mut offset = 0usize;
    for &w in &words {
        let core = w.trim_matches(|c: char| !c.is_alphanumeric());
        if FIRST_PERSON_PRONOUNS.contains(&core.to_lowercase().as_str()) {
            let start = offset + w.find(core).unwrap_or(0);
            results.push((start, start + core.len(), narrator_id));
        }
        offset += w.len() + 1; // +1 for the single space split_whitespace consumed
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entities(names: &[(i64, &str)]) -> Vec<(i64, String, Vec<String>)> {
        names
            .iter()
            .map(|&(id, name)| (id, name.to_string(), vec![]))
            .collect()
    }

    #[test]
    fn literal_match_resolves_both_endpoints() {
        let known = entities(&[(1, "Jane Doe"), (2, "John Smith")]);
        let sm = resolve_chunk_mentions(
            "Jane Doe married John Smith in the spring.",
            &known,
            &[],
            None,
        );
        assert_eq!(sm.len(), 1);
        let mentions = &sm[0].mentions;
        assert!(mentions
            .iter()
            .any(|m| m.entity_id == 1 && m.kind == MentionKind::Literal));
        assert!(mentions
            .iter()
            .any(|m| m.entity_id == 2 && m.kind == MentionKind::Literal));
    }

    #[test]
    fn surname_dropped_variant_matches() {
        let known = entities(&[(1, "Cissie Gool")]);
        let sm = resolve_chunk_mentions(
            "His marriage with Cissie was a decade old.",
            &known,
            &[],
            None,
        );
        let mentions = &sm[0].mentions;
        assert!(mentions
            .iter()
            .any(|m| m.entity_id == 1 && m.kind == MentionKind::SurnameDropped));
    }

    #[test]
    fn narrator_pronoun_resolves() {
        let known = entities(&[(1, "Jane Doe")]);
        let sm = resolve_chunk_mentions(
            "I married Jane Doe in the winter of that year.",
            &known,
            &[],
            Some((99, "John Smith")),
        );
        let mentions = &sm[0].mentions;
        assert!(mentions
            .iter()
            .any(|m| m.entity_id == 99 && m.kind == MentionKind::NarratorPronoun));
    }

    #[test]
    fn ambiguous_same_span_survives_dedup() {
        // Three distinct entities all truncate to the identical "Cissie" text —
        // genuine ambiguity must survive, not collapse to one.
        let known = entities(&[(1, "Cissie Gool"), (2, "Cissie Adams"), (3, "Cissie Baker")]);
        let sm = resolve_chunk_mentions("It was about Cissie and John Smith.", &known, &[], None);
        let cissie_mentions: Vec<_> = sm[0]
            .mentions
            .iter()
            .filter(|m| [1, 2, 3].contains(&m.entity_id))
            .collect();
        assert_eq!(cissie_mentions.len(), 3, "{:?}", sm[0].mentions);
    }
}
