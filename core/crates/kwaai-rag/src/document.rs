use std::io::Read;
use std::path::Path;

use anyhow::{bail, Context, Result};

/// Extract plain text from a document file.
///
/// Supported formats:
/// - `.txt`, `.md`, `.rst`, `.csv`, `.json`, `.yaml`, `.toml` — read as UTF-8
/// - `.pdf` — extracted via unpdf (requires the "pdf" feature)
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

#[cfg(feature = "pdf")]
fn extract_pdf(path: &Path) -> Result<String> {
    // A parser panic on a malformed PDF must not take down a whole sync, so the
    // extraction is contained the same way it was under pdf-extract.
    let path_owned = path.to_path_buf();
    let extracted = std::panic::catch_unwind(move || -> Result<String> {
        let doc = unpdf::parse_file(&path_owned)
            .map_err(|e| anyhow::anyhow!("parsing PDF {}: {e}", path_owned.display()))?;
        unpdf::render::to_text(&doc, &pdf_render_options())
            .map_err(|e| anyhow::anyhow!("rendering PDF text {}: {e}", path_owned.display()))
    });

    match extracted {
        Ok(Ok(text)) => Ok(text),
        Ok(Err(e)) => bail!("extracting PDF text from {}: {e}", path.display()),
        Err(_) => bail!(
            "extracting PDF text from {}: PDF is malformed (internal parser panic)",
            path.display()
        ),
    }
}

/// Render options tuned for tokenizer ingestion.
///
/// The `standard` cleanup preset is right for RAG — it normalizes Unicode,
/// strips page numbers and TOC dot-leaders, and collapses whitespace runs that
/// would otherwise burn tokens. One step has to be disabled:
///
/// `fix_hyphenation` joins `([a-zA-Z])-\s*\n?\s*([a-z])`, and because the `\n`
/// is optional it also fuses ordinary same-line compounds — `aide-de-camp`
/// becomes `aidedecamp`. On a Project Gutenberg War and Peace it destroyed 93%
/// of the document's hyphens (1771 → 126). unpdf already rejoins genuine
/// line-wrapped words while reconstructing layout, so turning this off loses
/// nothing: measured output has zero dangling line-end hyphens either way, and
/// the hyphen count then matches `pdftotext` exactly.
#[cfg(feature = "pdf")]
fn pdf_render_options() -> unpdf::render::RenderOptions {
    let cleanup = unpdf::render::CleanupOptions {
        fix_hyphenation: false,
        ..unpdf::render::CleanupOptions::standard()
    };

    unpdf::render::RenderOptions {
        cleanup: Some(cleanup),
        ..Default::default()
    }
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
                out.push_str(e.unescape().as_deref().unwrap_or(""));
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

    /// Guards the one cleanup step we deliberately turn off. If a future unpdf
    /// release fixes the over-broad regex and we drop the override, this keeps
    /// the intent visible; if someone restores the default preset, it fails.
    #[cfg(feature = "pdf")]
    #[test]
    fn pdf_cleanup_keeps_compound_hyphens() {
        let cleanup = pdf_render_options()
            .cleanup
            .expect("render options must carry explicit cleanup settings");
        assert!(
            !cleanup.fix_hyphenation,
            "fix_hyphenation fuses same-line compounds (aide-de-camp → aidedecamp)"
        );
        // The rest of the standard preset should still be in force.
        assert!(cleanup.normalize_unicode);
        assert!(cleanup.normalize_whitespace);
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
}
