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

// ── evidence selection ────────────────────────────────────────────────────────

/// The chosen evidence sentence and the window around it.
#[derive(Debug, Clone, PartialEq)]
pub struct EvidenceWindow {
    /// The window text: the trigger sentence plus `radius` sentences either side.
    pub text: String,
    /// Byte offsets of the triggering mention *within `text`*.
    pub trigger_start: usize,
    pub trigger_end: usize,
    /// The entity whose mention selected this sentence.
    pub entity_name: String,
}

/// Pick the best evidence window from a chunk's stored mention index.
///
/// The mention index records where each entity was actually located at ingest, so
/// this centres the quote on that position rather than re-deriving one by splitting
/// on punctuation and hoping a query word appears. The window is the trigger sentence
/// plus `radius` sentences either side, because the sentence the extractor matched
/// frequently does not itself contain the fact — it names the subject and the next
/// clause carries the claim.
///
/// A sentence scores on: mentions of an entity the query names (strongest), literal
/// matches over inferred ones (a pronoun is weaker evidence than a name), and mention
/// count. Returns `None` when the chunk has no stored mentions, which is the case for
/// KBs ingested before the index existed — callers must keep a fallback.
pub fn pick_evidence_window(
    sentences: &[SentenceMentions],
    query_terms: &[String],
    radius: usize,
) -> Option<EvidenceWindow> {
    let score_of = |sm: &SentenceMentions| -> (u32, usize) {
        let mut score = 0u32;
        // Query terms in the sentence itself. Most questions name places, objects or
        // events rather than graph entities ("the shop on Hanover Street"), so scoring
        // entity names alone picks whichever sentence is densest in people and loses
        // the topic entirely.
        let body = sm.sentence.to_lowercase();
        for t in query_terms {
            if body.contains(t.as_str()) {
                score += 8;
            }
        }
        for m in &sm.mentions {
            let name = m.entity_name.to_lowercase();
            if query_terms.iter().any(|t| name.contains(t.as_str())) {
                score += 10;
            }
            score += match m.kind {
                MentionKind::Literal => 3,
                MentionKind::SurnameDropped => 2,
                MentionKind::DefiniteDescription => 1,
                MentionKind::Pronoun | MentionKind::NarratorPronoun => 0,
            };
        }
        (score, sm.mentions.len())
    };

    // Every non-empty sentence is a candidate, not only those with a resolved entity:
    // a question about a place or an object ("the shop on Hanover Street") is often
    // answered by a sentence carrying no person mention at all, and requiring one
    // silently removed exactly those sentences from consideration.
    let (best_i, best) = sentences
        .iter()
        .enumerate()
        .filter(|(_, sm)| !sm.sentence.trim().is_empty())
        .max_by_key(|(_, sm)| score_of(sm))?;
    if score_of(best).0 == 0 {
        // Nothing to go on — let the caller fall back rather than quote arbitrarily.
        return None;
    }

    // Centre on the mention the query names; failing that, the one nearest a query
    // term in the sentence, so the window brackets the topic rather than the first
    // person who happens to appear.
    let body_lc = best.sentence.to_lowercase();
    let hit_at: Option<usize> = query_terms
        .iter()
        .filter_map(|t| body_lc.find(t.as_str()))
        .min();
    let best_mention = best.mentions.iter().max_by_key(|m| {
        let name = m.entity_name.to_lowercase();
        let named = query_terms.iter().any(|t| name.contains(t.as_str()));
        let near = match hit_at {
            Some(h) => usize::MAX - m.start.abs_diff(h),
            None => 0,
        };
        (
            named as u8,
            !m.kind.is_approximate() as u8,
            near,
            m.end - m.start,
        )
    });

    // Prefer the ingest-located mention. With none in this sentence, centre on the
    // query term that selected it, so the window still brackets the topic.
    let (trig_start, trig_len, trig_name) = match best_mention {
        Some(m) => (m.start, m.end - m.start, m.entity_name.clone()),
        None => {
            let (i, len) = query_terms
                .iter()
                .filter_map(|t| body_lc.find(t.as_str()).map(|i| (i, t.len())))
                .min_by_key(|(i, _)| *i)?;
            (i, len, best.sentence[i..i + len].to_string())
        }
    };

    let lo = best_i.saturating_sub(radius);
    let hi = (best_i + radius + 1).min(sentences.len());

    // Offset of the trigger once the neighbouring sentences are prepended.
    let mut prefix = 0usize;
    let mut text = String::new();
    for (i, sm) in sentences[lo..hi].iter().enumerate() {
        let s = sm.sentence.trim();
        if s.is_empty() {
            continue;
        }
        if !text.is_empty() {
            // The sentence splitter *consumes* the terminator, so joining with a bare
            // space runs two sentences together — and this window is rendered inside
            // quotation marks as if lifted verbatim from the source. Restore a full
            // stop unless the fragment already ends in punctuation of its own.
            if text.ends_with(|c: char| c.is_alphanumeric() || c == ')' || c == '"') {
                text.push('.');
            }
            text.push(' ');
        }
        if lo + i == best_i {
            // `start` is an offset into the untrimmed sentence.
            let lead = sm.sentence.len() - sm.sentence.trim_start().len();
            prefix = text.len() + trig_start.saturating_sub(lead);
        }
        text.push_str(s);
    }

    Some(EvidenceWindow {
        trigger_start: prefix.min(text.len()),
        trigger_end: (prefix + trig_len).min(text.len()),
        text,
        entity_name: trig_name,
    })
}

#[cfg(test)]
mod evidence_tests {
    use super::*;

    fn sm(sentence: &str, spans: &[(&str, usize, usize, MentionKind)]) -> SentenceMentions {
        SentenceMentions {
            sentence: sentence.to_string(),
            mentions: spans
                .iter()
                .map(|(n, s, e, k)| MentionSpan {
                    entity_id: 1,
                    entity_name: n.to_string(),
                    start: *s,
                    end: *e,
                    kind: *k,
                })
                .collect(),
        }
    }

    /// Regression: the window is rendered inside quotation marks, so joining the
    /// sentences must not run them together.
    ///
    /// The splitter consumes the terminator, so a bare space produced
    /// "Before Mike Allie He owned the shop" — presented as a verbatim quotation with
    /// every full stop missing. Existing tests missed it because they hand-build
    /// sentences that still carry their periods.
    #[test]
    fn window_restores_sentence_terminators() {
        let s = vec![
            sm("Before", &[]),
            sm("Mike Allie", &[("Mike Allie", 0, 10, MentionKind::Literal)]),
            sm("He owned the shop", &[]),
        ];
        let w = pick_evidence_window(&s, &["mike".into()], 1).unwrap();
        assert_eq!(w.text, "Before. Mike Allie. He owned the shop");
        // The trigger offset must survive the inserted punctuation.
        assert_eq!(&w.text[w.trigger_start..w.trigger_end], "Mike Allie");
    }

    /// A fragment that already ends in punctuation is not given a second terminator.
    #[test]
    fn window_does_not_double_punctuate() {
        let s = vec![
            sm("Was it him?", &[]),
            sm("Mike Allie", &[("Mike Allie", 0, 10, MentionKind::Literal)]),
        ];
        let w = pick_evidence_window(&s, &["mike".into()], 1).unwrap();
        assert_eq!(w.text, "Was it him? Mike Allie");
    }

    #[test]
    fn prefers_a_sentence_naming_a_query_entity() {
        let s = vec![
            sm(
                "Someone walked past.",
                &[("Unrelated", 0, 7, MentionKind::Pronoun)],
            ),
            sm(
                "Mike Allie ran the shop.",
                &[("Mike Allie", 0, 10, MentionKind::Literal)],
            ),
        ];
        let w = pick_evidence_window(&s, &["mike".into()], 0).unwrap();
        assert_eq!(w.entity_name, "Mike Allie");
        assert_eq!(&w.text[w.trigger_start..w.trigger_end], "Mike Allie");
    }

    /// The window is the point: the matched sentence often names the subject while
    /// the *next* sentence carries the claim.
    #[test]
    fn window_includes_neighbouring_sentences() {
        let s = vec![
            sm("Before.", &[]),
            sm(
                "Mike Allie.",
                &[("Mike Allie", 0, 10, MentionKind::Literal)],
            ),
            sm("He owned the shop.", &[]),
        ];
        let w = pick_evidence_window(&s, &["mike".into()], 1).unwrap();
        assert_eq!(w.text, "Before. Mike Allie. He owned the shop.");
        assert_eq!(&w.text[w.trigger_start..w.trigger_end], "Mike Allie");
    }

    /// The trigger offset must survive the neighbours being prepended.
    #[test]
    fn trigger_offset_tracks_the_prepended_context() {
        let s = vec![
            sm("A long preceding sentence here.", &[]),
            sm(
                "Then Mike Allie appeared.",
                &[("Mike Allie", 5, 15, MentionKind::Literal)],
            ),
        ];
        let w = pick_evidence_window(&s, &[], 1).unwrap();
        assert_eq!(&w.text[w.trigger_start..w.trigger_end], "Mike Allie");
    }

    #[test]
    fn literal_beats_pronoun_at_equal_relevance() {
        let s = vec![
            sm(
                "He went home.",
                &[("Yousuf", 0, 2, MentionKind::NarratorPronoun)],
            ),
            sm(
                "Yousuf went home.",
                &[("Yousuf", 0, 6, MentionKind::Literal)],
            ),
        ];
        let w = pick_evidence_window(&s, &[], 0).unwrap();
        assert_eq!(w.text, "Yousuf went home.");
    }

    /// Regression: a question naming no graph entity ("the shop on Hanover Street")
    /// used to select whichever sentence was densest in people, losing the topic.
    #[test]
    fn query_terms_in_the_sentence_outweigh_mention_density() {
        let s = vec![
            sm(
                "Naz and I were always very close.",
                &[
                    ("Naz", 0, 3, MentionKind::Literal),
                    ("Yousuf", 8, 9, MentionKind::NarratorPronoun),
                ],
            ),
            sm(
                "The shop on Hanover Street was owned by Prop Diamond.",
                &[("Prop Diamond", 40, 52, MentionKind::Literal)],
            ),
        ];
        let w = pick_evidence_window(&s, &["hanover".into(), "shop".into()], 0).unwrap();
        assert!(
            w.text.contains("Hanover"),
            "picked the wrong sentence: {}",
            w.text
        );
    }

    /// A sentence with no resolved entity is still the right evidence when it is the
    /// one that answers the question.
    #[test]
    fn a_sentence_without_mentions_can_win_on_query_terms() {
        let s = vec![
            sm(
                "Naz and I were close.",
                &[("Naz", 0, 3, MentionKind::Literal)],
            ),
            sm("The shop on Hanover Street sold spices.", &[]),
        ];
        let w = pick_evidence_window(&s, &["hanover".into(), "shop".into()], 0).unwrap();
        assert!(w.text.contains("Hanover"), "got: {}", w.text);
        assert_eq!(
            w.text[w.trigger_start..w.trigger_end].to_lowercase(),
            "shop"
        );
    }

    #[test]
    fn nothing_relevant_yields_none_so_the_caller_can_fall_back() {
        assert!(pick_evidence_window(&[], &[], 1).is_none());
        // No mentions and no query hits: refuse rather than quote arbitrarily.
        assert!(pick_evidence_window(&[sm("Nothing here.", &[])], &["zebra".into()], 1).is_none());
    }
}
