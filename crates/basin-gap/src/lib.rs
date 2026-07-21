use std::collections::BTreeSet;

use basin_corpus::{CorpusEntry, EvidenceLabel, Scale};
use basin_score::{Dimension, Rubric, Score, ScoreError};
use basin_tier::{tier_sla_gap, TierError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const UNDER_SERVED_THRESHOLD: f64 = 7.0;

/// Share of scored entries below threshold that turns a tail gap systemic.
const SYSTEMIC_SHARE: f64 = 0.5;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GapRegion {
    pub id: String,
    pub scale: Scale,
    pub dimension: Dimension,
    pub threshold: Score,
    pub observed: Option<Score>,
    pub entry_ids: Vec<String>,
    pub reason: String,
    pub source: GapSource,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapSource {
    EmptyRegion,
    UnderServedRegion,
    TailRegion,
    SystemicRegion,
    TierSla,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NullResult {
    pub region_id: String,
    pub scale: Scale,
    pub rubric_version: String,
    pub evidence_label: EvidenceLabel,
    pub rationale: String,
    pub inspected_entries: usize,
}

/// Find gaps within a single declared `scale`. Elements of other scales are
/// excluded unless `cross_scale` is set (REQ-016: cross-scale comparisons need
/// an explicit marker).
pub fn find_gaps(
    corpus: &[CorpusEntry],
    rubric: &Rubric,
    scale: Scale,
    cross_scale: bool,
) -> Result<Vec<GapRegion>, GapError> {
    let in_scope = entries_in_scope(corpus, scale, cross_scale);
    let mut gaps = Vec::new();

    for dimension in Dimension::ALL {
        let threshold = threshold_for(rubric, dimension)?;
        let scored = scored_entries(&in_scope, dimension)?;
        if scored.is_empty() {
            gaps.push(GapRegion {
                id: format!("empty-{}", dimension.key()),
                scale,
                dimension,
                threshold,
                observed: None,
                entry_ids: Vec::new(),
                reason: format!("no corpus entries scored for {}", dimension.key()),
                source: GapSource::EmptyRegion,
            });
            continue;
        }

        if let Some(gap) = under_served_gap(scale, dimension, threshold, &scored) {
            gaps.push(gap);
        }
        if let Some(gap) = tail_gap(scale, dimension, threshold, &scored) {
            gaps.push(gap);
        }
    }

    gaps.extend(tier_sla_gap_regions(&in_scope, scale)?);
    gaps.sort_by(|left, right| left.id.cmp(&right.id));
    gaps.dedup_by(|left, right| left.id == right.id);
    Ok(gaps)
}

pub fn null_result(
    region_id: impl Into<String>,
    corpus: &[CorpusEntry],
    rubric: &Rubric,
    scale: Scale,
    cross_scale: bool,
) -> Result<Option<NullResult>, GapError> {
    if !find_gaps(corpus, rubric, scale, cross_scale)?.is_empty() {
        return Ok(None);
    }

    Ok(Some(NullResult {
        region_id: region_id.into(),
        scale,
        rubric_version: rubric.version.clone(),
        evidence_label: EvidenceLabel::Implemented,
        rationale: "No gap manufactured: all rubric dimensions are scored at or above threshold and no tier-SLA shortfall is present at this scale.".to_string(),
        inspected_entries: entries_in_scope(corpus, scale, cross_scale).len(),
    }))
}

fn entries_in_scope(corpus: &[CorpusEntry], scale: Scale, cross_scale: bool) -> Vec<&CorpusEntry> {
    corpus
        .iter()
        .filter(|entry| cross_scale || entry.scale == Some(scale))
        .collect()
}

fn threshold_for(rubric: &Rubric, dimension: Dimension) -> Result<Score, GapError> {
    let weight = rubric.weight(dimension).unwrap_or(1.0);
    let threshold = (UNDER_SERVED_THRESHOLD * weight.max(0.0)).clamp(0.0, 10.0);
    Ok(Score::new(threshold)?)
}

fn under_served_gap(
    scale: Scale,
    dimension: Dimension,
    threshold: Score,
    scored: &[(String, Score)],
) -> Option<GapRegion> {
    let lowest = scored
        .iter()
        .min_by(|left, right| left.1.value().total_cmp(&right.1.value()))?;
    if lowest.1.value() >= threshold.value() {
        return None;
    }
    Some(GapRegion {
        id: format!("under-served-{}", dimension.key()),
        scale,
        dimension,
        threshold,
        observed: Some(lowest.1),
        entry_ids: scored.iter().map(|(id, _)| id.clone()).collect(),
        reason: format!("lowest {} score is below threshold", dimension.key()),
        source: GapSource::UnderServedRegion,
    })
}

fn tail_gap(
    scale: Scale,
    dimension: Dimension,
    threshold: Score,
    scored: &[(String, Score)],
) -> Option<GapRegion> {
    if scored.is_empty() {
        return None;
    }
    let mut sorted = scored.to_vec();
    sorted.sort_by(|left, right| left.1.value().total_cmp(&right.1.value()));
    let tail_len = sorted.len().div_ceil(4).max(1);
    let tail_sum: f64 = sorted
        .iter()
        .take(tail_len)
        .map(|(_, score)| score.value())
        .sum();
    let tail_mean = tail_sum / tail_len as f64;
    let below: Vec<String> = scored
        .iter()
        .filter(|(_, score)| score.value() < threshold.value())
        .map(|(id, _)| id.clone())
        .collect();
    if tail_mean >= threshold.value() || below.is_empty() {
        return None;
    }

    let below_count = below.len();
    let share = below_count as f64 / scored.len() as f64;
    let systemic = share >= SYSTEMIC_SHARE;
    let (id, source, kind) = if systemic {
        (
            format!("systemic-{}", dimension.key()),
            GapSource::SystemicRegion,
            "a systemic deficit (most of the class is below the bar)",
        )
    } else {
        (
            format!("tail-{}", dimension.key()),
            GapSource::TailRegion,
            "a concentrated tail (target the named minority)",
        )
    };

    Some(GapRegion {
        id,
        scale,
        dimension,
        threshold,
        observed: Some(Score::new(tail_mean.clamp(0.0, 10.0)).ok()?),
        entry_ids: below,
        reason: format!(
            "bottom-quartile {} mean is below threshold; {} of {} scored entries ({:.0}%) fall below the bar, classified as {}",
            dimension.key(),
            below_count,
            scored.len(),
            share * 100.0,
            kind
        ),
        source,
    })
}

fn scored_entries(
    corpus: &[&CorpusEntry],
    dimension: Dimension,
) -> Result<Vec<(String, Score)>, GapError> {
    let mut scored = Vec::new();
    for entry in corpus {
        let Some(raw_score) = entry.scores.get(dimension.key()).copied() else {
            continue;
        };
        let id = entry.id.clone().ok_or(GapError::MissingEntryId)?;
        scored.push((id, Score::new(raw_score)?));
    }
    Ok(scored)
}

fn tier_sla_gap_regions(corpus: &[&CorpusEntry], scale: Scale) -> Result<Vec<GapRegion>, GapError> {
    let mut regions = Vec::new();
    let mut seen = BTreeSet::new();
    for entry in corpus {
        let Some(gap) = tier_sla_gap(entry)? else {
            continue;
        };
        if !seen.insert(gap.entry_id.clone()) {
            continue;
        }
        regions.push(GapRegion {
            id: format!("tier-sla-{}", gap.entry_id),
            scale,
            dimension: gap.dimension,
            threshold: Score::new(10.0)?,
            observed: Some(gap.score),
            entry_ids: vec![gap.entry_id],
            reason: gap.reason,
            source: GapSource::TierSla,
        });
    }
    Ok(regions)
}

#[derive(Debug, Error, PartialEq)]
pub enum GapError {
    #[error("gap analysis requires scored entries to have stable ids")]
    MissingEntryId,
    #[error(transparent)]
    Score(#[from] ScoreError),
    #[error(transparent)]
    Tier(#[from] TierError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fully_scored_entry(id: &str, scale: Scale, score: f64) -> CorpusEntry {
        let mut entry = CorpusEntry {
            id: Some(id.to_string()),
            scale: Some(scale),
            tier: Some("T1".to_string()),
            ..CorpusEntry::default()
        };
        for dimension in Dimension::ALL {
            entry.scores.insert(dimension.key().to_string(), score);
        }
        entry
    }

    #[test]
    fn find_gaps_reports_under_served_dimension_at_scale() {
        let mut entry = fully_scored_entry("system-1", Scale::Regional, 9.0);
        entry.scores.insert("DIM-05".to_string(), 4.5);

        let gaps = find_gaps(&[entry], &Rubric::default_v0(), Scale::Regional, false)
            .expect("gap analysis succeeds");

        assert!(gaps.iter().any(|gap| {
            gap.id == "under-served-DIM-05"
                && gap.scale == Scale::Regional
                && gap.dimension == Dimension::Dim05
                && gap.observed.expect("observed score").value() == 4.5
                && gap.source == GapSource::UnderServedRegion
        }));
    }

    #[test]
    fn other_scale_entries_are_excluded_without_cross_scale_marker() {
        let mut regional = fully_scored_entry("regional-1", Scale::Regional, 9.0);
        regional.scores.insert("DIM-05".to_string(), 4.5);
        let local = fully_scored_entry("local-1", Scale::Local, 2.0);

        let gaps = find_gaps(
            &[regional, local],
            &Rubric::default_v0(),
            Scale::Regional,
            false,
        )
        .expect("gap analysis succeeds");

        assert!(gaps
            .iter()
            .all(|gap| !gap.entry_ids.iter().any(|id| id == "local-1")));

        let cross = find_gaps(
            &[
                fully_scored_entry("regional-2", Scale::Regional, 9.0),
                fully_scored_entry("local-2", Scale::Local, 2.0),
            ],
            &Rubric::default_v0(),
            Scale::Regional,
            true,
        )
        .expect("cross-scale gap analysis succeeds");

        assert!(cross
            .iter()
            .any(|gap| gap.entry_ids.iter().any(|id| id == "local-2")));
    }

    #[test]
    fn find_gaps_reports_empty_dimension_without_manufacturing_score() {
        let mut entry = fully_scored_entry("system-1", Scale::Regional, 9.0);
        entry.scores.remove("DIM-08");

        let gaps = find_gaps(&[entry], &Rubric::default_v0(), Scale::Regional, false)
            .expect("gap analysis succeeds");
        let gap = gaps
            .iter()
            .find(|gap| gap.id == "empty-DIM-08")
            .expect("empty dimension gap");

        assert_eq!(gap.observed, None);
        assert_eq!(gap.source, GapSource::EmptyRegion);
    }

    #[test]
    fn null_result_records_adequate_system_without_manufactured_gap() {
        let entry = fully_scored_entry("system-1", Scale::Regional, 10.0);

        let result = null_result(
            "adequate-system",
            &[entry],
            &Rubric::default_v0(),
            Scale::Regional,
            false,
        )
        .expect("null-result evaluation succeeds")
        .expect("no gaps yields null result");

        assert_eq!(result.region_id, "adequate-system");
        assert_eq!(result.scale, Scale::Regional);
        assert_eq!(result.evidence_label, EvidenceLabel::Implemented);
        assert!(result.rationale.contains("No gap manufactured"));
    }

    #[test]
    fn null_result_is_absent_when_gap_exists() {
        let mut entry = fully_scored_entry("system-1", Scale::Regional, 10.0);
        entry.scores.insert("DIM-01".to_string(), 3.0);

        assert_eq!(
            null_result(
                "not-null",
                &[entry],
                &Rubric::default_v0(),
                Scale::Regional,
                false
            ),
            Ok(None)
        );
    }

    #[test]
    fn tier_sla_gap_is_integrated_as_gap_region() {
        let mut entry = fully_scored_entry("system-1", Scale::Regional, 10.0);
        entry.scores.insert("DIM-13".to_string(), 6.0);

        let gaps = find_gaps(&[entry], &Rubric::default_v0(), Scale::Regional, false)
            .expect("gap analysis succeeds");

        assert!(gaps.iter().any(|gap| {
            gap.id == "tier-sla-system-1"
                && gap.dimension == Dimension::Dim13
                && gap.source == GapSource::TierSla
        }));
    }

    fn corpus_with_dim01(scores: &[f64]) -> Vec<CorpusEntry> {
        scores
            .iter()
            .enumerate()
            .map(|(idx, value)| {
                let mut entry = fully_scored_entry(&format!("system-{idx}"), Scale::Regional, 10.0);
                entry.scores.insert("DIM-01".to_string(), *value);
                entry
            })
            .collect()
    }

    #[test]
    fn tail_gap_isolates_concentrated_deficit_to_named_minority() {
        let corpus = corpus_with_dim01(&[10.0, 10.0, 7.0, 7.0, 7.0, 7.0, 4.0, 2.0]);
        let gaps = find_gaps(&corpus, &Rubric::default_v0(), Scale::Regional, false)
            .expect("gap analysis runs");
        let tail = gaps
            .iter()
            .find(|gap| gap.id == "tail-DIM-01")
            .expect("tail region present");

        assert_eq!(tail.source, GapSource::TailRegion);
        assert_eq!(tail.entry_ids.len(), 2);
        assert!(tail.entry_ids.contains(&"system-6".to_string()));
        assert!(tail.entry_ids.contains(&"system-7".to_string()));

        let under = gaps
            .iter()
            .find(|gap| gap.id == "under-served-DIM-01")
            .expect("min region present");
        assert_eq!(under.entry_ids.len(), corpus.len());
    }

    #[test]
    fn adequate_distribution_has_no_tail_gap() {
        let corpus = corpus_with_dim01(&[8.0, 8.0, 9.0, 10.0]);
        let gaps = find_gaps(&corpus, &Rubric::default_v0(), Scale::Regional, false)
            .expect("gap analysis runs");

        assert!(!gaps.iter().any(|gap| gap.source == GapSource::TailRegion));
    }

    #[test]
    fn majority_deficit_is_classified_systemic_not_tail() {
        // 4 of 6 entries below the 7.0 bar (67%): the bottom-quartile trigger
        // still fires, but the share crosses SYSTEMIC_SHARE so it is reported as
        // a systemic deficit rather than a concentrated tail.
        let corpus = corpus_with_dim01(&[10.0, 9.0, 2.0, 2.0, 1.0, 1.0]);
        let gaps = find_gaps(&corpus, &Rubric::default_v0(), Scale::Regional, false)
            .expect("gap analysis runs");

        assert!(
            !gaps.iter().any(|gap| gap.source == GapSource::TailRegion),
            "a majority deficit must not be labelled a tail"
        );
        let systemic = gaps
            .iter()
            .find(|gap| gap.id == "systemic-DIM-01")
            .expect("systemic region present");
        assert_eq!(systemic.source, GapSource::SystemicRegion);
        assert_eq!(systemic.entry_ids.len(), 4);
        assert!(systemic.entry_ids.contains(&"system-2".to_string()));
        assert!(systemic.entry_ids.contains(&"system-3".to_string()));
        assert!(systemic.entry_ids.contains(&"system-4".to_string()));
        assert!(systemic.entry_ids.contains(&"system-5".to_string()));
        assert!(systemic.reason.contains("67%"));
    }
}
