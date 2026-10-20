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
/// - `.html`, `.htm` — markup, scripts, styles and page chrome stripped (`html_to_text`)
pub fn extract_text(path: &Path) -> Result<String> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Trust the content over the name: Manhattan's "Handy to Spaatz.html" is a
    // scanned PDF, and read as text it became 225 chunks of binary junk.
    let ext = match sniff_format(path) {
        Some(real) if real != ext => {
            tracing::warn!(file = %path.display(), named = %ext, content = real, "file content does not match its extension; using the content");
            real.to_string()
        }
        _ => ext,
    };

    match ext.as_str() {
        "pdf" => extract_pdf(path),
        "docx" => extract_docx(path),
        "doc" => extract_doc_legacy(path),
        "html" | "htm" | "xhtml" => {
            let raw = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
            Ok(html_to_text(&String::from_utf8_lossy(&raw)))
        }
        _ => std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display())),
    }
}

/// The format a file's first bytes declare, when they declare one.
fn sniff_format(path: &Path) -> Option<&'static str> {
    let mut head = [0u8; 8];
    let n = std::fs::File::open(path)
        .and_then(|mut f| f.read(&mut head))
        .ok()?;
    format_of(&head[..n])
}

fn format_of(head: &[u8]) -> Option<&'static str> {
    if head.starts_with(b"%PDF-") {
        Some("pdf")
    } else {
        None
    }
}

/// Returns the file extensions this module can handle, for use as sync defaults.
pub fn supported_extensions() -> &'static [&'static str] {
    &[
        "txt", "md", "rst", "csv", "json", "yaml", "toml", "pdf", "docx", "doc", "html", "htm",
        "xhtml",
    ]
}

// ── HTML ──────────────────────────────────────────────────────────────────────

/// Elements whose whole content is dropped: code, styling, and page chrome.
const HTML_DROP: &[&str] = &[
    "script", "style", "head", "noscript", "svg", "template", "iframe", "nav", "header", "footer",
    "aside", "button",
];
/// Elements that end a line of text.
const HTML_BLOCK: &[&str] = &[
    "p",
    "div",
    "br",
    "li",
    "tr",
    "td",
    "th",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "blockquote",
    "pre",
    "section",
    "article",
    "table",
    "ul",
    "ol",
    "dd",
    "dt",
    "hr",
    "title",
];

/// Plain text from HTML.
///
/// `.html` used to fall through to the plain-text reader, so pages were ingested
/// as raw source: the Einstein–Szilard page became 103 chunks of CSS, and
/// PythonDocs 80% markup. HTML was also missing from `supported_extensions`, so
/// folder sync skipped it: WarPeace's 12 HTML companion works were never
/// ingested at all.
pub fn html_to_text(html: &str) -> String {
    let lower = html.to_ascii_lowercase();
    let b = html.as_bytes();
    let mut out = String::with_capacity(html.len() / 2);
    let mut i = 0;
    let mut text_start = 0;
    while i < b.len() {
        if b[i] != b'<' {
            i += 1;
            continue;
        }
        out.push_str(&decode_entities(&html[text_start..i]));
        if lower[i..].starts_with("<!--") {
            i = lower[i..].find("-->").map_or(b.len(), |e| i + e + 3);
            text_start = i;
            continue;
        }
        let end = match lower[i..].find('>') {
            Some(e) => i + e + 1,
            None => b.len(),
        };
        // An unclosed `<` takes the rest of the text as its tag: `end - 1` could
        // land inside a trailing multibyte character.
        let tag = if end <= b.len() && lower[i..].contains('>') {
            &lower[i + 1..end - 1]
        } else {
            &lower[i + 1..]
        };
        let closing = tag.starts_with('/');
        let name: String = tag
            .trim_start_matches('/')
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric())
            .collect();
        i = end;
        if !closing && HTML_DROP.contains(&name.as_str()) && !tag.ends_with('/') {
            // Skip to the matching close tag (these elements do not nest in practice).
            // A close tag HTML lets authors omit must not swallow the rest of the
            // page: `<head>` ends at `<body>`, and anything else unclosed is kept.
            let close = format!("</{name}");
            let end = lower[i..]
                .find(&close)
                .map(|e| lower[i + e..].find('>').map_or(b.len(), |g| i + e + g + 1))
                .or_else(|| {
                    (name == "head")
                        .then(|| lower[i..].find("<body"))
                        .flatten()
                        .map(|e| i + e)
                });
            if let Some(end) = end {
                i = end;
            }
        } else if HTML_BLOCK.contains(&name.as_str()) {
            out.push('\n');
        }
        text_start = i;
    }
    if text_start < b.len() {
        out.push_str(&decode_entities(&html[text_start..]));
    }
    // Collapse runs of spaces within lines and of blank lines.
    let mut clean = String::with_capacity(out.len());
    let mut blank = 0;
    for line in out.lines() {
        let l = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if l.is_empty() {
            blank += 1;
            if blank == 1 && !clean.is_empty() {
                clean.push('\n');
            }
            continue;
        }
        blank = 0;
        clean.push_str(&l);
        clean.push('\n');
    }
    clean.trim().to_string()
}

fn decode_entities(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        let after = &rest[amp + 1..];
        let semi = after.find(';').filter(|&n| n <= 10);
        let decoded = semi.and_then(|n| {
            let ent = &after[..n];
            let ch = if let Some(num) = ent.strip_prefix("#x").or_else(|| ent.strip_prefix("#X")) {
                u32::from_str_radix(num, 16).ok().and_then(char::from_u32)
            } else if let Some(num) = ent.strip_prefix('#') {
                num.parse::<u32>().ok().and_then(char::from_u32)
            } else {
                match ent {
                    "amp" => Some('&'),
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "quot" => Some('"'),
                    "apos" => Some('\''),
                    "nbsp" => Some(' '),
                    "mdash" => Some('—'),
                    "ndash" => Some('–'),
                    "lsquo" => Some('‘'),
                    "rsquo" => Some('’'),
                    "ldquo" => Some('“'),
                    "rdquo" => Some('”'),
                    "hellip" => Some('…'),
                    "shy" => Some('\u{AD}'),
                    _ => None,
                }
            };
            ch.map(|c| (c, n))
        });
        match decoded {
            Some((c, n)) => {
                out.push(c);
                rest = &after[n + 1..];
            }
            None => {
                out.push('&');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod html_tests {
    use super::*;

    #[test]
    fn scripts_styles_head_and_chrome_are_dropped() {
        let t = html_to_text(
            "<html><head><title>T</title><style>.a{color:red}</style></head><body>\
             <nav><a href='/'>Home</a> · Site Map</nav>\
             <script>var x = 1; function f(){}</script>\
             <p>Some recent work by E. Fermi and L. Szilard &mdash; communicated to me.</p>\
             <footer>Privacy · Support Our Site</footer></body></html>",
        );
        assert_eq!(
            t,
            "Some recent work by E. Fermi and L. Szilard — communicated to me."
        );
    }

    #[test]
    fn block_tags_break_lines_and_inline_tags_do_not() {
        // Adjacent paragraphs keep one blank line between them: paragraph structure survives.
        let t = html_to_text("<p>War is <i>merely</i> the continuation</p><p>of policy.</p>");
        assert_eq!(t, "War is merely the continuation\n\nof policy.");
        assert_eq!(html_to_text("one<br>two"), "one\ntwo");
    }

    #[test]
    fn entities_are_decoded_and_unknown_ones_kept() {
        assert_eq!(
            decode_entities("A &amp; B &#8212; C &#x2019;s &bogus; D"),
            "A & B — C ’s &bogus; D"
        );
    }

    /// Regression: an unclosed `<` before a multibyte tail panicked.
    #[test]
    fn an_unclosed_tag_before_a_multibyte_tail_does_not_panic() {
        let _ = html_to_text("price < 5 €");
        let _ = html_to_text("<p>a</p><é");
    }

    #[test]
    fn comments_are_dropped() {
        assert_eq!(html_to_text("<p>a<!-- hidden <p>x</p> -->b</p>"), "ab");
    }

    #[test]
    fn an_unclosed_head_or_dropped_element_keeps_the_body() {
        // `</head>` is optional in HTML; the body must survive its absence.
        let t = html_to_text("<head><title>T</title><body><p>kept</p></body>");
        assert!(t.contains("kept") && !t.contains('T'), "{t:?}");
        // An unclosed dropped element drops only its tag, not the page.
        assert!(html_to_text("<nav>menu<p>kept</p>").contains("kept"));
        // Forms wrap the body text of many archival pages.
        assert!(html_to_text("<form><p>kept</p></form>").contains("kept"));
    }

    #[test]
    fn a_pdf_is_recognised_by_its_content() {
        assert_eq!(format_of(b"%PDF-1.4\n%"), Some("pdf"));
        assert_eq!(format_of(b"<!DOCTYPE"), None);
        let dir = std::env::temp_dir().join(format!("sniff-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("orders.html");
        std::fs::write(&f, b"%PDF-1.4 binary").unwrap();
        assert_eq!(sniff_format(&f), Some("pdf"));
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn html_is_a_supported_extension() {
        assert!(supported_extensions().contains(&"html"));
    }
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

/// Ontology-gated form of [`clean_pdf_text`].
///
/// The underscore→initials rewrite ("J_ M_ H_" → "J. M. H.") is an artifact of
/// one PDF's glyph mapping, not a property of text, yet it was applied to every
/// document in every KB. A corpus that legitimately uses underscores — code
/// documentation, for instance — was being corrupted by it.
/// Every fix `clean_pdf_text` applies except the underscore→initials rewrite.
pub fn clean_pdf_text_no_underscore_rule(text: &str) -> String {
    // Protect underscores through the shared cleaner, then restore them.
    const SENTINEL: char = '\u{E000}'; // private-use area: cannot occur in source text
    let guarded: String = text
        .chars()
        .map(|c| if c == '_' { SENTINEL } else { c })
        .collect();
    clean_pdf_text(&guarded)
        .chars()
        .map(|c| if c == SENTINEL { '_' } else { c })
        .collect()
}

pub fn clean_pdf_text_with_rules(s: &str, ont: Option<&crate::ontology::Ontology>) -> String {
    match ont {
        // Absent an ontology, keep the historical behaviour so no existing KB
        // changes; a KB that declares rules gets exactly what it asks for.
        None => clean_pdf_text(s),
        Some(o) if o.text_rules.underscores_are_initials => clean_pdf_text(s),
        Some(_) => clean_pdf_text_no_underscore_rule(s),
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
pub fn clean_pdf_text(text: &str) -> String {
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
