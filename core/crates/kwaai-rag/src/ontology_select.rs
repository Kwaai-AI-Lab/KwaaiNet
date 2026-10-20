//! Choosing an ontology per document.
//!
//! A *library* is a set of ratified ontologies, kept as YAML files: data, never
//! compiled in, so domain knowledge stays out of Rust. For each document the
//! selector scores every candidate by *coverage*, the share of the document's
//! chunks that carry the candidate's vocabulary. This is the measure the
//! ontologies were derived with (each type's `evidence`), and the NCBO
//! recommender's first criterion. It is deterministic, needs no LLM call, and
//! every score is kept, so a choice can be audited and overridden.
//!
//! See `projects/kwaai-knowledge/plans/OntologySelection-plan.md` §2.2–2.3.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::ontology::Ontology;

/// Words too common to count as a candidate's vocabulary.
const STOP: &[&str] = &[
    "the", "and", "for", "with", "from", "that", "this", "into", "onto", "over", "under", "about",
    "named", "name", "other", "type", "types", "thing", "things", "any", "all", "one", "two",
];

/// A chunk counts as covered when the weighted mass of the distinct terms it
/// carries reaches this: roughly two distinctive phrases, or four single words.
const MIN_MASS_PER_CHUNK: f32 = 2.0;

/// A term's specificity. A single word ("white", "community", "election") is
/// half as telling as a phrase or a name: a large ontology holds many common
/// words that are genuine domain vocabulary but hit in any English text.
fn specificity(term: &str) -> f32 {
    if term.split_whitespace().count() >= 2 {
        1.0
    } else {
        0.5
    }
}

/// One ontology in the library, with its weighted vocabulary.
#[derive(Debug, Clone)]
pub struct Candidate {
    pub name: String,
    /// Where it came from, for the report.
    pub source: String,
    /// `provenance.derivation` in the file ("derived", "unverified", "module"...).
    pub derivation: Option<String>,
    pub ontology: Ontology,
    /// (term, weight): 2 × IDF across candidates × specificity, so a phrase
    /// unique to one candidate weighs 2.0 at any library size.
    terms: Vec<(String, f32)>,
}

impl Candidate {
    pub fn term_count(&self) -> usize {
        self.terms.len()
    }
}

/// The set of ontologies a selector chooses among.
#[derive(Debug, Clone, Default)]
pub struct Library {
    pub candidates: Vec<Candidate>,
}

fn split_camel(name: &str) -> String {
    let mut out = String::new();
    for (i, ch) in name.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            out.push(' ');
        }
        out.push(ch);
    }
    out.to_lowercase()
}

/// Where a term came from; each kind has its own distinctiveness bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TermKind {
    /// A type's name ("Court", "TippingElement"). Generic ones ("Person",
    /// "Organization") are shared by many ontologies and say nothing.
    TypeName,
    /// A relation's trigger phrase. One-word triggers are common verbs
    /// ("said", "developed"), so only phrases count.
    Trigger,
    /// A type's examples and closed-vocabulary markers: the domain's own
    /// instances, the strongest evidence.
    Instance,
}

impl TermKind {
    /// The most candidates that may share a term of this kind before it stops
    /// telling them apart. Measured on the shipped library of 19 (18 since D6.yaml left the public tree): generic type
    /// names and one-word verbs put the largest ontology (D6's) second on
    /// almost every document, and first on a segregation case and on NASA's
    /// Golden Record page.
    fn max_df(self) -> usize {
        match self {
            TermKind::TypeName | TermKind::Trigger => 2,
            TermKind::Instance => 3,
        }
    }
}

/// The raw vocabulary of an ontology: what a document written in its domain
/// would contain.
fn vocabulary(o: &Ontology) -> HashMap<String, TermKind> {
    let mut v: HashMap<String, TermKind> = HashMap::new();
    let mut add = |s: &str, kind: TermKind| {
        let t = s
            .trim()
            .trim_matches(|c: char| c == '"' || c == '\'')
            .to_lowercase();
        if t.chars().count() >= 3
            && !STOP.contains(&t.as_str())
            && t.chars().any(|c| c.is_alphabetic())
        {
            // An instance outranks the same string as a type name or trigger.
            let e = v.entry(t).or_insert(kind);
            if kind == TermKind::Instance {
                *e = kind;
            }
        }
    };
    for e in &o.entity_types {
        add(&split_camel(&e.name), TermKind::TypeName);
        for x in e
            .examples
            .iter()
            .chain(&e.markers.exact)
            .chain(&e.markers.any)
        {
            add(x, TermKind::Instance);
        }
    }
    for r in &o.relation_types {
        for t in r
            .triggers
            .iter()
            .filter(|t| t.split_whitespace().count() >= 2)
        {
            add(t, TermKind::Trigger);
        }
    }
    v
}

impl Library {
    /// Build a library from named ontologies, weighting each term by how few
    /// candidates share it. A term shared by more candidates than its kind
    /// allows (`TermKind::max_df`) says nothing about which one fits, and is
    /// dropped.
    pub fn new(members: Vec<(String, Option<String>, Ontology)>) -> Self {
        let vocabs: Vec<HashMap<String, TermKind>> =
            members.iter().map(|(_, _, o)| vocabulary(o)).collect();
        let n = vocabs.len().max(1) as f32;
        let mut df: HashMap<&str, usize> = HashMap::new();
        for v in &vocabs {
            for t in v.keys() {
                *df.entry(t.as_str()).or_default() += 1;
            }
        }
        let candidates = members
            .iter()
            .zip(&vocabs)
            .map(|((source, derivation, o), v)| {
                let mut terms: Vec<(String, f32)> = v
                    .iter()
                    .filter(|(t, kind)| df[t.as_str()] <= kind.max_df())
                    .map(|(t, _)| {
                        // 1.0 for a term only one candidate has, whatever the library size;
                        // lower as more share it. (ln(n/df)/ln(n) gave every term 0.1 in a
                        // one-ontology library, so nothing could ever be chosen.)
                        let idf = (1.0 + (n / df[t.as_str()] as f32).ln()) / (1.0 + n.ln());
                        let idf = if df[t.as_str()] == 1 { 1.0 } else { idf };
                        (t.clone(), idf * specificity(t) * 2.0)
                    })
                    .collect();
                terms.sort_by(|a, b| a.0.cmp(&b.0));
                Candidate {
                    name: o.ontology.name.clone(),
                    source: source.clone(),
                    derivation: derivation.clone(),
                    ontology: o.clone(),
                    terms,
                }
            })
            .collect();
        Library { candidates }
    }

    /// Load every `*.yaml` in `dir` that parses as an ontology. Files that fail
    /// to parse are returned as errors beside the library, never skipped
    /// silently.
    pub fn load_dir(dir: &Path) -> anyhow::Result<(Self, Vec<String>)> {
        let mut members = Vec::new();
        let mut errors = Vec::new();
        let mut paths: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("yaml"))
            .collect();
        paths.sort();
        for p in paths {
            let src = std::fs::read_to_string(&p)?;
            let file = p
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            match Ontology::from_yaml(&src) {
                Ok(o) if !o.is_empty() => {
                    let derivation = serde_yaml::from_str::<serde_yaml::Value>(&src)
                        .ok()
                        .and_then(|v| {
                            v.get("provenance")?
                                .get("derivation")?
                                .as_str()
                                .map(String::from)
                        });
                    members.push((file, derivation, o));
                }
                Ok(_) => errors.push(format!("{file}: empty ontology")),
                Err(e) => errors.push(format!("{file}: {e}")),
            }
        }
        Ok((Library::new(members), errors))
    }

    pub fn get(&self, name: &str) -> Option<&Candidate> {
        self.candidates.iter().find(|c| c.name == name)
    }
}

fn contains_term(hay: &str, term: &str) -> bool {
    let mut from = 0;
    while let Some(pos) = hay[from..].find(term) {
        let s = from + pos;
        let e = s + term.len();
        let before = hay[..s]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric());
        let after = hay[e..].chars().next().is_none_or(|c| !c.is_alphanumeric());
        if before && after {
            return true;
        }
        from = e;
    }
    false
}

/// A candidate's score on one document.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Coverage {
    pub ontology: String,
    /// Share of sampled chunks whose term mass reaches `MIN_MASS_PER_CHUNK`.
    pub share: f32,
    /// Mean term mass per chunk, divided by √(vocabulary size) so a large
    /// vocabulary does not win ties by size alone. The tie-breaker.
    pub mass: f32,
    /// The candidate's most frequent terms in this document, for the report.
    pub top_terms: Vec<String>,
}

pub fn coverage(c: &Candidate, chunks: &[String]) -> Coverage {
    let mut covered = 0usize;
    let mut mass = 0f32;
    let mut freq: HashMap<&str, usize> = HashMap::new();
    for ch in chunks {
        let lower = ch.to_lowercase();
        let mut chunk_mass = 0f32;
        for (t, w) in &c.terms {
            if contains_term(&lower, t) {
                chunk_mass += w;
                *freq.entry(t.as_str()).or_default() += 1;
            }
        }
        mass += chunk_mass;
        if chunk_mass >= MIN_MASS_PER_CHUNK {
            covered += 1;
        }
    }
    let mass = mass / (c.terms.len().max(1) as f32).sqrt();
    let n = chunks.len().max(1) as f32;
    let mut top: Vec<(&str, usize)> = freq.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    Coverage {
        ontology: c.name.clone(),
        share: covered as f32 / n,
        mass: mass / n,
        top_terms: top
            .into_iter()
            .take(6)
            .map(|(t, _)| t.to_string())
            .collect(),
    }
}

/// The recorded choice for one document. Every field defaults, so a record
/// written by an older binary still parses when fields are added.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct DocChoice {
    /// The chosen ontology's name, or `None` when no candidate cleared the bar.
    pub ontology: Option<String>,
    /// How the choice was made: `"coverage"` (the selector). The only method so
    /// far; a person's override would be recorded as another.
    pub method: String,
    /// schema.org-style document type, when an identifier in the text shows it.
    pub doc_type: Option<String>,
    pub doc_type_evidence: Option<String>,
    pub chunks_sampled: usize,
    /// Every candidate's score, best first.
    pub scores: Vec<Coverage>,
}

/// Settings for `select`.
#[derive(Debug, Clone)]
pub struct SelectConfig {
    /// Chunks sampled per document (spread evenly through it).
    pub sample: usize,
    /// A candidate must cover at least this share of chunks to be chosen.
    pub min_share: f32,
    /// ...and beat the runner-up by at least this much, or the choice is `None`.
    pub min_margin: f32,
    /// The one corpus-scoped ontology eligible here: the KB's own, when it has
    /// one loaded. Other corpus-scoped ontologies are never chosen.
    pub own_corpus: Option<String>,
}

impl Default for SelectConfig {
    fn default() -> Self {
        SelectConfig {
            sample: 60,
            min_share: 0.15,
            min_margin: 0.03,
            own_corpus: None,
        }
    }
}

/// Evenly spaced sample of up to `n` items, deterministic.
pub fn spread<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    if items.len() <= n {
        return items.to_vec();
    }
    (0..n).map(|i| items[i * items.len() / n].clone()).collect()
}

/// A document's text as windows of about `size` characters, broken at
/// whitespace: the chunks to score before the document is chunked for real.
pub fn windows(text: &str, size: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if cur.len() + word.len() + 1 > size && !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur.push_str(word);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Choose an ontology for one document from its name and chunks (in reading
/// order).
pub fn select(lib: &Library, doc_name: &str, chunks: &[String], cfg: &SelectConfig) -> DocChoice {
    let head: String = std::iter::once(doc_name.to_string())
        .chain(chunks.iter().take(5).cloned())
        .collect::<Vec<_>>()
        .join("\n");
    let (doc_type, doc_type_evidence) = match detect_doc_type(&head) {
        Some((t, e)) => (Some(t.to_string()), Some(e)),
        None => (None, None),
    };
    // A detected document type narrows the field to the ontologies declaring
    // it, when any does. Plessy v. Ferguson carries a memoir's racial
    // vocabulary ("white", "colored") and McConnell v. FEC its election
    // vocabulary, but both are court opinions.
    // Leave-one-out showed why corpus scope exists: without Legal's own
    // ontology, 8 of 13 court opinions went to a memoir's ontology.
    let in_scope = |c: &&Candidate| {
        c.ontology.scope.as_deref() != Some("corpus")
            || cfg.own_corpus.as_deref() == Some(c.name.as_str())
    };
    let pool: Vec<&Candidate> = lib.candidates.iter().filter(in_scope).collect();
    let declares = |c: &&&Candidate| {
        doc_type
            .as_ref()
            .is_some_and(|t| c.ontology.document_types.contains(t))
    };
    let eligible: Vec<&Candidate> = if pool.iter().any(|c| declares(&c)) {
        pool.iter().filter(declares).copied().collect()
    } else {
        pool
    };
    let sample = spread(chunks, cfg.sample);
    let mut scores: Vec<Coverage> = eligible.iter().map(|c| coverage(c, &sample)).collect();
    scores.sort_by(|a, b| {
        b.share
            .partial_cmp(&a.share)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(
                b.mass
                    .partial_cmp(&a.mass)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
    });
    let chosen = match scores.as_slice() {
        [best, rest @ ..] if best.share >= cfg.min_share => {
            let runner = rest.first().map_or(0.0, |r| r.share);
            let ahead = best.share - runner >= cfg.min_margin
                || rest
                    .first()
                    .is_some_and(|r| best.mass > 0.0 && best.mass >= r.mass * 1.5);
            ahead.then(|| best.ontology.clone())
        }
        _ => None,
    };
    DocChoice {
        ontology: chosen,
        method: "coverage".into(),
        doc_type,
        doc_type_evidence,
        chunks_sampled: sample.len(),
        scores,
    }
}

/// A document type from an identifier in the document's opening text, with the
/// matched text as evidence. Identifiers only, never guesses: `None` when none
/// is found. Plain string scans; each rule is one identifier form.
pub fn detect_doc_type(head: &str) -> Option<(&'static str, String)> {
    // ASCII lowercasing keeps every byte offset valid in `head`. Full Unicode
    // lowercasing changes lengths (İ, ẞ, the Kelvin sign), so offsets found in
    // the lowercased copy sliced `head` mid-character.
    let lower = head.to_ascii_lowercase();
    // Evidence is taken by characters from an offset that is always a char
    // boundary (the start of an ASCII match), never by a byte range: a byte
    // range ending inside "—" panicked.
    let snippet = |at: usize, chars: usize| -> String {
        head.get(at..).unwrap_or("").chars().take(chars).collect()
    };
    let words: Vec<&str> = head.split_whitespace().collect();
    let digits = |w: &str| {
        let w = w.trim_matches(|c: char| !c.is_alphanumeric());
        !w.is_empty() && w.len() <= 4 && w.chars().all(|c| c.is_ascii_digit())
    };
    // A US reporter citation: "410 U.S. 113", "60 U. S. 393" or "163 US 537".
    for i in 1..words.len() {
        let (vol, rest) = (words[i - 1], &words[i..]);
        let (is_us, next) = match rest {
            [us, n, ..] if matches!(us.trim_end_matches(','), "U.S." | "US") => (true, *n),
            [u, s, n, ..] if *u == "U." && s.trim_end_matches(',') == "S." => (true, *n),
            _ => (false, ""),
        };
        if is_us && digits(vol) && digits(next) {
            let span = words[i - 1..(i + 3).min(words.len())].join(" ");
            return Some(("CourtOpinion", span));
        }
    }
    let find = |needles: &[&str]| {
        needles
            .iter()
            .find_map(|n| lower.find(n).map(|at| snippet(at, 60)))
    };
    if let Some(e) = find(&["be it enacted", "an act to "]) {
        return Some(("Legislation", e));
    }
    if let Some(at) = lower.find("public law ") {
        if lower[at + 11..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
        {
            return Some(("Legislation", snippet(at, 20)));
        }
    }
    // A DOI: "10." + 4-9 digits + "/".
    let mut from = 0;
    while let Some(pos) = lower[from..].find("10.") {
        let at = from + pos;
        let tail = &lower[at + 3..];
        let n = tail.chars().take_while(|c| c.is_ascii_digit()).count();
        if (4..=9).contains(&n) && tail[n..].starts_with('/') {
            let len = lower[at..]
                .split(char::is_whitespace)
                .next()
                .map_or(0, |t| t.chars().count());
            return Some(("ScholarlyArticle", snippet(at, len)));
        }
        from = at + 3;
    }
    if let Some(e) = find(&["isbn"]) {
        return Some(("Book", e));
    }
    if let Some(e) = find(&["final report", "report of the", "review board"]) {
        return Some(("Report", e));
    }
    None
}

/// Per-document ontology choices for a KB, stored in the graph's metadata
/// (`kb_doc_ontologies`). The chosen ontologies are stored once each by name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DocOntologies {
    pub ontologies: BTreeMap<String, Ontology>,
    pub docs: BTreeMap<String, DocChoice>,
    /// The ontology the KB had before any selection, loaded by hand (`graph
    /// schema load`). Documents with no choice use it, and the union includes
    /// it. Storing a selection used to overwrite it with the union: a D6 KB
    /// that gained one court opinion lost D6's ontology for every other chunk.
    pub base: Option<Ontology>,
}

impl DocOntologies {
    /// The ontology chosen for a document, if one was.
    pub fn for_doc(&self, doc_name: &str) -> Option<&Ontology> {
        let name = self.docs.get(doc_name)?.ontology.as_ref()?;
        self.ontologies.get(name)
    }

    /// The ontology a document's chunks are extracted with: its own choice,
    /// else the KB's hand-loaded base, else none (the compiled defaults). Never
    /// the union: a document nothing fits must not be given every other
    /// document's vocabulary.
    pub fn extraction_ontology(&self, doc_name: &str) -> Option<&Ontology> {
        self.for_doc(doc_name).or(self.base.as_ref())
    }

    /// The corpus-scoped ontology that is this KB's own, if it has one.
    pub fn own_corpus(&self) -> Option<String> {
        self.base
            .iter()
            .chain(self.ontologies.values())
            .find(|o| o.scope.as_deref() == Some("corpus"))
            .map(|o| o.ontology.name.clone())
    }

    /// Record one document's choice, adding its ontology when one was chosen.
    /// Other documents' choices are kept, so documents can be selected one at
    /// a time, as they are ingested.
    pub fn record(&mut self, doc: &str, choice: DocChoice, lib: &Library) {
        if let Some(c) = choice.ontology.as_ref().and_then(|n| lib.get(n)) {
            self.ontologies.insert(c.name.clone(), c.ontology.clone());
        }
        self.docs.insert(doc.to_string(), choice);
        // Drop ontologies no document uses any more.
        let used: std::collections::BTreeSet<String> = self
            .docs
            .values()
            .filter_map(|d| d.ontology.clone())
            .collect();
        self.ontologies.retain(|name, _| used.contains(name));
    }

    /// The union of every chosen ontology: what the KB as a whole admits.
    pub fn union(&self, kb: &str) -> Option<Ontology> {
        let mut used: Vec<Ontology> = self.base.iter().cloned().collect();
        used.extend(
            self.docs
                .values()
                .filter_map(|d| d.ontology.as_ref())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .filter(|n| self.base.as_ref().is_none_or(|b| &b.ontology.name != *n))
                .filter_map(|n| self.ontologies.get(n).cloned()),
        );
        (!used.is_empty()).then(|| Ontology::union(&format!("{kb} (per-document union)"), &used))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shipped_dir() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/kwaai-knowledge/ontologies")
    }

    #[test]
    fn the_shipped_library_loads_without_errors() {
        let (lib, errors) = Library::load_dir(&shipped_dir()).unwrap();
        assert!(errors.is_empty(), "{errors:?}");
        assert!(lib.candidates.len() >= 15, "{}", lib.candidates.len());
        for name in [
            "us-case-law",
            "ai-risk-standards",
            "spaceflight-and-cosmology",
            "manhattan-project-sources",
        ] {
            assert!(
                lib.get(name).unwrap().term_count() >= 20,
                "{name} has distinctive vocabulary"
            );
        }
    }

    #[test]
    fn a_court_opinion_selects_case_law_and_a_climate_text_does_not() {
        let (lib, _) = Library::load_dir(&shipped_dir()).unwrap();
        let cfg = SelectConfig::default();
        let opinion: Vec<String> = [
            "MR. CHIEF JUSTICE TANEY delivered the opinion of the Court in Dred Scott v. Sandford, 60 U.S. 393. The Circuit Court held that the Missouri Compromise was void.",
            "Congress passed the Missouri Compromise; the question is whether the Circuit Court had jurisdiction and whether the Fifth Amendment protects the property claimed.",
            "The judgment of the Circuit Court is reversed. Lochner v. New York and Plessy v. Ferguson were cited; the dissent of Mr. Justice Curtis follows.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let c = select(&lib, "opinion.html", &opinion, &cfg);
        assert_eq!(
            c.ontology.as_deref(),
            Some("us-case-law"),
            "{:?}",
            &c.scores[..3]
        );
        assert_eq!(c.doc_type.as_deref(), Some("CourtOpinion"));
        let climate: Vec<String> = [
            "The AMOC is a tipping element; greenhouse gas forcing weakens it in CMIP6 model simulations.",
            "Aerosols and well-mixed greenhouse gases are the main forcings; the Boreal forest is another tipping element.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let c = select(&lib, "climate.pdf", &climate, &cfg);
        assert_ne!(
            c.ontology.as_deref(),
            Some("us-case-law"),
            "{:?}",
            &c.scores[..3]
        );
    }

    #[test]
    fn nothing_is_chosen_when_no_candidate_fits() {
        let (lib, _) = Library::load_dir(&shipped_dir()).unwrap();
        let chunks =
            vec!["Preheat the oven and whisk two eggs with flour and a pinch of salt.".to_string()];
        assert_eq!(
            select(&lib, "recipe.txt", &chunks, &SelectConfig::default()).ontology,
            None
        );
    }

    #[test]
    fn document_types_come_from_identifiers() {
        assert_eq!(
            detect_doc_type("Roe v. Wade, 410 U.S. 113 (1973)").map(|t| t.0),
            Some("CourtOpinion")
        );
        assert_eq!(
            detect_doc_type("Dred Scott v. Sandford, 60 U. S. 393").map(|t| t.0),
            Some("CourtOpinion")
        );
        assert_eq!(
            detect_doc_type("Be it enacted by the Senate").map(|t| t.0),
            Some("Legislation")
        );
        assert_eq!(
            detect_doc_type("doi: 10.1103/PhysRevLett.116.061102").map(|t| t.0),
            Some("ScholarlyArticle")
        );
        assert_eq!(
            detect_doc_type("ISBN 978-0-19-953556-9").map(|t| t.0),
            Some("Book")
        );
        assert_eq!(detect_doc_type("It was a dark and stormy night."), None);
        assert_eq!(
            detect_doc_type("released in 10.2 seconds"),
            None,
            "not a DOI"
        );
    }

    /// A document typed as a court opinion is chosen among the ontologies that
    /// declare court opinions, even when a larger ontology outscores them on
    /// shared vocabulary.
    #[test]
    fn a_detected_document_type_narrows_the_choice() {
        let (lib, _) = Library::load_dir(&shipped_dir()).unwrap();
        let chunks: Vec<String> = [
            "The white and colored races were assigned separate coaches; the colored passenger refused.",
            "The statute of Louisiana required separate railway carriages for the white and colored races.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let cfg = SelectConfig {
            min_share: 0.0,
            ..Default::default()
        };
        let c = select(&lib, "Plessy v Ferguson 163 US 537 (1896)", &chunks, &cfg);
        assert_eq!(c.doc_type.as_deref(), Some("CourtOpinion"));
        assert!(
            c.scores.iter().all(|s| s.ontology == "us-case-law"),
            "{:?}",
            c.scores
        );
    }

    /// The shipped library plus the synthetic corpus-scoped memoir ontology.
    fn library_with_test_memoir() -> Library {
        let mut members: Vec<(String, Option<String>, Ontology)> = Vec::new();
        for p in std::fs::read_dir(shipped_dir()).unwrap().flatten() {
            let p = p.path();
            if p.extension().and_then(|x| x.to_str()) != Some("yaml") {
                continue;
            }
            if let Ok(o) = Ontology::from_yaml(&std::fs::read_to_string(&p).unwrap()) {
                if !o.is_empty() {
                    let file = p.file_name().unwrap().to_string_lossy().to_string();
                    members.push((file, None, o));
                }
            }
        }
        members.push((
            "test-memoir.yaml".into(),
            None,
            crate::ontology::test_memoir(),
        ));
        Library::new(members)
    }

    /// A corpus-scoped ontology is eligible only in its own KB.
    #[test]
    fn a_corpus_ontology_is_never_chosen_for_other_documents() {
        let lib = library_with_test_memoir();
        let memoir: Vec<String> = [
            "Granny Ilse and Uncle Bram lived on Vos Street; my mother was the daughter of a cooper.",
            "Pieter was the son of Jan, and the family lived at the mill by the river.",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let anywhere = select(&lib, "memoir.pdf", &memoir, &SelectConfig::default());
        assert!(
            anywhere.scores.iter().all(|s| s.ontology != "test-memoir"),
            "{:?}",
            anywhere.scores
        );
        let own = SelectConfig {
            own_corpus: Some("test-memoir".into()),
            ..Default::default()
        };
        let mine = select(&lib, "memoir.pdf", &memoir, &own);
        assert_eq!(
            mine.ontology.as_deref(),
            Some("test-memoir"),
            "in its own KB it is chosen: {:?}",
            mine.scores
        );
    }

    #[test]
    fn recording_one_document_keeps_the_others() {
        let (lib, _) = Library::load_dir(&shipped_dir()).unwrap();
        let mut d = DocOntologies::default();
        let pick = |o: Option<&str>| DocChoice {
            ontology: o.map(String::from),
            method: "coverage".into(),
            doc_type: None,
            doc_type_evidence: None,
            chunks_sampled: 0,
            scores: vec![],
        };
        d.record("a.pdf", pick(Some("us-case-law")), &lib);
        d.record("b.pdf", pick(Some("ai-risk-standards")), &lib);
        assert_eq!(d.ontologies.len(), 2);
        d.record("b.pdf", pick(None), &lib);
        assert_eq!(d.docs.len(), 2, "b.pdf is still recorded, with no ontology");
        assert_eq!(
            d.ontologies.keys().collect::<Vec<_>>(),
            vec!["us-case-law"],
            "unused ontology dropped"
        );
    }

    #[test]
    fn windows_split_at_whitespace_near_the_size() {
        let w = windows("aaa bbb ccc ddd eee", 8);
        assert_eq!(w, vec!["aaa bbb", "ccc ddd", "eee"]);
        assert!(windows("", 10).is_empty());
    }

    /// Regression: byte-range slicing panicked when a multibyte character sat
    /// across the end of the evidence snippet, or when lowercasing changed a
    /// character's length ahead of the match.
    #[test]
    fn document_type_detection_never_panics_on_non_ascii_text() {
        assert_eq!(
            detect_doc_type("ISBN 0-19-953556-9 (Oxford, 1998 — 2nd ed.").map(|t| t.0),
            Some("Book")
        );
        assert_eq!(
            detect_doc_type("İİİİ Public Law 85-568 — the Act — é").map(|t| t.0),
            Some("Legislation")
        );
        assert_eq!(
            detect_doc_type("ẞẞ doi 10.1103/PhysRev—Lett x").map(|t| t.0),
            Some("ScholarlyArticle")
        );
        let _ = detect_doc_type("—— be it enacted ——— ẞ İ K");
    }

    #[test]
    fn an_empty_eligible_pool_chooses_nothing() {
        let c = select(
            &Library::default(),
            "a.pdf",
            &["some text".to_string()],
            &SelectConfig::default(),
        );
        assert_eq!(c.ontology, None);
        assert!(c.scores.is_empty());
    }

    /// A KB's hand-loaded ontology survives selection: documents with no
    /// choice keep using it, and the union includes it.
    #[test]
    fn the_hand_loaded_base_is_kept_and_unselected_documents_use_it() {
        let lib = library_with_test_memoir();
        let mut d = DocOntologies {
            base: Some(lib.get("test-memoir").unwrap().ontology.clone()),
            ..Default::default()
        };
        let pick = |o: Option<&str>| DocChoice {
            ontology: o.map(String::from),
            method: "coverage".into(),
            ..Default::default()
        };
        d.record("opinion.pdf", pick(Some("us-case-law")), &lib);
        d.record("recipe.txt", pick(None), &lib);
        assert_eq!(
            d.extraction_ontology("opinion.pdf").unwrap().ontology.name,
            "us-case-law"
        );
        assert_eq!(
            d.extraction_ontology("recipe.txt").unwrap().ontology.name,
            "test-memoir",
            "base, not the union"
        );
        assert_eq!(
            d.extraction_ontology("never-selected.pdf")
                .unwrap()
                .ontology
                .name,
            "test-memoir"
        );
        let u = d.union("T").unwrap();
        assert!(u.relation_type("overrules").is_some() && u.relation_type("lived_at").is_some());
        assert_eq!(d.own_corpus().as_deref(), Some("test-memoir"));
        let no_base = DocOntologies::default();
        assert!(
            no_base.extraction_ontology("recipe.txt").is_none(),
            "no base: compiled defaults"
        );
    }

    /// A one-ontology library can still choose it.
    #[test]
    fn a_single_candidate_library_can_choose() {
        let (full, _) = Library::load_dir(&shipped_dir()).unwrap();
        let lib = Library::new(vec![(
            "Legal.yaml".into(),
            None,
            full.get("us-case-law").unwrap().ontology.clone(),
        )]);
        let chunks: Vec<String> = vec!["The Circuit Court held the statute void; the Supreme Court reversed. Chief Justice Taney delivered the opinion.".to_string(); 3];
        assert_eq!(
            select(&lib, "x.pdf", &chunks, &SelectConfig::default())
                .ontology
                .as_deref(),
            Some("us-case-law")
        );
    }

    /// Old stored records (fewer fields) still parse.
    #[test]
    fn a_stored_record_missing_new_fields_still_parses() {
        let d: DocOntologies =
            serde_json::from_str(r#"{"ontologies":{},"docs":{"a.pdf":{"ontology":null}}}"#)
                .unwrap();
        assert!(d.docs["a.pdf"].ontology.is_none() && d.base.is_none());
    }

    #[test]
    fn terms_match_whole_words_only() {
        assert!(contains_term("the case law of the court", "case law"));
        assert!(!contains_term("showcase lawful", "case law"));
    }

    #[test]
    fn a_union_admits_what_any_member_declares() {
        let (lib, _) = Library::load_dir(&shipped_dir()).unwrap();
        let mut d = DocOntologies::default();
        for (doc, name) in [("a.pdf", "us-case-law"), ("b.pdf", "ai-risk-standards")] {
            d.ontologies
                .insert(name.into(), lib.get(name).unwrap().ontology.clone());
            d.docs.insert(
                doc.into(),
                DocChoice {
                    ontology: Some(name.into()),
                    method: "coverage".into(),
                    doc_type: None,
                    doc_type_evidence: None,
                    chunks_sampled: 0,
                    scores: vec![],
                },
            );
        }
        let u = d.union("T").unwrap();
        assert!(u.relation_type("overrules").is_some() && u.relation_type("mitigates").is_some());
        assert_eq!(
            d.for_doc("b.pdf").unwrap().ontology.name,
            "ai-risk-standards"
        );
        assert!(d.for_doc("missing.pdf").is_none());
    }
}
