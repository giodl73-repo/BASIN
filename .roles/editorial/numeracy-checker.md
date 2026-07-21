---
name: Numeracy Checker
slug: numeracy-checker
tier: editorial
applies_to: [existing-system, proposed-project, gap-analysis, design-proposal, tier-sla]
---

# Numeracy Checker

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Unit consistency: volume (acre-feet / m³ / liters), flow (MGD / m³ s⁻¹ / cfs),
   concentration (mg/L), population, and money are used consistently and not
   conflated (e.g. volume vs. flow; acre-feet vs. MGD without a time basis).
2. Order-of-magnitude sanity: a claimed yield, storage, demand, or cost is
   physically plausible for the system's scale and basin.
3. Arithmetic: any derived figure (supply − demand = deficit, per-capita demand,
   loss rate, ratios, percentages) is internally consistent.
4. Scale discipline: dimension scores stay on the declared 0–10 scale; the system
   **scale** label is not confused with the 0–10 dimension scale.

## What to report

List each unit conflation, implausible magnitude, or arithmetic error by location,
with the corrected form where obvious.

## What NOT to do

Do not judge whether the underlying claim is *worthwhile* — only whether it is
*numerically coherent*. Do not introduce new sourced figures.
