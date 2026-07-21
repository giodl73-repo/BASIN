use basin_corpus::{CorpusEntry, EvidenceLabel};
use basin_network::{Network, NetworkError};
use basin_score::{Dimension, Score, ScoreError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Tier {
    T1,
    T2,
    T3,
    T4,
}

impl Tier {
    pub const ALL: [Self; 4] = [Self::T1, Self::T2, Self::T3, Self::T4];

    pub fn key(self) -> &'static str {
        match self {
            Self::T1 => "T1",
            Self::T2 => "T2",
            Self::T3 => "T3",
            Self::T4 => "T4",
        }
    }

    pub fn parse(value: &str) -> Result<Self, TierError> {
        match value.trim().to_ascii_uppercase().as_str() {
            "T1" => Ok(Self::T1),
            "T2" => Ok(Self::T2),
            "T3" => Ok(Self::T3),
            "T4" => Ok(Self::T4),
            other => Err(TierError::UnknownTier(other.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sla {
    pub tier: Tier,
    pub supply_reliability: String,
    pub quality: String,
    pub continuity: String,
    pub access: String,
    pub evidence_label: EvidenceLabel,
    pub rationale: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gap {
    pub entry_id: String,
    pub tier: Tier,
    pub dimension: Dimension,
    pub score: Score,
    pub reason: String,
    pub basis: String,
}

pub fn classify(entry: &CorpusEntry) -> Result<Tier, TierError> {
    let tier = entry.tier.as_deref().ok_or(TierError::MissingTier)?;
    Tier::parse(tier)
}

pub fn default_sla(tier: Tier) -> Sla {
    let (supply_reliability, quality, continuity, access) = match tier {
        Tier::T1 => (
            "firm drought-of-record yield",
            "potable / treated",
            "redundant supply",
            "regional gateway supply",
        ),
        Tier::T2 => (
            "firm normal-year yield",
            "potable / treated",
            "redundant supply",
            "municipal supply",
        ),
        Tier::T3 => (
            "conditional yield",
            "treated to standard",
            "single-source supply",
            "community supply",
        ),
        Tier::T4 => (
            "interruptible yield",
            "baseline treatment",
            "local-source supply",
            "local / rural supply",
        ),
    };
    Sla {
        tier,
        supply_reliability: supply_reliability.to_string(),
        quality: quality.to_string(),
        continuity: continuity.to_string(),
        access: access.to_string(),
        evidence_label: EvidenceLabel::Planned,
        rationale: "Provisional tier-SLA record pending calibrated operational evidence."
            .to_string(),
    }
}

pub fn conformance(entry: &CorpusEntry, network: &Network) -> Result<Score, TierError> {
    classify(entry)?;
    let required_af = required_capacity_af(entry).ok_or(TierError::MissingCapacityAf)?;
    let mut observed = 0_usize;
    let mut worst_score = 10.0_f64;

    for terminus in entry
        .termini
        .iter()
        .filter(|value| !value.trim().is_empty())
    {
        observed += 1;
        let redundant_af = network.redundant_capacity_af(terminus)?;
        let score = if required_af <= 0.0 {
            10.0
        } else {
            (redundant_af / required_af * 10.0).clamp(0.0, 10.0)
        };
        worst_score = worst_score.min(score);
    }

    if observed == 0 {
        return Err(TierError::MissingTerminus);
    }

    Score::new(worst_score).map_err(TierError::Score)
}

pub fn tier_sla_gap(entry: &CorpusEntry) -> Result<Option<Gap>, TierError> {
    let tier = classify(entry)?;
    let Some(raw_score) = entry.scores.get(Dimension::Dim13.key()).copied() else {
        return Ok(None);
    };
    gap_from_score(entry, tier, Score::new(raw_score)?)
}

pub fn tier_sla_gap_with_network(
    entry: &CorpusEntry,
    network: &Network,
) -> Result<Option<Gap>, TierError> {
    let tier = classify(entry)?;
    gap_from_score(entry, tier, conformance(entry, network)?)
}

fn gap_from_score(entry: &CorpusEntry, tier: Tier, score: Score) -> Result<Option<Gap>, TierError> {
    if score.value() >= 10.0 {
        return Ok(None);
    }

    Ok(Some(Gap {
        entry_id: entry.id.clone().ok_or(TierError::MissingEntryIdForGap)?,
        tier,
        dimension: Dimension::Dim13,
        score,
        reason: "DIM-13 score below full tier-SLA conformance".to_string(),
        basis: "tier SLA evaluated against redundant firm-yield (AF) basis".to_string(),
    }))
}

fn required_capacity_af(entry: &CorpusEntry) -> Option<f64> {
    entry
        .quantities
        .iter()
        .find(|quantity| quantity.unit.eq_ignore_ascii_case("AF"))
        .map(|quantity| quantity.value)
}

#[derive(Debug, Error, PartialEq)]
pub enum TierError {
    #[error("missing tier")]
    MissingTier,
    #[error("unknown tier: {0}")]
    UnknownTier(String),
    #[error("missing AF capacity quantity")]
    MissingCapacityAf,
    #[error("missing terminus for tier-SLA conformance")]
    MissingTerminus,
    #[error("missing entry id for tier-SLA gap")]
    MissingEntryIdForGap,
    #[error(transparent)]
    Network(#[from] NetworkError),
    #[error(transparent)]
    Score(#[from] ScoreError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use basin_corpus::Quantity;
    use basin_network::{Conveyance, HydroBasis, Node};

    fn node(id: &str) -> Node {
        Node {
            id: id.to_string(),
            name: format!("{id} node"),
            kind: "reservoir".to_string(),
        }
    }

    fn conveyance(id: &str, capacity_af: f64) -> Conveyance {
        Conveyance {
            id: id.to_string(),
            capacity_af,
            basis: HydroBasis::FirmYield,
        }
    }

    fn entry_with_tier(tier: &str) -> CorpusEntry {
        CorpusEntry {
            id: Some("system-1".to_string()),
            tier: Some(tier.to_string()),
            ..CorpusEntry::default()
        }
    }

    fn redundant_system() -> Network {
        let mut network = Network::new();
        network.add_node(node("a")).expect("node a accepted");
        network.add_node(node("b")).expect("node b accepted");
        network.add_node(node("c")).expect("node c accepted");
        network
            .add_conveyance("a", "b", conveyance("ab", 800.0))
            .expect("conveyance ab accepted");
        network
            .add_conveyance("a", "c", conveyance("ac", 600.0))
            .expect("conveyance ac accepted");
        network
    }

    #[test]
    fn classify_reads_declared_tier() {
        let entry = entry_with_tier("T2");

        assert_eq!(classify(&entry), Ok(Tier::T2));
    }

    #[test]
    fn default_sla_records_provisional_label() {
        let sla = default_sla(Tier::T1);

        assert_eq!(sla.evidence_label, EvidenceLabel::Planned);
        assert!(sla.rationale.contains("Provisional"));
        assert_eq!(sla.supply_reliability, "firm drought-of-record yield");
    }

    #[test]
    fn conformance_uses_redundant_capacity_for_dim13() {
        let mut entry = entry_with_tier("T1");
        entry.termini.push("a".to_string());
        entry.quantities.push(Quantity {
            value: 500.0,
            unit: "AF".to_string(),
            label: Some(EvidenceLabel::Planned),
            source_id: None,
        });

        let score = conformance(&entry, &redundant_system()).expect("entry conforms");

        assert_eq!(score.value(), 10.0);
    }

    #[test]
    fn tier_sla_gap_reports_dim13_shortfall() {
        let mut entry = entry_with_tier("T3");
        entry.scores.insert("DIM-13".to_string(), 6.5);

        let gap = tier_sla_gap(&entry)
            .expect("gap evaluation succeeds")
            .expect("shortfall produces gap");

        assert_eq!(gap.entry_id, "system-1");
        assert_eq!(gap.tier, Tier::T3);
        assert_eq!(gap.dimension, Dimension::Dim13);
        assert_eq!(gap.score.value(), 6.5);
        assert!(gap.basis.contains("AF"));
    }

    #[test]
    fn full_dim13_score_has_no_gap() {
        let mut entry = entry_with_tier("T4");
        entry.scores.insert("DIM-13".to_string(), 10.0);

        assert_eq!(tier_sla_gap(&entry), Ok(None));
    }
}
