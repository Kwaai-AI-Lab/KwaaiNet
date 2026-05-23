# NotebookLM Ground-Truth Extraction Prompt

Paste this prompt into NotebookLM after uploading your source document.
The output JSON is directly importable via:

```
kwaainet rag graph seed-from-json --file output.json --kb <KB>
```

To also emit a seed YAML for manual review:

```
kwaainet rag graph seed-from-json --file output.json --kb <KB> --emit-yaml seed.yaml
```

---

## Prompt (paste into NotebookLM)

You are extracting structured ground-truth knowledge from the uploaded document for a knowledge graph.

Return ONLY valid JSON matching this exact schema — no markdown, no explanation:

```json
{
  "document": {
    "title": "<exact document title>",
    "author": "<author name or null>",
    "period": "<time period covered, e.g. '1880–1960' or null>",
    "genre": "<memoir | biography | history | fiction | other>"
  },
  "entities": [
    {
      "canonical": "<most complete, unambiguous name>",
      "type": "<Person | Organization | Location | Event | Concept>",
      "aliases": ["<nickname>", "<abbreviation>", "<alternative spelling>"],
      "gender": "<Male | Female | null>",
      "description": "<2-4 sentences: who/what they are, their role, key facts, significance>",
      "birth_year": "<year as number or null>",
      "death_year": "<year as number or null>",
      "confidence": "<high | medium | low>"
    }
  ],
  "relations": [
    {
      "from": "<canonical name of source entity>",
      "to": "<canonical name of target entity>",
      "type": "<relation type from list below>",
      "evidence": "<brief quote or paraphrase from the text supporting this relation>",
      "confidence": "<high | medium | low>"
    }
  ]
}
```

VALID RELATION TYPES (use only these):
parent_of, child_of, spouse_of, sibling_of, half_sibling_of, grandparent_of, grandchild_of,
uncle_of, aunt_of, niece_of, nephew_of, cousin_of, foster_parent_of, foster_child_of,
works_at, founded, manages, belongs_to, endorses, part_of, contains, located_in,
related_to, associated_with, caused_by, followed_by, precedes

EXTRACTION RULES:
1. Use the most complete, unambiguous name as "canonical" (e.g. "Peter Alexander Rassool" not "Peter").
2. List every nickname, abbreviation, and alternate spelling as aliases.
3. For Person entities: infer gender from pronouns in the text.
4. For family relations: be precise about direction. "parent_of" means FROM=parent, TO=child.
   If you are unsure about full vs half sibling, use "half_sibling_of" — it will be corrected later.
5. "spouse_of" requires EXPLICIT marriage evidence ("married", "wife of", "husband of").
   Do NOT use spouse_of for associates, colleagues, or companions.
6. Use "confidence: low" for entities mentioned only once or relations inferred without clear evidence.
7. Do NOT include the document title itself as a Location entity.
   The document title is a CreativeWork — exclude it from relation targets.
8. Descriptions must NOT start with the entity's own name.
9. Include ALL named persons, regardless of how minor their role.
10. For organisations: include political parties, schools, clubs, businesses, government bodies.

OUTPUT: Return only the JSON object. No preamble, no explanation, no markdown fences.

---

## Output format notes

- `birth_year` and `death_year` must be integers (e.g. `1899`), not strings.
- `aliases` must be a JSON array, even if empty: `[]`.
- `gender` must be `"Male"`, `"Female"`, or `null` — never a string like `"unknown"`.
- `confidence` must be one of: `"high"`, `"medium"`, `"low"`.
- Relation `from` and `to` must exactly match a `canonical` value in the `entities` array.

## What happens with confidence levels

| confidence | entities | relations |
|------------|----------|-----------|
| high       | included | included  |
| medium     | included | included  |
| low        | included | **skipped** |

Low-confidence entities are always seeded (the name is known); only their relations are dropped
to avoid polluting the graph with uncertain edges.
