use std::{
    fs,
    path::{Path, PathBuf},
};

use basin_corpus::{CorpusEntry, Scale, ValidationSeverity};
use basin_gap::{find_gaps, null_result, GapRegion, NullResult};
use basin_score::{Dimension, DimensionScorer, Rubric, StoredScoreScorer};
use basin_tier::{classify, tier_sla_gap, Gap as TierGap, Tier};
use clap::{Parser, Subcommand};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(name = "basin")]
#[command(about = "BASIN product CLI: corpus, score, tier-sla, and gap artifacts")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate corpus markdown while preserving evidence labels and scale.
    Corpus(ArtifactArgs),
    /// Emit DIM-01..13 score artifacts from corpus entries.
    Score(ArtifactArgs),
    /// Emit tier-SLA classification and shortfall artifacts.
    TierSla(ArtifactArgs),
    /// Emit gap analysis and null-result artifacts at a chosen scale.
    Gap(GapArgs),
}

#[derive(Debug, Clone, Parser)]
struct ArtifactArgs {
    /// Corpus markdown file or directory of .md files.
    #[arg(long)]
    input: PathBuf,
    /// Optional JSON artifact target. When omitted, JSON is written to stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Parser)]
struct GapArgs {
    /// Corpus markdown file or directory of .md files.
    #[arg(long)]
    input: PathBuf,
    /// Optional JSON artifact target. When omitted, JSON is written to stdout.
    #[arg(long)]
    output: Option<PathBuf>,
    /// Scale to analyze: international, national, regional, or local.
    #[arg(long)]
    scale: String,
    /// Allow cross-scale comparison (REQ-016 explicit marker).
    #[arg(long, default_value_t = false)]
    cross_scale: bool,
}

fn main() {
    if let Err(err) = run(Cli::parse()) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), CliError> {
    match cli.command {
        Command::Corpus(args) => {
            write_artifact(&args, corpus_artifact(&load_entries(&args.input)?))
        }
        Command::Score(args) => write_artifact(&args, score_artifact(&load_entries(&args.input)?)?),
        Command::TierSla(args) => {
            write_artifact(&args, tier_sla_artifact(&load_entries(&args.input)?)?)
        }
        Command::Gap(args) => {
            let scale = parse_scale(&args.scale)?;
            let entries = load_entries(&args.input)?;
            let artifact = gap_artifact(&entries, scale, args.cross_scale)?;
            write_gap_artifact(&args, artifact)
        }
    }
}

fn parse_scale(value: &str) -> Result<Scale, CliError> {
    Scale::parse(value).ok_or_else(|| CliError::UnknownScale(value.to_string()))
}

fn load_entries(input: &Path) -> Result<Vec<CorpusEntry>, CliError> {
    let mut paths = Vec::new();
    if input.is_dir() {
        for entry in fs::read_dir(input)? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                paths.push(path);
            }
        }
        paths.sort();
    } else {
        paths.push(input.to_path_buf());
    }

    let mut entries = Vec::new();
    for path in paths {
        let body = fs::read_to_string(&path)?
            .replace("\r\n", "\n")
            .replace('\r', "\n");
        let mut entry = CorpusEntry::from_markdown(&body)?;
        entry.scores.extend(parse_score_rows(&body)?);
        entries.push(entry);
    }
    Ok(entries)
}

fn parse_score_rows(body: &str) -> Result<Vec<(String, f64)>, CliError> {
    let mut scores = Vec::new();
    for raw_line in body.lines() {
        let line = raw_line.trim();
        let Some(row) = line.strip_prefix("score:") else {
            continue;
        };
        let parts = row.split('|').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(CliError::MalformedScore(line.to_string()));
        }
        scores.push((
            parts[0].to_string(),
            parts[1]
                .parse::<f64>()
                .map_err(|_| CliError::MalformedScore(line.to_string()))?,
        ));
    }
    Ok(scores)
}

fn write_artifact<T: Serialize>(args: &ArtifactArgs, artifact: T) -> Result<(), CliError> {
    write_json(args.output.as_deref(), artifact)
}

fn write_gap_artifact(args: &GapArgs, artifact: GapArtifact) -> Result<(), CliError> {
    write_json(args.output.as_deref(), artifact)
}

fn write_json<T: Serialize>(output: Option<&Path>, artifact: T) -> Result<(), CliError> {
    let body = serde_json::to_string_pretty(&artifact)?;
    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output, format!("{body}\n"))?;
    } else {
        println!("{body}");
    }
    Ok(())
}

fn corpus_artifact(entries: &[CorpusEntry]) -> CorpusArtifact {
    let mut rows = Vec::new();
    for entry in entries {
        let report = entry.validate();
        rows.push(CorpusValidationRow {
            id: entry.id.clone(),
            scale: entry.scale,
            promotable: report.is_promotable(),
            held: report
                .issues
                .iter()
                .filter(|issue| issue.severity == ValidationSeverity::Held)
                .map(|issue| issue.reason.clone())
                .collect(),
            rejected: report
                .issues
                .iter()
                .filter(|issue| issue.severity == ValidationSeverity::Rejected)
                .map(|issue| issue.reason.clone())
                .collect(),
        });
    }
    CorpusArtifact {
        artifact: "corpus-validation-v0",
        entries: rows,
    }
}

fn score_artifact(entries: &[CorpusEntry]) -> Result<ScoreArtifact, CliError> {
    let scorer = StoredScoreScorer::default();
    let mut rows = Vec::new();
    for entry in entries {
        let mut scores = Vec::new();
        for dimension in Dimension::ALL {
            scores.push(DimensionScore {
                dimension: dimension.key().to_string(),
                score: scorer.score(entry, dimension)?.value(),
            });
        }
        rows.push(ScoreRow {
            id: entry.id.clone().ok_or(CliError::MissingEntryId)?,
            scores,
        });
    }
    Ok(ScoreArtifact {
        artifact: "score-v0",
        rubric_version: Rubric::default_v0().version,
        entries: rows,
    })
}

fn tier_sla_artifact(entries: &[CorpusEntry]) -> Result<TierSlaArtifact, CliError> {
    let mut rows = Vec::new();
    for entry in entries {
        let id = entry.id.clone().ok_or(CliError::MissingEntryId)?;
        let tier = classify(entry)?;
        rows.push(TierSlaRow {
            id,
            tier,
            gap: tier_sla_gap(entry)?,
        });
    }
    Ok(TierSlaArtifact {
        artifact: "tier-sla-v0",
        entries: rows,
    })
}

fn gap_artifact(
    entries: &[CorpusEntry],
    scale: Scale,
    cross_scale: bool,
) -> Result<GapArtifact, CliError> {
    let rubric = Rubric::default_v0();
    let gaps = find_gaps(entries, &rubric, scale, cross_scale)?;
    let null = if gaps.is_empty() {
        null_result("adequate-system", entries, &rubric, scale, cross_scale)?
    } else {
        None
    };
    Ok(GapArtifact {
        artifact: "gap-v0",
        scale,
        cross_scale,
        gaps,
        null_result: null,
    })
}

#[derive(Debug, Serialize)]
struct CorpusArtifact {
    artifact: &'static str,
    entries: Vec<CorpusValidationRow>,
}

#[derive(Debug, Serialize)]
struct CorpusValidationRow {
    id: Option<String>,
    scale: Option<Scale>,
    promotable: bool,
    held: Vec<String>,
    rejected: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ScoreArtifact {
    artifact: &'static str,
    rubric_version: String,
    entries: Vec<ScoreRow>,
}

#[derive(Debug, Serialize)]
struct ScoreRow {
    id: String,
    scores: Vec<DimensionScore>,
}

#[derive(Debug, Serialize)]
struct DimensionScore {
    dimension: String,
    score: f64,
}

#[derive(Debug, Serialize)]
struct TierSlaArtifact {
    artifact: &'static str,
    entries: Vec<TierSlaRow>,
}

#[derive(Debug, Serialize)]
struct TierSlaRow {
    id: String,
    tier: Tier,
    gap: Option<TierGap>,
}

#[derive(Debug, Serialize)]
struct GapArtifact {
    artifact: &'static str,
    scale: Scale,
    cross_scale: bool,
    gaps: Vec<GapRegion>,
    null_result: Option<NullResult>,
}

#[derive(Debug, Error)]
enum CliError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Corpus(#[from] basin_corpus::CorpusError),
    #[error(transparent)]
    Score(#[from] basin_score::ScoreError),
    #[error(transparent)]
    Tier(#[from] basin_tier::TierError),
    #[error(transparent)]
    Gap(#[from] basin_gap::GapError),
    #[error("score row is malformed: {0}")]
    MalformedScore(String),
    #[error("unknown scale: {0}")]
    UnknownScale(String),
    #[error("entry requires stable id for CLI artifact")]
    MissingEntryId,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed_corpus() -> String {
        let mut body = String::from(
            "---\nid: seed-1\ntype: reservoir\nscale: regional\nbasin: sacramento\ntermini: [a]\ntier: T1\n---\nquantity: 500 | AF | planned | -\n",
        );
        for dimension in Dimension::ALL {
            body.push_str(&format!("score: {} | 10\n", dimension.key()));
        }
        body
    }

    fn temp_file(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join("basin-cli-tests");
        fs::create_dir_all(&root).expect("test temp dir");
        root.join(name)
    }

    #[test]
    fn corpus_score_tier_and_gap_artifacts_are_deterministic() {
        let input = temp_file("seed.md");
        fs::write(&input, seed_corpus()).expect("seed corpus write");
        let entries = load_entries(&input).expect("entries load");

        let corpus = serde_json::to_string(&corpus_artifact(&entries)).expect("corpus json");
        let score = serde_json::to_string(&score_artifact(&entries).expect("score artifact"))
            .expect("score json");
        let tier = serde_json::to_string(&tier_sla_artifact(&entries).expect("tier artifact"))
            .expect("tier json");
        let gap = gap_artifact(&entries, Scale::Regional, false).expect("gap artifact");

        assert!(corpus.contains("corpus-validation-v0"));
        assert!(score.contains("\"DIM-13\""));
        assert!(tier.contains("tier-sla-v0"));
        assert!(gap.gaps.is_empty());
        assert_eq!(
            gap.null_result.expect("null result").region_id,
            "adequate-system"
        );
    }

    #[test]
    fn help_surface_exposes_scale_and_excludes_process_subcommands() {
        use clap::CommandFactory;

        let mut command = Cli::command();
        let help = command.render_long_help().to_string();
        let subcommands = command
            .get_subcommands()
            .map(|subcommand| subcommand.get_name().to_string())
            .collect::<Vec<_>>();

        assert!(help.contains("corpus"));
        assert!(help.contains("score"));
        assert!(help.contains("tier-sla"));
        assert!(help.contains("gap"));

        let gap_help = Cli::command()
            .get_subcommands()
            .find(|sub| sub.get_name() == "gap")
            .expect("gap subcommand")
            .clone()
            .render_long_help()
            .to_string();
        assert!(gap_help.contains("--scale"));

        assert!(!subcommands.iter().any(|name| name == "work-package"));
        assert!(!subcommands.iter().any(|name| name == "prove"));
        assert!(!subcommands.iter().any(|name| name == "evidence"));
    }
}
