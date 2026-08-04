use crate::generic_command::CommandResult;
use serde::Serialize;
use std::path::Path;
use xray_gamedata::{
  GamedataVerificationCheckReport, GamedataVerificationFinding, GamedataVerificationResult,
};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GamedataVerificationReportOutput {
  checks: Vec<GamedataVerificationCheckReportOutput>,
  duration_ms: u128,
  status: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GamedataVerificationCheckReportOutput {
  duration_ms: Option<u128>,
  findings: Vec<GamedataVerificationFindingOutput>,
  status: String,
  summary: String,
  verification_type: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GamedataVerificationFindingOutput {
  asset_path: Option<String>,
  message: String,
  rule_id: Option<String>,
}

pub struct GamedataVerificationReportWriter<'a> {
  root: &'a Path,
  report: &'a GamedataVerificationResult,
}

impl<'a> GamedataVerificationReportWriter<'a> {
  pub fn new(root: &'a Path, report: &'a GamedataVerificationResult) -> Self {
    Self { root, report }
  }

  pub fn write(&self, report_path: &Path) -> CommandResult {
    let output: GamedataVerificationReportOutput = self.report_output();
    let json: String = serde_json::to_string_pretty(&output)?;

    std::fs::write(report_path, format!("{json}\n"))?;

    Ok(())
  }

  fn report_output(&self) -> GamedataVerificationReportOutput {
    GamedataVerificationReportOutput {
      checks: self
        .report
        .checks
        .iter()
        .map(|check| self.check_report_output(check))
        .collect(),
      duration_ms: self.report.duration,
      status: self.report.status().to_string(),
    }
  }

  fn check_report_output(
    &self,
    check: &GamedataVerificationCheckReport,
  ) -> GamedataVerificationCheckReportOutput {
    let mut findings: Vec<GamedataVerificationFindingOutput> = check
      .findings
      .iter()
      .map(|finding| self.finding_output(finding))
      .collect();

    findings.sort_by(|left, right| {
      left
        .asset_path
        .cmp(&right.asset_path)
        .then_with(|| left.message.cmp(&right.message))
    });

    GamedataVerificationCheckReportOutput {
      duration_ms: check.duration,
      findings,
      status: check.status.to_string(),
      summary: check.summary.clone(),
      verification_type: check.verification_type.to_string(),
    }
  }

  fn finding_output(
    &self,
    finding: &GamedataVerificationFinding,
  ) -> GamedataVerificationFindingOutput {
    GamedataVerificationFindingOutput {
      asset_path: finding.asset_path.as_ref().map(|asset_path| {
        asset_path
          .strip_prefix(self.root)
          .unwrap_or(asset_path)
          .to_string_lossy()
          .replace('\\', "/")
      }),
      message: finding.message.clone(),
      rule_id: finding.rule_id.clone(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataVerificationReportWriter;
  use std::fs;
  use std::path::PathBuf;
  use std::sync::atomic::{AtomicU64, Ordering};
  use xray_gamedata::{
    GamedataVerificationCheckReport, GamedataVerificationFinding, GamedataVerificationReport,
    GamedataVerificationStatus, GamedataVerificationType,
  };

  static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

  fn temporary_gamedata_root() -> PathBuf {
    let unique: u64 = NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed);
    let root: PathBuf = std::env::temp_dir().join(format!(
      "xrf-cli-verification-report-test-{}-{unique}",
      std::process::id()
    ));

    fs::create_dir_all(root.join("textures")).unwrap();
    fs::write(root.join("textures").join("a.dds"), []).unwrap();
    fs::write(root.join("textures").join("z.dds"), []).unwrap();

    root
  }

  #[test]
  fn writes_root_relative_paths_and_sorted_findings() {
    let root: PathBuf = temporary_gamedata_root();
    let report_path: PathBuf = root.join("report.json");
    let report: GamedataVerificationReport = GamedataVerificationReport {
      checks: vec![GamedataVerificationCheckReport {
        duration: Some(7),
        findings: vec![
          GamedataVerificationFinding::for_asset_in_rule(
            "textures.dds",
            root.join("textures").join("z.dds"),
            "Second finding",
          ),
          GamedataVerificationFinding::for_asset(
            root.join("textures").join("a.dds"),
            "First finding",
          ),
        ],
        status: GamedataVerificationStatus::Failed,
        summary: String::from("2/2 textures are invalid"),
        verification_type: GamedataVerificationType::Textures,
      }],
      duration: 42,
    };

    GamedataVerificationReportWriter::new(&root, &report)
      .write(&report_path)
      .unwrap();
    let json: serde_json::Value =
      serde_json::from_str(&fs::read_to_string(&report_path).unwrap()).unwrap();

    fs::remove_dir_all(&root).unwrap();

    assert!(json.get("schemaVersion").is_none());
    assert_eq!(json["status"], "failed");
    assert_eq!(json["durationMs"], 42);
    assert_eq!(json["checks"][0]["durationMs"], 7);
    assert_eq!(json["checks"][0]["verificationType"], "textures");
    assert_eq!(
      json["checks"][0]["findings"][0]["assetPath"],
      "textures/a.dds"
    );
    assert_eq!(
      json["checks"][0]["findings"][1]["assetPath"],
      "textures/z.dds"
    );
    assert_eq!(json["checks"][0]["findings"][1]["ruleId"], "textures.dds");
  }
}
