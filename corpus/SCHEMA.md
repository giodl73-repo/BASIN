# Corpus Schema

Each corpus entry is a markdown file with YAML-like frontmatter and dimension
blocks. Stable identity is mandatory: `id` is the join key, while labels, basin,
and map ids are presentation or grouping fields. Every entry must also declare a
`scale` so that scores, tiers, and gaps are interpreted within scale (CON-007).

## Frontmatter

```yaml
---
id: reservoir:shasta
type: reservoir
scale: regional
basin: sacramento
termini: [shasta]
tier: T1
sla: supply-critical
---
```

Required keys:

| Key | Rule |
|---|---|
| `id` | Stable source, node, or conveyance id. Missing ids reject the entry. |
| `type` | Element class such as `reservoir`, `node`, or `conveyance`. |
| `scale` | One of `international`, `national`, `regional`, `local`. Missing scale holds the entry (CON-007). |

Optional keys:

| Key | Rule |
|---|---|
| `basin` | Watershed or jurisdiction label; not an identity key. |
| `termini` | Stable endpoint node ids for conveyances. |
| `tier` | T1-T4 classification, populated by WP-004. |
| `sla` | Declared SLA basis, populated by WP-004. |

## Quantity lines

Quantity lines use the body form:

```text
quantity: 4500000 | AF | source-needed | -
```

Columns are `value | unit | evidence-label | source-id`. A quantity with no
`source-id` and no `evidence-label` is held, not promoted. Supported labels are:
`implemented`, `heuristic`, `simulated`, `proxy`, `planned`, `held`,
`source-needed`, and `confidence-limited`.
