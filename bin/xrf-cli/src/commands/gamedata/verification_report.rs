use crate::generic_command::CommandResult;
use serde::Serialize;
use std::path::Path;
use xray_gamedata::{GamedataVerificationCheckReport, GamedataVerificationResult};
use xray_report::{CheckReport, Finding, Report};

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
  rule_id: String,
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
    let output: GamedataVerificationReportOutput = self.report_output()?;
    let json: String = serde_json::to_string_pretty(&output)?;

    std::fs::write(report_path, format!("{json}\n"))?;

    Ok(())
  }

  fn report_output(&self) -> CommandResult<GamedataVerificationReportOutput> {
    let report: Report = self.report.to_report();
    let checks: Vec<GamedataVerificationCheckReportOutput> = self
      .report
      .checks()
      .iter()
      .zip(report.checks())
      .map(|(gamedata_check, check)| self.check_report_output(gamedata_check, check))
      .collect();

    Ok(GamedataVerificationReportOutput {
      checks,
      duration_ms: self.report.duration().as_millis(),
      status: report.status().to_string(),
    })
  }

  fn check_report_output(
    &self,
    gamedata_report: &GamedataVerificationCheckReport,
    report: &CheckReport,
  ) -> GamedataVerificationCheckReportOutput {
    let findings: Vec<GamedataVerificationFindingOutput> = report
      .findings()
      .iter()
      .map(|finding| self.finding_output(finding))
      .collect();

    GamedataVerificationCheckReportOutput {
      duration_ms: report.duration().map(|duration| duration.as_millis()),
      findings,
      status: report.status().to_string(),
      summary: gamedata_report.summary().to_string(),
      verification_type: report.id().to_string(),
    }
  }

  fn finding_output(&self, finding: &Finding) -> GamedataVerificationFindingOutput {
    GamedataVerificationFindingOutput {
      asset_path: finding.subject().map(|subject| {
        let asset_path: &Path = Path::new(subject);

        asset_path
          .strip_prefix(self.root)
          .unwrap_or(asset_path)
          .to_string_lossy()
          .replace('\\', "/")
      }),
      message: finding.message().to_string(),
      rule_id: finding.rule_id().to_string(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GamedataVerificationReportWriter;
  use std::fs;
  use std::path::PathBuf;
  use std::sync::atomic::{AtomicU64, Ordering};
  use std::time::Duration;
  use xray_gamedata::{
    GamedataCheckResult, GamedataVerificationFinding, GamedataVerificationReport,
    GamedataVerificationRule, GamedataVerificationStatus, GamedataVerificationType,
  };

  static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

  struct TestCheckResult {
    duration: Duration,
    findings: Vec<GamedataVerificationFinding>,
  }

  impl GamedataCheckResult for TestCheckResult {
    fn duration(&self) -> Option<Duration> {
      Some(self.duration)
    }

    fn status(&self) -> GamedataVerificationStatus {
      GamedataVerificationStatus::Failed
    }

    fn failure_message(&self) -> String {
      String::from("2/2 textures are invalid")
    }

    fn findings(&self) -> &[GamedataVerificationFinding] {
      &self.findings
    }
  }

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
    let mut report: GamedataVerificationReport =
      GamedataVerificationReport::with_duration(Duration::from_millis(42));

    report.add_check(
      GamedataVerificationType::Textures,
      Ok(TestCheckResult {
        duration: Duration::from_millis(7),
        findings: vec![
          GamedataVerificationFinding::for_asset(
            GamedataVerificationRule::TexturesValidation,
            root.join("textures").join("z.dds"),
            "Second finding",
          ),
          GamedataVerificationFinding::for_asset(
            GamedataVerificationRule::TexturesValidation,
            root.join("textures").join("a.dds"),
            "First finding",
          ),
        ],
      }),
    );

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
