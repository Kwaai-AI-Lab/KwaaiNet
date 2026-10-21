# The predicate as a vector — research avenue

2026-10-20. Proposed by Reza. **Status: research avenue, not planned work.**
Nothing here is scheduled or implemented. It records a direction, the
evidence that motivates it, the objection it has to answer, and the smallest
experiment that would tell us whether to pursue it.

Builds on [`PredicateVectors-exploration.md`](./PredicateVectors-exploration.md)
(2026-08-26) and the predicate-collapse finding in
[`OntologySession-assessment.md`](./OntologySession-assessment.md) §4d.

---

## 1. The problem: a triple forces a predicate to be a symbol

The graph stores `(subject, predicate, object)`. Subject and object are
entities, and entities have *identity*: they can be enumerated, deduplicated,
merged and coreferenced, because two mentions either are the same thing or
are not. The predicate is stored the same way, as a name from a closed list,
but a predicate does not have identity. It has *meaning*, and meaning is
graded. `nursed_by` and `cared_for` are not one edge under two names. They
are neighbouring points.

A closed predicate list can respond to a predicate outside it in only two
ways, and both lose:

- **Reject it.** The edge is gone.
- **Coerce it** to a declared predicate or to the fallback. The edge survives,
  but its meaning has been replaced by a neighbour's.

This is **predicate collapse**. The case for enforcing a closed list came
first: in the first D6 ontology A/B, when nothing checked the output, 88
distinct predicates were extracted against 27 declared (the doc comment on
`Ontology::coerce_relation`). Enforcement fixed that, and we have since seen
the cost in two experiments:

| Where | What happened |
|---|---|
| D6 ontology A/B (`OntologySession-assessment.md` §4d) | Undeclared predicates fell from 17.0% to 2.4% with enforcement (run 3). The per-edge breakdown shows what those undeclared edges were: the control's 87 were almost all *nuance*, undeclared predicates carrying real meaning, against 0 junk. The assessment's reading is that enforcement removes meaning and noise together. |
| NIST 1% forgetting stage (2026-10-20; `DreamRAG-Forgetting-plan.md`, on branch `feat/dream-forgetting`, not yet on main) | 18 declared predicates, 11 never used. About 13 distinct facts over about 85 entities after 16 dream cycles, 9 of them `published_by`. A 47-chunk slice of document openings, so a weak signal, but the shape is the same: the declared vocabulary is barely reached. |

What is in place today is a mitigation, not a fix:
`RelationRecord.original_predicate` keeps what the extractor emitted when
coercion rewrites it, and `GraphStore::coercion_residue()` lists those
predicates. The nuance is recoverable but not usable: traversal, retrieval
and axioms still see only the coerced symbol.

## 2. The proposal

**Keep entities symbolic and enumerated. Store each predicate as a vector, and
look predicates up by similarity, not by name.**

An edge becomes:

```
(subject_id, object_id,
 predicate_text,      -- what the extractor said, verbatim
 predicate_vector,    -- an embedding of it (unit to embed: see §4, Q2)
 evidence)
```

- **Writing an edge never coerces.** Whatever the extractor said is kept, in
  words and as a position in the space.
- **Reading an edge is a similarity search.** "Who looked after him?" embeds
  the relational intent and finds `nursed_by`, `cared_for` and
  `foster_child_of` edges by distance, not by string match.
- **The ontology's declared predicates become named regions** of the space
  (prototypes or centroids), not gates. They remain the vocabulary for
  traversal, display and axioms, reached by projecting an edge onto its
  nearest prototype with a confidence. An edge that sits far from every
  prototype is still in the graph and still retrievable; it simply carries no
  axiom.

Entities are enumerable and predicates are searchable: the representation
follows the difference between identity and meaning.

## 3. The objection it has to answer: polarity

The exploration found the disqualifying fact (§2 there): the one embedder
measured, `nomic-embed-text`, did not separate polarity. Whether any
embedder does is open (§4, Q1).

```
supported vs opposed    0.694   (above the 0.624 median)
parent_of vs child_of   0.758
```

That result ruled out **write-time coercion by nearest neighbour**, and it
still does. If "supported" is coerced to its nearest neighbour `opposed`, the
graph *asserts* a falsehood.

This proposal moves the error to a different place, where it costs less:

| | Write-time coercion (ruled out) | Predicate as vector (proposed) |
|---|---|---|
| What is stored | the neighbour's name | the extractor's own words, plus a vector |
| A polarity error becomes | a stored false assertion, served with confidence | a ranking error at query time: an `opposed` query also retrieves a `supported` edge |
| Who can catch it | nobody downstream, because the original words are gone | the generator, which reads the edge's own text; or an NLI check of edge text against the query |

So the polarity failure does not disqualify this design, but it does bound
it:

- **Direction stays symbolic.** Subject and object order is structural, and
  the ontology already declares `inverse:` and contradiction axioms. The
  vector should carry *what kind* of relation an edge is, never which way
  round it points.
- **Axioms fire only on confident prototype membership.** `functional`,
  `transitive`, `inverse` and contradiction checks need a symbol, so an edge
  takes part in them only when it projects onto a declared predicate above a
  threshold. Uncertain edges are retrievable but inert.
- **Polarity-sensitive queries need a check after retrieval**, not trust in
  the ranking.

## 4. Research questions, in order

1. **Is there an embedder that separates polarity?** This is the gate for
   everything else. Candidates, cheapest first:
   - NLI-trained sentence-transformers, which separate contradiction from
     entailment by construction;
   - instruction-tuned embedders given a relation-focused instruction;
   - only if both fail, a fine-tuned or purpose-built relation embedder (the
     "our own embedder" question from 2026-08-26).

   The predicate set already exists: D6's 59 rejected and 43 declared
   predicates, with nearest-neighbour data in
   `tests/kwaai-knowledge/results/predicate_nn.json`. Its existing labels are
   not a polarity benchmark, though. The antonym and inverse pairs have to be
   built, from every curated ontology and not only D6's; the ontologies'
   `inverse:` declarations are a ready source of the inverse half.
2. **What unit should be embedded?** There are three options: the bare label
   (`supported`); the label in a template (`"X supported Y"`, which gives the
   embedder a relational frame); or the evidence span.
   - The exploration's evidence-vector run (§6 there) was null, but on too
     small a sample to settle anything.
   - Span recovery reached only 21 of 340 D6 edges, so the mention index
     ([`Phase6-PersistentMentionIndex-plan.md`](./Phase6-PersistentMentionIndex-plan.md))
     is a prerequisite for the span option.
3. **Does similarity lookup improve answers?** This is the question that
   matters for the product. The graph-only eval (`rag eval --mode
   graph-only`, used by Eval v2 and the forgetting experiment) answers from
   entity cards alone, which makes it the natural testbed. Compare recall
   with exact-predicate edges against similarity-matched edges, with the
   empty-graph floor as the reference.
4. **What does the ontology become?** It currently governs write-time
   acceptance. Here it would supply prototypes for projection and the
   symbolic layer for axioms. Open:
   - how prototypes are defined: from the declared description and examples,
     or from the centroid of confidently matched edges;
   - how the membership threshold is calibrated per KB.
5. **When are two edges the same?** Two edges with the same endpoints and
   near-identical predicate vectors are probably one fact, said twice. Edge
   deduplication then depends on the same polarity-safe similarity as Q1.
6. **Cost.** One vector per edge: at 768 dimensions that is about 3 KB per
   edge, small next to the chunk embeddings already stored, since a KB has
   far fewer edges than chunks (D6's production graph had 340 relations over
   1152 chunks). Brute-force search is adequate at that scale, so no new
   index structure is needed to find out whether the idea works.

## 5. The smallest first experiment

Offline, read-only, no change to the graph or the extraction code:

1. Collect the predicates of existing graphs: declared edges plus the
   coercion residue (`original_predicate`), across D6 (local only) and the
   four public KBs with curated ontologies in `tests/kwaai-knowledge/ontologies/`:
   Manhattan_v3 (`Manhattan.yaml`), Legal, NIST and Astrophysics.
2. Embed each in two units (bare label and `"X <predicate> Y"` template) with
   two or three candidate embedders.
3. Measure, for each pair of embedder and unit:
   - **polarity separation** on the labelled antonym and inverse pairs (the
     gate);
   - **alias recovery** against the hand-authored alias tables (the
     exploration's baseline: 11 of 38 at ≥ 0.80);
   - **residue recovery**: the share of collapsed nuance edges whose nearest
     declared prototype is the right one, by hand label.

If no candidate separates polarity, stop: the exploration's three roles for
vectors (alias proposal, induction from residue, retrieval matching) remain
the ceiling. If one does, the next step is Q3: a graph-only eval comparing
symbolic and vector predicate lookup on one public KB.

## 6. Out of scope

- **Write-time coercion by nearest neighbour** stays ruled out, whatever the
  embedder, unless the polarity gate in Q1 is passed with margin.
- **Replacing entity identity with vectors.** Entities are the enumerable
  half of the graph, and dedup and coreference depend on that.
- **Changing production extraction** before the offline experiment has a
  result.
