use std::collections::BTreeMap;

use basin_corpus::CorpusEntry;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Dimension {
    Dim01,
    Dim02,
    Dim03,
    Dim04,
    Dim05,
    Dim06,
    Dim07,
    Dim08,
    Dim09,
    Dim10,
    Dim11,
    Dim12,
    Dim13,
}

impl Dimension {
    pub const ALL: [Self; 13] = [
        Self::Dim01,
        Self::Dim02,
        Self::Dim03,
        Self::Dim04,
        Self::Dim05,
        Self::Dim06,
        Self::Dim07,
        Self::Dim08,
        Self::Dim09,
        Self::Dim10,
        Self::Dim11,
        Self::Dim12,
        Self::Dim13,
    ];

    pub fn key(self) -> &'static str {
        match self {
            Self::Dim01 => "DIM-01",
            Self::Dim02 => "DIM-02",
            Self::Dim03 => "DIM-03",
            Self::Dim04 => "DIM-04",
            Self::Dim05 => "DIM-05",
            Self::Dim06 => "DIM-06",
            Self::Dim07 => "DIM-07",
            Self::Dim08 => "DIM-08",
            Self::Dim09 => "DIM-09",
            Self::Dim10 => "DIM-10",
            Self::Dim11 => "DIM-11",
            Self::Dim12 => "DIM-12",
            Self::Dim13 => "DIM-13",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Score(f64);

impl Score {
    pub fn new(value: f64) -> Result<Self, ScoreError> {
        if !(0.0..=10.0).contains(&value) || value.is_nan() {
            return Err(ScoreError::OutOfRange(value));
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rubric {
    pub version: String,
    pub rationale: String,
    pub weights: BTreeMap<Dimension, f64>,
}

impl Rubric {
    pub fn default_v0() -> Self {
        let weights = Dimension::ALL
            .into_iter()
            .map(|dimension| (dimension, 1.0))
            .collect();
        Self {
            version: "v0".to_string(),
            rationale: "Provisional equal-weight rubric; no calibration claim.".to_string(),
            weights,
        }
    }

    pub fn weight(&self, dimension: Dimension) -> Option<f64> {
        self.weights.get(&dimension).copied()
    }
}

pub trait DimensionScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Result<Score, ScoreError>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoredScoreScorer {
    fallback: Score,
}

impl StoredScoreScorer {
    pub fn new(fallback: Score) -> Self {
        Self { fallback }
    }
}

impl Default for StoredScoreScorer {
    fn default() -> Self {
        Self {
            fallback: Score::new(0.0).expect("zero is a valid score"),
        }
    }
}

impl DimensionScorer for StoredScoreScorer {
    fn score(&self, entry: &CorpusEntry, dimension: Dimension) -> Result<Score, ScoreError> {
        entry
            .scores
            .get(dimension.key())
            .copied()
            .map_or(Ok(self.fallback), Score::new)
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum ScoreError {
    #[error("score must be between 0 and 10 inclusive, got {0}")]
    OutOfRange(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_bounds_accept_zero_and_ten() {
        assert_eq!(Score::new(0.0).expect("zero valid").value(), 0.0);
        assert_eq!(Score::new(10.0).expect("ten valid").value(), 10.0);
    }

    #[test]
    fn score_bounds_reject_out_of_range_and_nan() {
        assert_eq!(Score::new(-0.1), Err(ScoreError::OutOfRange(-0.1)));
        assert_eq!(Score::new(10.1), Err(ScoreError::OutOfRange(10.1)));
        assert!(
            matches!(Score::new(f64::NAN), Err(ScoreError::OutOfRange(value)) if value.is_nan())
        );
    }

    #[test]
    fn default_rubric_v0_has_all_dimensions_and_rationale() {
        let rubric = Rubric::default_v0();

        assert_eq!(rubric.version, "v0");
        assert!(rubric.rationale.contains("no calibration claim"));
        assert_eq!(rubric.weights.len(), 13);
        assert_eq!(rubric.weight(Dimension::Dim13), Some(1.0));
    }

    #[test]
    fn stored_score_scorer_reads_dimension_key_from_corpus_entry() {
        let mut entry = CorpusEntry::default();
        entry.scores.insert("DIM-03".to_string(), 7.5);

        let scorer = StoredScoreScorer::default();

        assert_eq!(
            scorer
                .score(&entry, Dimension::Dim03)
                .expect("stored score valid")
                .value(),
            7.5
        );
        assert_eq!(
            scorer
                .score(&entry, Dimension::Dim04)
                .expect("fallback score valid")
                .value(),
            0.0
        );
    }

    #[test]
    fn stored_score_scorer_rejects_invalid_entry_score() {
        let mut entry = CorpusEntry::default();
        entry.scores.insert("DIM-01".to_string(), 12.0);

        let scorer = StoredScoreScorer::default();

        assert_eq!(
            scorer.score(&entry, Dimension::Dim01),
            Err(ScoreError::OutOfRange(12.0))
        );
    }
}
