use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use basin_network::HydroBasis;

/// Analysis scale (NEED-008 / CON-007): every corpus element declares the scale
/// it is interpreted within.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scale {
    International,
    National,
    Regional,
    Local,
}

impl Scale {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "international" => Some(Self::International),
            "national" => Some(Self::National),
            "regional" => Some(Self::Regional),
            "local" => Some(Self::Local),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvidenceLabel {
    Implemented,
    Heuristic,
    Simulated,
    Proxy,
    Planned,
    Held,
    SourceNeeded,
    ConfidenceLimited,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: f64,
    pub unit: String,
    pub label: Option<EvidenceLabel>,
    pub source_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CorpusEntry {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub element_type: Option<String>,
    pub scale: Option<Scale>,
    pub basin: Option<String>,
    pub termini: Vec<String>,
    pub tier: Option<String>,
    pub sla: Option<String>,
    pub quantities: Vec<Quantity>,
    pub scores: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationSeverity {
    Held,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub reason: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidationReport {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    pub fn rejected(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ValidationSeverity::Rejected)
    }

    pub fn held(&self) -> impl Iterator<Item = &ValidationIssue> {
        self.issues
            .iter()
            .filter(|issue| issue.severity == ValidationSeverity::Held)
    }

    pub fn is_promotable(&self) -> bool {
        self.issues.is_empty()
    }
}

impl CorpusEntry {
    pub fn validate(&self) -> ValidationReport {
        let mut report = ValidationReport::default();
        if self.id.as_deref().is_none_or(str::is_empty) {
            report.issues.push(ValidationIssue {
                severity: ValidationSeverity::Rejected,
                reason: "missing stable element id".to_string(),
            });
        }

        if self.scale.is_none() {
            report.issues.push(ValidationIssue {
                severity: ValidationSeverity::Held,
                reason: "missing scale tag (CON-007)".to_string(),
            });
        }

        for quantity in &self.quantities {
            if quantity.source_id.as_deref().is_none_or(str::is_empty) && quantity.label.is_none() {
                report.issues.push(ValidationIssue {
                    severity: ValidationSeverity::Held,
                    reason: format!(
                        "quantity {} {} lacks source id or evidence label",
                        quantity.value, quantity.unit
                    ),
                });
            }
        }

        report
    }

    pub fn from_markdown(markdown: &str) -> Result<Self, CorpusError> {
        let (frontmatter, body) = split_frontmatter(markdown)?;
        let mut entry = parse_frontmatter(frontmatter)?;
        entry.quantities = parse_quantities(body)?;
        Ok(entry)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CorpusError {
    #[error("corpus entry is missing frontmatter")]
    MissingFrontmatter,
    #[error("frontmatter line is malformed: {0}")]
    MalformedFrontmatter(String),
    #[error("quantity row is malformed: {0}")]
    MalformedQuantity(String),
    #[error("unknown evidence label: {0}")]
    UnknownEvidenceLabel(String),
    #[error("unknown scale: {0}")]
    UnknownScale(String),
}

fn split_frontmatter(markdown: &str) -> Result<(&str, &str), CorpusError> {
    let Some(rest) = markdown.strip_prefix("---\n") else {
        return Err(CorpusError::MissingFrontmatter);
    };
    let Some((frontmatter, body)) = rest.split_once("\n---\n") else {
        return Err(CorpusError::MissingFrontmatter);
    };
    Ok((frontmatter, body))
}

fn parse_frontmatter(frontmatter: &str) -> Result<CorpusEntry, CorpusError> {
    let mut entry = CorpusEntry::default();
    for raw_line in frontmatter.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            return Err(CorpusError::MalformedFrontmatter(line.to_string()));
        };
        let value = value.trim();
        match key.trim() {
            "id" => entry.id = optional_string(value),
            "type" => entry.element_type = optional_string(value),
            "scale" => entry.scale = parse_optional_scale(value)?,
            "basin" => entry.basin = optional_string(value),
            "termini" => entry.termini = parse_string_list(value),
            "tier" => entry.tier = optional_string(value),
            "sla" => entry.sla = optional_string(value),
            _ => {}
        }
    }
    Ok(entry)
}

fn parse_quantities(body: &str) -> Result<Vec<Quantity>, CorpusError> {
    let mut quantities = Vec::new();
    for raw_line in body.lines() {
        let line = raw_line.trim();
        let Some(row) = line.strip_prefix("quantity:") else {
            continue;
        };
        let parts = row.split('|').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(CorpusError::MalformedQuantity(line.to_string()));
        }
        let value = parts[0]
            .parse::<f64>()
            .map_err(|_| CorpusError::MalformedQuantity(line.to_string()))?;
        quantities.push(Quantity {
            value,
            unit: parts[1].to_string(),
            label: parse_optional_label(parts[2])?,
            source_id: optional_string(parts[3]),
        });
    }
    Ok(quantities)
}

fn parse_string_list(value: &str) -> Vec<String> {
    value
        .trim_matches(['[', ']'])
        .split(',')
        .map(|item| item.trim().trim_matches('"'))
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn optional_string(value: &str) -> Option<String> {
    let trimmed = value.trim().trim_matches('"');
    if trimmed.is_empty() || trimmed == "-" {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_optional_scale(value: &str) -> Result<Option<Scale>, CorpusError> {
    let Some(value) = optional_string(value) else {
        return Ok(None);
    };
    Scale::parse(&value)
        .map(Some)
        .ok_or(CorpusError::UnknownScale(value))
}

fn parse_optional_label(value: &str) -> Result<Option<EvidenceLabel>, CorpusError> {
    let Some(value) = optional_string(value) else {
        return Ok(None);
    };
    let label = match value.as_str() {
        "implemented" => EvidenceLabel::Implemented,
        "heuristic" => EvidenceLabel::Heuristic,
        "simulated" => EvidenceLabel::Simulated,
        "proxy" => EvidenceLabel::Proxy,
        "planned" => EvidenceLabel::Planned,
        "held" => EvidenceLabel::Held,
        "source-needed" => EvidenceLabel::SourceNeeded,
        "confidence-limited" => EvidenceLabel::ConfidenceLimited,
        _ => return Err(CorpusError::UnknownEvidenceLabel(value)),
    };
    Ok(Some(label))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_id_is_rejected() {
        let entry = CorpusEntry {
            id: None,
            element_type: Some("reservoir".to_string()),
            scale: Some(Scale::Regional),
            ..CorpusEntry::default()
        };

        let report = entry.validate();

        assert_eq!(report.rejected().count(), 1);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.reason == "missing stable element id"));
    }

    #[test]
    fn missing_scale_is_held() {
        let entry = CorpusEntry {
            id: Some("reservoir:shasta".to_string()),
            scale: None,
            ..CorpusEntry::default()
        };

        let report = entry.validate();

        assert_eq!(report.held().count(), 1);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.reason.contains("missing scale tag")));
    }

    #[test]
    fn uncited_quantity_without_label_is_held() {
        let entry = CorpusEntry {
            id: Some("reservoir:shasta".to_string()),
            scale: Some(Scale::Regional),
            quantities: vec![Quantity {
                value: 4_500_000.0,
                unit: "AF".to_string(),
                label: None,
                source_id: None,
            }],
            ..CorpusEntry::default()
        };

        let report = entry.validate();

        assert_eq!(report.held().count(), 1);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.reason.contains("lacks source id or evidence label")));
    }

    #[test]
    fn label_and_scale_are_preserved_from_markdown_frontmatter_entry() {
        let entry = CorpusEntry::from_markdown(
            "---
id: reservoir:shasta
type: reservoir
scale: regional
basin: sacramento
termini: [shasta]
tier: T1
sla: supply-critical
---

quantity: 4500000 | AF | source-needed | -
",
        )
        .expect("fixture should parse");

        assert_eq!(entry.id.as_deref(), Some("reservoir:shasta"));
        assert_eq!(entry.scale, Some(Scale::Regional));
        assert_eq!(entry.basin.as_deref(), Some("sacramento"));
        assert_eq!(entry.quantities[0].label, Some(EvidenceLabel::SourceNeeded));
        assert_eq!(entry.validate().held().count(), 0);
    }

    #[test]
    fn hydro_basis_is_reexported() {
        assert_ne!(HydroBasis::FirmYield, HydroBasis::AverageYear);
    }
}
