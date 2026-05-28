//! Obsidian vault export/import for the KwaaiNet knowledge graph.
//!
//! Export: writes one markdown file per entity under `entities/<Type>/<Name>.md`
//! with YAML frontmatter and [[wikilinks]] for relations. Produces a
//! `_Knowledge_Graph.md` index.  A `.obsidian/app.json` stub is written so
//! Obsidian opens the vault without prompts.
//!
//! Import: walks the vault for markdown files newer than `last_import_secs`,
//! parses YAML frontmatter and `[[wikilinks]]` relation lines, and writes
//! changed entity descriptions / relations back to the GraphStore.  Changed
//! descriptions are re-embedded so semantic search stays in sync.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};

use crate::embedder::EmbedClient;
use crate::graph::{entity_id, EntityNode, GraphStore};

// ── helpers ───────────────────────────────────────────────────────────────────

fn slug(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}

fn entity_path(out_dir: &Path, entity_type: &str, name: &str) -> PathBuf {
    out_dir
        .join("entities")
        .join(entity_type)
        .join(format!("{}.md", slug(name)))
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ── export ────────────────────────────────────────────────────────────────────

/// Export the graph to an Obsidian-compatible markdown vault.
///
/// Directory layout:
/// ```text
/// <out_dir>/
///   entities/
///     Person/Joe Rassool.md
///     Location/District Six.md
///     …
///   _Knowledge_Graph.md      ← index with entity counts and top nodes
///   .obsidian/app.json       ← minimal stub so Obsidian opens without prompts
/// ```
pub fn export_vault(graph: &GraphStore, out_dir: &Path, kb_name: &str) -> Result<ExportStats> {
    std::fs::create_dir_all(out_dir.join(".obsidian"))?;

    // Minimal Obsidian app.json so the vault opens cleanly
    let app_json = r#"{"legacyEditor":false,"livePreview":true}"#;
    std::fs::write(out_dir.join(".obsidian").join("app.json"), app_json)?;

    let mut stats = ExportStats::default();
    let exported_at = chrono_iso();

    // Build a name→type lookup so relations can resolve target files
    let name_type: HashMap<i64, (String, String)> = graph
        .all_entities()
        .map(|n| (n.id, (n.name.clone(), n.entity_type.clone())))
        .collect();

    // Track every path we write so we can clean up stale files from previous exports
    let mut written_paths: std::collections::HashSet<std::path::PathBuf> =
        std::collections::HashSet::new();

    for node in graph.all_entities() {
        let path = entity_path(out_dir, &node.entity_type, &node.name);
        std::fs::create_dir_all(path.parent().unwrap())?;

        let relations = graph.outgoing_relations(node.id)?;
        let mut rel_lines = String::new();
        for (dst_id, rel_type, strength) in &relations {
            if let Some((dst_name, _)) = name_type.get(dst_id) {
                rel_lines.push_str(&format!(
                    "- **{}** [[{}]]  *(strength: {:.2})*\n",
                    rel_type,
                    slug(dst_name),
                    strength
                ));
            }
        }

        let content = format!(
            "---\n\
             entity_type: {etype}\n\
             mention_count: {mc}\n\
             kb: {kb}\n\
             exported_at: {ts}\n\
             tags: [{etype}, {kb}-kb]\n\
             ---\n\n\
             {desc}\n\n\
             ## Relations\n\n\
             {rels}",
            etype = node.entity_type,
            mc = node.mention_count,
            kb = kb_name,
            ts = exported_at,
            desc = node.description,
            rels = if rel_lines.is_empty() {
                "*(none)*\n".to_string()
            } else {
                rel_lines
            },
        );

        std::fs::write(&path, &content)?;
        written_paths.insert(path.canonicalize().unwrap_or(path));
        stats.entities += 1;
        stats.relations += relations.len();
    }

    // Remove entity .md files from previous exports that are no longer in the graph.
    // This prevents pruned or merged entities from appearing as ghost nodes in Obsidian.
    let entities_dir = out_dir.join("entities");
    if entities_dir.exists() {
        collect_md_files(&entities_dir, &written_paths, &mut stats);
    }

    // Write index
    write_index(graph, out_dir, kb_name, &exported_at, &stats)?;

    Ok(stats)
}

fn write_index(
    graph: &GraphStore,
    out_dir: &Path,
    kb_name: &str,
    exported_at: &str,
    stats: &ExportStats,
) -> Result<()> {
    // Top 20 by mention count
    let mut nodes: Vec<_> = graph.all_entities().collect();
    nodes.sort_by_key(|n| std::cmp::Reverse(n.mention_count));

    let top_list: String = nodes
        .iter()
        .take(20)
        .map(|n| {
            format!(
                "| [[{}\\|{}]] | {} | {} |\n",
                slug(&n.name),
                n.name,
                n.entity_type,
                n.mention_count
            )
        })
        .collect();

    // Entity type counts
    let mut type_counts: HashMap<&str, usize> = HashMap::new();
    for n in graph.all_entities() {
        *type_counts.entry(n.entity_type.as_str()).or_default() += 1;
    }
    let mut type_list: Vec<_> = type_counts.iter().collect();
    type_list.sort_by(|a, b| b.1.cmp(a.1));
    let type_summary: String = type_list
        .iter()
        .map(|(t, c)| format!("- **{}**: {}\n", t, c))
        .collect();

    let index = format!(
        "# Knowledge Graph — {kb}\n\n\
         > Exported {ts}\n\n\
         ## Summary\n\n\
         | Metric | Value |\n\
         |--------|-------|\n\
         | Entities | {entities} |\n\
         | Relations | {relations} |\n\n\
         ## Entity types\n\n\
         {types}\n\
         ## Top entities by mention count\n\n\
         | Entity | Type | Mentions |\n\
         |--------|------|----------|\n\
         {top}\n\
         ## How to use\n\n\
         - Open the **Graph view** (Ctrl/Cmd+G) to browse entity relationships visually.\n\
         - Each entity file under `entities/` contains its description and wikilinked relations.\n\
         - To curate: edit an entity's description or add/remove relation lines, save, \
         then run `kwaainet rag import --from obsidian --input-dir <this-vault>` to sync changes back.\n",
        kb = kb_name,
        ts = exported_at,
        entities = stats.entities,
        relations = stats.relations / 2, // adj is bidirectional
        types = type_summary,
        top = top_list,
    );

    std::fs::write(out_dir.join("_Knowledge_Graph.md"), index)?;
    Ok(())
}

// ── import ────────────────────────────────────────────────────────────────────

/// Import curated changes from an Obsidian vault back into the graph.
///
/// Only files under `entities/` that are newer than `since_secs` are processed.
/// For each changed file:
/// - The YAML `entity_type` is read from frontmatter.
/// - The first non-frontmatter, non-heading paragraph becomes the new description.
/// - `**relation** [[Target]]` lines in the `## Relations` section are parsed and
///   upserted as relations (strength 0.5 for human-added, capped at existing).
/// - If the description changed, the entity is re-embedded.
pub async fn import_vault(
    graph: &mut GraphStore,
    vault_dir: &Path,
    since_secs: u64,
    embed: &EmbedClient,
) -> Result<ImportStats> {
    let entities_dir = vault_dir.join("entities");
    if !entities_dir.exists() {
        anyhow::bail!(
            "no `entities/` directory found in vault at {}",
            vault_dir.display()
        );
    }

    let mut stats = ImportStats::default();

    for entry in walkdir(&entities_dir)? {
        let path = entry;
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        // Skip files not newer than last import
        let mtime = std::fs::metadata(&path)?
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if mtime <= since_secs {
            stats.skipped += 1;
            continue;
        }

        let raw = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;

        let parsed = match parse_entity_file(&raw) {
            Some(p) => p,
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        // Determine entity name from filename (reverse slug — underscores to spaces is fine
        // since we're matching by content hash, not name)
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let display_name = stem.replace('_', " ");

        let eid = entity_id(&display_name, &parsed.entity_type);

        // Check if description changed
        let existing = graph.get_entity(eid).cloned();
        let desc_changed = existing
            .as_ref()
            .map(|e| e.description != parsed.description)
            .unwrap_or(true);

        let embedding = if desc_changed && !parsed.description.is_empty() {
            match embed.embed_one(&parsed.description).await {
                Ok(v) => v,
                Err(_) => existing
                    .as_ref()
                    .map(|e| e.embedding.clone())
                    .unwrap_or_default(),
            }
        } else {
            existing
                .as_ref()
                .map(|e| e.embedding.clone())
                .unwrap_or_default()
        };

        let node = EntityNode {
            id: eid,
            name: existing
                .as_ref()
                .map(|e| e.name.clone())
                .unwrap_or(display_name.clone()),
            entity_type: parsed.entity_type.clone(),
            description: parsed.description.clone(),
            embedding,
            mention_count: existing.as_ref().map(|e| e.mention_count).unwrap_or(1),
            first_chunk_id: existing.as_ref().map(|e| e.first_chunk_id).unwrap_or(0),
            aliases: existing
                .as_ref()
                .map(|e| e.aliases.clone())
                .unwrap_or_default(),
            schema_type: existing.as_ref().and_then(|e| e.schema_type.clone()),
            evidence: Vec::new(),
            gender: existing.as_ref().and_then(|e| e.gender.clone()),
            fields: existing
                .as_ref()
                .map(|e| e.fields.clone())
                .unwrap_or_default(),
        };

        graph.upsert_entity(node)?;

        // Upsert human-edited relations
        for (rel_type, target_name) in &parsed.relations {
            let target_id = entity_id(target_name, "Unknown");
            // Only upsert if target exists in graph (avoid polluting with dead links)
            if graph.get_entity(target_id).is_some() || graph.find_by_name(target_name).is_some() {
                let resolved_target = graph
                    .find_by_name(target_name)
                    .map(|n| n.id)
                    .unwrap_or(target_id);
                graph.upsert_relation(eid, resolved_target, rel_type, 0)?;
                stats.relations_updated += 1;
            }
        }

        if desc_changed {
            stats.descriptions_updated += 1;
        }
        stats.entities_processed += 1;
    }

    Ok(stats)
}

// ── parsing ───────────────────────────────────────────────────────────────────

struct ParsedEntity {
    entity_type: String,
    description: String,
    /// (relation_type, target_name)
    relations: Vec<(String, String)>,
}

fn parse_entity_file(raw: &str) -> Option<ParsedEntity> {
    // Split frontmatter
    let after_front = if let Some(stripped) = raw.strip_prefix("---") {
        let end = stripped.find("\n---").map(|i| i + 3 + 4)?;
        &raw[end..]
    } else {
        raw
    };

    // entity_type from frontmatter
    let entity_type = raw
        .lines()
        .find(|l| l.starts_with("entity_type:"))
        .and_then(|l| l.split_once(':'))
        .map(|(_, v)| v.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    // First non-empty, non-heading paragraph before `## Relations`
    let body_before_relations = after_front
        .split("\n## Relations")
        .next()
        .unwrap_or(after_front);

    let description: String = body_before_relations
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");

    // Relations section: lines matching `- **rel_type** [[Target Name]]`
    let relations_section = after_front.split("\n## Relations").nth(1).unwrap_or("");

    let mut relations = Vec::new();
    for line in relations_section.lines() {
        let line = line.trim();
        if !line.starts_with("- **") {
            continue;
        }
        // parse: - **rel_type** [[Target Name]]  *(strength: 0.xx)*
        if let Some(rel_end) = line[4..].find("**") {
            let rel_type = line[4..4 + rel_end].to_string();
            if let (Some(link_start), Some(link_end)) = (line.find("[["), line.find("]]")) {
                let target = line[link_start + 2..link_end].to_string();
                // strip Obsidian display alias: [[File|Display]] → File
                let target = target
                    .split('|')
                    .next()
                    .unwrap_or(&target)
                    .trim()
                    .to_string();
                // convert slug back (underscores → spaces)
                let target = target.replace('_', " ");
                relations.push((rel_type, target));
            }
        }
    }

    Some(ParsedEntity {
        entity_type,
        description,
        relations,
    })
}

// ── small helpers ─────────────────────────────────────────────────────────────

fn chrono_iso() -> String {
    // Use Unix epoch + format without chrono dep
    let secs = now_secs();
    let days = secs / 86400;
    let rem = secs % 86400;
    let h = rem / 3600;
    let m = (rem % 3600) / 60;
    let s = rem % 60;
    // Rough Gregorian date
    let (y, mo, d) = days_to_ymd(days);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

fn days_to_ymd(mut days: u64) -> (u64, u64, u64) {
    let mut y = 1970u64;
    loop {
        let leap = (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400);
        let dy = if leap { 366 } else { 365 };
        if days < dy {
            break;
        }
        days -= dy;
        y += 1;
    }
    let leap = (y.is_multiple_of(4) && !y.is_multiple_of(100)) || y.is_multiple_of(400);
    let months = [
        31u64,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut mo = 1u64;
    for &ml in &months {
        if days < ml {
            break;
        }
        days -= ml;
        mo += 1;
    }
    (y, mo, days + 1)
}

fn walkdir(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !dir.exists() {
        return Ok(out);
    }
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let p = entry.path();
        if p.is_dir() {
            out.extend(walkdir(&p)?);
        } else {
            out.push(p);
        }
    }
    Ok(out)
}

// ── public stats types ────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct ExportStats {
    pub entities: usize,
    pub relations: usize,
    pub stale_removed: usize,
}

#[derive(Debug, Default)]
pub struct ImportStats {
    pub entities_processed: usize,
    pub descriptions_updated: usize,
    pub relations_updated: usize,
    pub skipped: usize,
}

/// Recursively walk `dir`, delete any `.md` file not in `keep`, and increment `stats.stale_removed`.
fn collect_md_files(
    dir: &std::path::Path,
    keep: &std::collections::HashSet<std::path::PathBuf>,
    stats: &mut ExportStats,
) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(&path, keep, stats);
        } else if path.extension().and_then(|x| x.to_str()) == Some("md") {
            let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
            if !keep.contains(&canon) {
                let _ = std::fs::remove_file(&path);
                stats.stale_removed += 1;
            }
        }
    }
}
