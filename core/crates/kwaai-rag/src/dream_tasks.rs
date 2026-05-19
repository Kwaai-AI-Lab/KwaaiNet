use crate::dream::{complete_entity, EntityCompletion};

/// Which completion work an entity needs, derived from whether its schema.org
/// type is already resolved.
#[derive(Debug)]
pub enum TaskKind {
    /// schema_type is None — ask the LLM to infer type + description + relations.
    Full,
    /// schema_type is already set — focus on description enrichment and relation extraction.
    SummaryAndRelations { schema_type: String },
}

pub fn task_for_schema_type(schema_type: Option<&str>) -> TaskKind {
    match schema_type {
        None => TaskKind::Full,
        Some(st) => TaskKind::SummaryAndRelations {
            schema_type: st.to_string(),
        },
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn run_task(
    kind: TaskKind,
    entity_id: i64,
    name: &str,
    entity_type: &str,
    description: &str,
    evidence_text: &str,
    url: &str,
    model: &str,
) -> EntityCompletion {
    let current_type = match &kind {
        TaskKind::Full => entity_type,
        TaskKind::SummaryAndRelations { schema_type } => schema_type.as_str(),
    };
    complete_entity(entity_id, name, current_type, description, evidence_text, url, model).await
}
