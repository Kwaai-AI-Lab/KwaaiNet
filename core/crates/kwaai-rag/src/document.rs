use std::io::Read;
use std::path::Path;

use anyhow::{bail, Context, Result};

/// Extract plain text from a document file.
///
/// Supported formats:
/// - `.txt`, `.md`, `.rst`, `.csv`, `.json`, `.yaml`, `.toml` — read as UTF-8
/// - `.pdf` — extracted via pdf-extract (requires the "pdf" feature)
/// - `.docx` — extracted from the embedded XML (no extra tools needed)
/// - `.doc`  — extracted via `antiword` or `libreoffice --headless` (must be installed)
pub fn extract_text(path: &Path) -> Result<String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "pdf" => extract_pdf(path),
        "docx" => extract_docx(path),
        "doc" => extract_doc_legacy(path),
        _ => std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display())),
    }
}

/// Returns the file extensions this module can handle, for use as sync defaults.
pub fn supported_extensions() -> &'static [&'static str] {
    &[
        "txt", "md", "rst", "csv", "json", "yaml", "toml", "pdf", "docx", "doc",
    ]
}

// ── PDF ───────────────────────────────────────────────────────────────────────

/// Below this lexical score, extracted text is treated as probably garbled.
///
/// Calibrated against the regression that blocked the 0.12 upgrade: on
/// `scaling-trust-ebook.pdf`, 0.10 scores ~0.92 and 0.12 ~0.80, because a broken
/// font map turns roughly an eighth of the document into shifted-glyph noise
/// ("Organizations are under" → "Svkermexmsrw evi yrhiv"). Anything at or above
/// this is accepted without a second opinion.
#[cfg(feature = "pdf")]
const LEXICAL_ACCEPT: f32 = 0.88;

/// Minimum substantial lines before a score means anything. Short documents,
/// slide decks, and tables legitimately have few prose lines; scoring those
/// would trigger pointless fallbacks.
#[cfg(feature = "pdf")]
const MIN_SCORED_LINES: usize = 12;

/// Skip the 0.10 second opinion above this size.
///
/// 0.10's parser is O(n²) in object count — the very bug that motivated moving
/// to 0.12. Falling back on a huge file could hang for minutes, so past this
/// point we keep 0.12's output and warn instead.
#[cfg(feature = "pdf")]
const FALLBACK_MAX_BYTES: u64 = 64 * 1024 * 1024;

/// Whether the legacy second opinion is permitted.
///
/// The fallback is what keeps garbled text out of the corpus, but it is also the
/// only thing that loads pdf-extract 0.10 and with it lopdf 0.38, which carries
/// RUSTSEC-2026-0187 (stack overflow on deeply nested objects). Reaching that
/// path requires a PDF that both garbles under 0.12 *and* is maliciously nested,
/// and a stack overflow aborts rather than unwinds, so `catch_unwind` cannot save
/// us. Deployments ingesting genuinely untrusted PDFs can set
/// `KWAAI_PDF_LEGACY_FALLBACK=0` to refuse the fallback and accept that some
/// documents extract badly instead.
#[cfg(feature = "pdf")]
fn legacy_fallback_enabled() -> bool {
    !matches!(
        std::env::var("KWAAI_PDF_LEGACY_FALLBACK").as_deref(),
        Ok("0") | Ok("false") | Ok("no")
    )
}

/// Extract text from a PDF, preferring the fast parser but never trusting it blindly.
///
/// `pdf-extract` 0.12 is 3–7× faster than 0.10 and fixes RUSTSEC-2026-0187 (stack
/// overflow on deeply nested objects — and PDFs are untrusted input by definition).
/// But on some font encodings it silently returns shifted-glyph gibberish: the
/// extraction *succeeds*, the character count looks plausible, and the garbage
/// flows into the corpus and gets embedded where nobody ever sees it.
///
/// So: run 0.12, score the result, and only when it looks non-lexical pay for a
/// second opinion from 0.10 and keep whichever scores better. Clean documents —
/// the overwhelming majority — never touch 0.10 and never load the vulnerable
/// parser at all.
#[cfg(feature = "pdf")]
fn extract_pdf(path: &Path) -> Result<String> {
    let primary = run_extractor(path, Extractor::Fast);

    // 0.12 produced something. Decide whether to believe it.
    if let Ok(text) = &primary {
        let cleaned = clean_pdf_text(text);
        let score = lexical_score(&cleaned);

        // Not enough prose to judge, or it reads fine — take it.
        if !score.is_conclusive() || score.ratio >= LEXICAL_ACCEPT {
            return Ok(cleaned);
        }

        if !legacy_fallback_enabled() {
            tracing::warn!(
                path = %path.display(),
                lexical_score = score.ratio,
                "PDF text looks garbled, but the legacy fallback is disabled \
                 (KWAAI_PDF_LEGACY_FALLBACK=0); keeping possibly-corrupt text"
            );
            return Ok(cleaned);
        }

        let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        if size > FALLBACK_MAX_BYTES {
            tracing::warn!(
                path = %path.display(),
                lexical_score = score.ratio,
                size_mb = size / (1024 * 1024),
                "PDF text looks garbled, but the file is too large to re-parse with the \
                 slower extractor; keeping possibly-corrupt text"
            );
            return Ok(cleaned);
        }

        tracing::info!(
            path = %path.display(),
            lexical_score = score.ratio,
            "PDF text looks garbled; retrying with the legacy extractor"
        );

        // Second opinion. If it is genuinely better, prefer it.
        if let Ok(legacy) = run_extractor(path, Extractor::Legacy) {
            let legacy_cleaned = clean_pdf_text(&legacy);
            let legacy_score = lexical_score(&legacy_cleaned);
            if legacy_score.ratio > score.ratio {
                tracing::info!(
                    path = %path.display(),
                    from = score.ratio,
                    to = legacy_score.ratio,
                    "legacy extractor recovered the text"
                );
                return Ok(legacy_cleaned);
            }
            tracing::warn!(
                path = %path.display(),
                lexical_score = score.ratio,
                "both extractors produced low-lexical text; the source PDF may be \
                 scanned, non-English, or genuinely unparseable"
            );
        }
        return Ok(cleaned);
    }

    // 0.12 failed outright (error or panic). 0.10 parses some malformed files
    // that 0.12 rejects, so it is still worth one attempt before giving up.
    if !legacy_fallback_enabled() {
        return Err(primary.unwrap_err());
    }
    match run_extractor(path, Extractor::Legacy) {
        Ok(text) => Ok(clean_pdf_text(&text)),
        Err(_) => Err(primary.unwrap_err()),
    }
}

#[cfg(feature = "pdf")]
#[derive(Clone, Copy)]
enum Extractor {
    /// pdf-extract 0.12 — fast, lopdf 0.42, no known advisory.
    Fast,
    /// pdf-extract 0.10 — slow, lopdf 0.38 (RUSTSEC-2026-0187), better font handling.
    Legacy,
}

/// Run one extractor, converting its panics into errors.
///
/// Both versions panic on some malformed PDFs (wrong object types), so a single
/// bad file must not take down a sync run.
#[cfg(feature = "pdf")]
fn run_extractor(path: &Path, which: Extractor) -> Result<String> {
    let path_owned = path.to_path_buf();
    // The two majors have distinct `OutputError` types, so flatten to String
    // inside each arm rather than trying to unify them.
    let result = std::panic::catch_unwind(move || match which {
        Extractor::Fast => pdf_extract::extract_text(&path_owned).map_err(|e| e.to_string()),
        Extractor::Legacy => {
            pdf_extract_legacy::extract_text(&path_owned).map_err(|e| e.to_string())
        }
    });
    match result {
        Ok(Ok(text)) => Ok(text),
        Ok(Err(e)) => bail!("extracting PDF text from {}: {e}", path.display()),
        Err(_) => bail!(
            "extracting PDF text from {}: PDF is malformed (internal parser panic)",
            path.display()
        ),
    }
}

/// How much of the text reads like real prose.
#[cfg(feature = "pdf")]
#[derive(Debug, Clone, Copy)]
struct LexicalScore {
    /// Fraction of substantial lines containing at least one common word.
    ratio: f32,
    /// Substantial lines examined.
    lines: usize,
}

#[cfg(feature = "pdf")]
impl LexicalScore {
    fn is_conclusive(&self) -> bool {
        self.lines >= MIN_SCORED_LINES
    }
}

/// Words frequent enough that almost any English line of prose contains one, and
/// short enough that a glyph shift destroys them. A Caesar-style font offset maps
/// "the" to "xli", so a corrupted document scores near zero on this.
#[cfg(feature = "pdf")]
const COMMON_WORDS: &[&str] = &[
    "the", "and", "of", "to", "in", "a", "is", "that", "for", "it", "as", "with", "on", "was",
    "by", "at", "from", "or", "this", "an", "be", "are", "not", "but", "have", "has", "which",
    "we", "you", "they", "their", "there", "were", "been", "would", "can", "will", "all", "if",
    "no", "so", "its", "our", "more", "when", "than", "these", "may", "one",
];

/// Score text on whether it reads like language rather than shifted-glyph noise.
///
/// Deliberately crude: a line counts as lexical if it holds one common word. That
/// is nearly free to compute and survives tables, headers, and citation blocks,
/// while a broken font map — which shifts *every* letter uniformly — collapses to
/// almost zero because no common word survives intact.
#[cfg(feature = "pdf")]
fn lexical_score(text: &str) -> LexicalScore {
    let mut substantial = 0usize;
    let mut lexical = 0usize;

    for line in text.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        // Short lines are headings, page numbers, table cells — no signal.
        if words.len() < 5 {
            continue;
        }
        substantial += 1;
        let hit = words.iter().any(|w| {
            let cleaned: String = w
                .chars()
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
                .collect();
            COMMON_WORDS.contains(&cleaned.as_str())
        });
        if hit {
            lexical += 1;
        }
    }

    LexicalScore {
        ratio: if substantial == 0 {
            1.0
        } else {
            lexical as f32 / substantial as f32
        },
        lines: substantial,
    }
}

/// Fix underscore artifacts introduced by pdf-extract's glyph-to-Unicode mapping.
///
/// Some PDFs encode periods and apostrophes using glyph IDs that pdf-extract maps
/// to `_` instead of the intended character:
///   "Dr_"       → "Dr."      (period after abbreviation)
///   "J_ M_ H_"  → "J. M. H." (spaced initials)
///   "M_K_"      → "M.K."     (chained initials — next char is uppercase)
///   "Wooding_s" → "Wooding's" (apostrophe in possessive)
///
/// Rules applied in order per `_`:
///   1. `_s` at a word boundary → `'s`
///   2. `_` preceded by a letter and followed by whitespace, end, or an uppercase
///      letter (chained initial) → `.`
///   3. All other underscores → stripped
#[allow(dead_code)]
fn clean_pdf_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    let mut i = 0;
    while i < n {
        let c = chars[i];
        if c != '_' {
            out.push(c);
            i += 1;
            continue;
        }
        // Rule 1: `_s` where s is followed by non-alpha or end → apostrophe-s
        if i + 1 < n && chars[i + 1] == 's' {
            let after = i + 2;
            let at_boundary = after >= n || !chars[after].is_alphabetic();
            if at_boundary {
                out.push('\'');
                out.push('s');
                i += 2;
                continue;
            }
        }
        // Rule 2: `_` preceded by a letter and followed by whitespace, end, or an
        // uppercase letter (chained initials like M_K_ → M.K.) → period
        let prev_is_alpha = out
            .chars()
            .last()
            .map(|p| p.is_alphabetic())
            .unwrap_or(false);
        let next_is_break_or_initial = i + 1 >= n
            || chars[i + 1].is_whitespace()
            || chars[i + 1] == '\n'
            || chars[i + 1] == '\r'
            || chars[i + 1].is_uppercase();
        if prev_is_alpha && next_is_break_or_initial {
            out.push('.');
            i += 1;
            continue;
        }
        // Rule 3: strip
        i += 1;
    }
    out
}

#[cfg(not(feature = "pdf"))]
fn extract_pdf(_path: &Path) -> Result<String> {
    bail!("PDF support requires rebuilding with the 'pdf' feature:\n  cargo build --features pdf");
}

// ── DOCX ──────────────────────────────────────────────────────────────────────

fn extract_docx(path: &Path) -> Result<String> {
    let file = std::fs::File::open(path).with_context(|| format!("opening {}", path.display()))?;
    let mut archive = zip::ZipArchive::new(file)
        .with_context(|| format!("reading DOCX archive {}", path.display()))?;

    let mut xml = String::new();
    archive
        .by_name("word/document.xml")
        .with_context(|| "word/document.xml not found — is this a valid .docx file?")?
        .read_to_string(&mut xml)?;

    parse_ooxml_text(&xml)
}

/// Extract text from Office Open XML (word/document.xml).
/// Reads `<w:t>` elements; inserts spaces at paragraph (`<w:p>`) and run breaks.
fn parse_ooxml_text(xml: &str) -> Result<String> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);

    let mut out = String::with_capacity(xml.len() / 4);
    let mut in_text = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e) | Event::Empty(ref e)) => {
                match e.name().as_ref() {
                    b"w:t" => in_text = true,
                    // Paragraph end — add a newline so sentences don't merge.
                    b"w:p" if !out.is_empty() && !out.ends_with('\n') => out.push('\n'),
                    b"w:p" => {}
                    // Line / carriage-return break inside a run.
                    b"w:br" | b"w:cr" => out.push('\n'),
                    // Tab character.
                    b"w:tab" => out.push('\t'),
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"w:t" => {
                in_text = false;
            }
            Ok(Event::End(_)) => {}
            Ok(Event::Text(e)) if in_text => {
                // 0.41 renamed the decode step and no longer folds entity
                // references in here — see GeneralRef below.
                out.push_str(e.xml10_content().as_deref().unwrap_or(""));
            }
            // quick-xml 0.41 emits `&amp;`, `&#39;` and friends as their own
            // event instead of unescaping them into the surrounding Text. Left
            // unhandled they fall through the catch-all and vanish silently,
            // which in a .docx means dropped ampersands and apostrophes.
            Ok(Event::GeneralRef(e)) if in_text => {
                match e.resolve_char_ref() {
                    // Numeric: &#39; / &#x27;
                    Ok(Some(ch)) => out.push(ch),
                    // Named: &amp; &lt; &gt; &quot; &apos;
                    Ok(None) => {
                        if let Ok(name) = e.decode() {
                            if let Some(text) = quick_xml::escape::resolve_predefined_entity(&name)
                            {
                                out.push_str(text);
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => bail!("XML parse error in DOCX: {e}"),
            _ => {}
        }
    }

    Ok(out)
}

// ── Legacy .doc ───────────────────────────────────────────────────────────────

fn extract_doc_legacy(path: &Path) -> Result<String> {
    // Try antiword first (lighter, faster).
    if let Ok(out) = std::process::Command::new("antiword").arg(path).output() {
        if out.status.success() {
            return Ok(String::from_utf8_lossy(&out.stdout).into_owned());
        }
    }

    // Fall back to LibreOffice headless conversion.
    let tmp = std::env::temp_dir().join("kwaainet-doc-convert");
    std::fs::create_dir_all(&tmp)?;
    let status = std::process::Command::new("libreoffice")
        .args(["--headless", "--convert-to", "txt:Text", "--outdir"])
        .arg(&tmp)
        .arg(path)
        .status();

    if let Ok(s) = status {
        if s.success() {
            let stem = path.file_stem().unwrap_or_default();
            let txt = tmp.join(stem).with_extension("txt");
            if txt.exists() {
                return std::fs::read_to_string(&txt)
                    .with_context(|| format!("reading converted file {}", txt.display()));
            }
        }
    }

    bail!(
        "Cannot extract text from '{}'. \
        Install antiword (brew install antiword) or LibreOffice.",
        path.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_pdf_text_chained_initials() {
        // Chained initials without spaces: M_K_ → M.K.
        assert_eq!(clean_pdf_text("M_K_ Gandhi"), "M.K. Gandhi");
        // Initials with spaces: J_ M_ H_ → J. M. H.
        assert_eq!(clean_pdf_text("J_ M_ H_ Gool"), "J. M. H. Gool");
        // Trailing period on title: Dr_ → Dr.
        assert_eq!(clean_pdf_text("Dr_ Smith"), "Dr. Smith");
        // Possessive: Wooding_s → Wooding's
        assert_eq!(clean_pdf_text("Wooding_s house"), "Wooding's house");
        // E_S_ Reddy → E.S. Reddy
        assert_eq!(clean_pdf_text("E_S_ Reddy"), "E.S. Reddy");
    }

    #[test]
    fn test_parse_ooxml_text() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>Hello </w:t></w:r><w:r><w:t>world.</w:t></w:r></w:p>
    <w:p><w:r><w:t>Second paragraph.</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
        let text = parse_ooxml_text(xml).unwrap();
        assert!(text.contains("Hello world."), "got: {text:?}");
        assert!(text.contains("Second paragraph."), "got: {text:?}");
    }

    /// Entity references must survive extraction.
    ///
    /// quick-xml 0.41 stopped folding entities into `Text` and started emitting
    /// them as `Event::GeneralRef`. Before this was handled they fell through the
    /// catch-all arm and disappeared — "R&amp;D" silently became "RD". Named and
    /// numeric forms both matter: Word writes `&amp;` for ampersands and numeric
    /// refs for typographic punctuation.
    #[test]
    fn test_parse_ooxml_entities_are_preserved() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>Smith &amp; Wesson</w:t></w:r></w:p>
    <w:p><w:r><w:t>5 &lt; 7 &gt; 3</w:t></w:r></w:p>
    <w:p><w:r><w:t>Wooding&#39;s house</w:t></w:r></w:p>
    <w:p><w:r><w:t>caf&#xE9; society</w:t></w:r></w:p>
    <w:p><w:r><w:t>She said &quot;hello&quot;</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
        let text = parse_ooxml_text(xml).unwrap();
        assert!(
            text.contains("Smith & Wesson"),
            "named entity lost: {text:?}"
        );
        assert!(text.contains("5 < 7 > 3"), "lt/gt lost: {text:?}");
        assert!(
            text.contains("Wooding's house"),
            "decimal char ref lost: {text:?}"
        );
        assert!(text.contains("café society"), "hex char ref lost: {text:?}");
        assert!(
            text.contains("She said \"hello\""),
            "quot entity lost: {text:?}"
        );
    }

    /// An entity split across runs must not lose the surrounding text either.
    #[test]
    fn test_parse_ooxml_entity_between_runs() {
        let xml = r#"<?xml version="1.0"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p><w:r><w:t>R</w:t></w:r><w:r><w:t>&amp;</w:t></w:r><w:r><w:t>D budget</w:t></w:r></w:p>
  </w:body>
</w:document>"#;
        let text = parse_ooxml_text(xml).unwrap();
        assert!(text.contains("R&D budget"), "got: {text:?}");
    }

    // ── Garbled-PDF detection ────────────────────────────────────────────────
    //
    // Regression cover for the pdf-extract 0.12 upgrade. 0.12 is faster and fixes
    // RUSTSEC-2026-0187, but silently returns shifted-glyph text on some font
    // encodings. These lock in that such output is *detected*, so the extractor
    // falls back instead of quietly poisoning the corpus.

    /// Real prose (the 0.10 output from scaling-trust-ebook.pdf).
    #[cfg(feature = "pdf")]
    const CLEAN_SAMPLE: &str = "\
Organizations are under pressure to digitize both customer-facing and internal pro-
cesses in order to remain competitive in the market they operate within today.
The efficiency of a client process is a function of the trust that exists between
the parties to it, and the cost of establishing that trust in the first place.
This is the central claim of the book and the one we return to in later chapters.
Where trust is expensive, organizations substitute process, and that process is
itself a cost that must be carried by someone in the end.
We can therefore treat trust as infrastructure rather than as sentiment, which is
the shift in framing that the rest of this argument depends upon completely.
Digital identity is one such piece of infrastructure and it is the one that has
received the most attention from regulators over the last several years now.
The result is a body of law that is not yet settled but is no longer absent.";

    /// The same passage as 0.12 renders it — every letter shifted by a broken
    /// font map. Verbatim shape of the corruption from the PR review.
    #[cfg(feature = "pdf")]
    const GARBLED_SAMPLE: &str = "\
Svkermexmsrw evi yrhiv tviwwyvi xs hmkmxmi fsxl gywxsiv1egmrk erh mrxivrep tvs-
giwwiw mr svhiv xs viqemr gsqtixmxmzi mr xli qevoix xliy stivexi amxlmr xshey.
Xli ijjmgmirgy sj e gpmirx tvsgiww mw e jyrgxmsr sj xli xvywx xlex ibmwxw fixaiir
xli tevxmiw xs mx, erh xli gswx sj iwxefpmwlmrk xlex xvywx mr xli jmvwx tpegi.
Xlmw mw xli girxvep gpemq sj xli fsso erh xli sri ai vixyvr xs mr pexiv gletxivw.
Almvi xvywx mw ibtirwmzi, svkermexmsrw wyfwxmxyxi tvsgiww, erh xlex tvsgiww mw
mxwipj e gswx xlex qywx fi gevvmih fy wsqisri mr xli irh.
Ai ger xlivijsvi xviex xvywx ew mrjvewxvygxyvi vexliv xler ew wirxmqirx, almgl mw
xli wlmjx mr jveqmrk xlex xli viwx sj xlmw evkyqirx hitirhw ytsr gsqtpixipy.
Hmkmxep mhirxmxy mw sri wygl tmigi sj mrjvewxvygxyvi erh mx mw xli sri xlex lew
vigimzih xli qswx exxirxmsr jvsq vikypexsvw sziv xli pewx wizivep yievw rsa.
Xli viwypx mw e fshy sj pea xlex mw rsx yix wixxpih fyx mw rs psrkiv efwirx.";

    #[cfg(feature = "pdf")]
    #[test]
    fn clean_text_scores_as_lexical() {
        let score = lexical_score(CLEAN_SAMPLE);
        assert!(
            score.is_conclusive(),
            "sample too short to judge: {score:?}"
        );
        assert!(
            score.ratio >= LEXICAL_ACCEPT,
            "real prose must be accepted without a fallback, got {score:?}"
        );
    }

    #[cfg(feature = "pdf")]
    #[test]
    fn shifted_glyph_text_is_detected_as_garbled() {
        let score = lexical_score(GARBLED_SAMPLE);
        assert!(
            score.is_conclusive(),
            "sample too short to judge: {score:?}"
        );
        assert!(
            score.ratio < LEXICAL_ACCEPT,
            "shifted-glyph output must trigger the fallback, got {score:?}"
        );
    }

    /// The corrupted text must score clearly worse, not marginally — that gap is
    /// what lets extract_pdf pick the better of the two extractions.
    #[cfg(feature = "pdf")]
    #[test]
    fn garbled_scores_far_below_clean() {
        let clean = lexical_score(CLEAN_SAMPLE).ratio;
        let garbled = lexical_score(GARBLED_SAMPLE).ratio;
        assert!(
            clean - garbled > 0.5,
            "expected a wide margin so selection is unambiguous; clean={clean} garbled={garbled}"
        );
    }

    /// Short inputs must be inconclusive, so slide decks and tables of figures
    /// don't drag every ingest onto the slow legacy parser.
    #[cfg(feature = "pdf")]
    #[test]
    fn short_documents_are_inconclusive() {
        let score = lexical_score("Table 4 shows the quarterly totals by region\nFigure 9 follows");
        assert!(
            !score.is_conclusive(),
            "too few lines should not produce a verdict, got {score:?}"
        );
    }

    /// A page of table rows and figure captions has no prose but isn't garbled.
    /// It may score low; it must not be *mistaken for* shifted-glyph noise, which
    /// scores near zero.
    #[cfg(feature = "pdf")]
    #[test]
    fn tabular_text_does_not_score_like_garbage() {
        let tabular = "\
Region North Units 4192 Revenue 88214 Margin 0.31
Region South Units 3771 Revenue 79002 Margin 0.28
Region East Units 5120 Revenue 96338 Margin 0.34
Region West Units 2884 Revenue 61190 Margin 0.22
Region Central Units 4406 Revenue 84771 Margin 0.30
Total All Units 20373 Revenue 409515 Margin 0.29
Quarter 1 2026 audited figures pending review
Quarter 2 2026 audited figures pending review
Quarter 3 2026 audited figures pending review
Quarter 4 2026 projected figures unaudited draft
Prepared by finance operations group internal
Distribution restricted to management committee";
        let garbled = lexical_score(GARBLED_SAMPLE).ratio;
        let tab = lexical_score(tabular).ratio;
        assert!(
            tab > garbled,
            "tabular text must outrank shifted-glyph noise; tabular={tab} garbled={garbled}"
        );
    }
}
