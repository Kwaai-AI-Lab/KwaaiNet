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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocSchema {
    /// The exact title of the source document — used to prevent treating it as a location entity.
    pub document_title: Option<String>,
    /// Name of the default narrator / first-person voice in the main body.
    pub default_narrator: Option<String>,
    /// Ordered list of section definitions; first pattern to match wins.
    #[serde(default)]
    pub sections: Vec<SectionDef>,
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
