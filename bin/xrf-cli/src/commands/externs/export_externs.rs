use crate::generic_command::{CommandResult, GenericCommand};
use clap::{Arg, ArgMatches, Command, value_parser};
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use xray_error::XRayError;
use xray_export::{
  ExternFormat, ExternManifest, ExternManifestParser, LineEndings, normalize_line_endings,
  render_extern_manifest,
};

/// Generate or verify a stable extern manifest from TypeScript declarations.
#[derive(Default)]
pub struct ExportExternsCommand;

impl GenericCommand for ExportExternsCommand {
  fn name(&self) -> &'static str {
    "export-externs"
  }

  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Export TypeScript extern declarations as JSON, XML, or HTML")
      .arg(
        Arg::new("declarations-root")
          .help("Root directory containing TypeScript declaration sources")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("format")
          .help("Output format; required with --output and inferred from --check when omitted")
          .long("format")
          .value_parser(["json", "xml", "html"]),
      )
      .arg(
        Arg::new("output")
          .help("Artifact to create or replace")
          .long("output")
          .conflicts_with("check")
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("check")
          .help("Existing artifact to verify without writing")
          .long("check")
          .conflicts_with("output")
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("line-endings")
          .help("Override generated line endings")
          .long("line-endings")
          .value_parser(["lf", "crlf"]),
      )
  }

  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let declarations_root: &PathBuf = matches
      .get_one("declarations-root")
      .expect("Expected declarations root");
    let output: Option<&PathBuf> = matches.get_one("output");
    let check: Option<&PathBuf> = matches.get_one("check");

    if output.is_none() && check.is_none() {
      return Err(
        XRayError::new_invalid_error("Specify exactly one of --output or --check.").into(),
      );
    }

    let format: ExternFormat = Self::resolve_format(matches, output, check)?;
    let line_endings: Option<LineEndings> = matches
      .get_one::<String>("line-endings")
      .map(|value: &String| LineEndings::from_str(value))
      .transpose()?;
    let parsed = ExternManifestParser::new().parse_directory(declarations_root)?;

    if let Some(path) = output {
      let content: String = render_extern_manifest(&parsed.manifest, format, line_endings)?;
      write_output(path, &content)?;
      println!(
        "Exported {} externs to '{}'.",
        parsed.manifest.exports.len(),
        path.display()
      );
      return Ok(());
    }

    let path: &PathBuf = check.expect("Checked path is required after validation");
    verify_artifact(path, format, &parsed.manifest, line_endings)?;
    println!(
      "Extern artifact '{}' matches {} declarations.",
      path.display(),
      parsed.manifest.exports.len()
    );

    Ok(())
  }
}

impl ExportExternsCommand {
  fn resolve_format(
    matches: &ArgMatches,
    output: Option<&PathBuf>,
    check: Option<&PathBuf>,
  ) -> Result<ExternFormat, XRayError> {
    if let Some(value) = matches.get_one::<String>("format") {
      return ExternFormat::from_str(value);
    }
    if let Some(path) = check {
      return ExternFormat::from_extension(path);
    }
    let path: &PathBuf = output.expect("Output is required after validation");

    Err(XRayError::new_invalid_error(format!(
      "--format is required when writing '{}'.",
      path.display()
    )))
  }
}

fn write_output(path: &Path, content: &str) -> Result<(), XRayError> {
  if let Some(parent) = path.parent() {
    fs::create_dir_all(parent)?;
  }
  fs::write(path, content)?;

  Ok(())
}

fn verify_artifact(
  path: &Path,
  format: ExternFormat,
  manifest: &ExternManifest,
  line_endings: Option<LineEndings>,
) -> Result<(), XRayError> {
  let existing: String = fs::read_to_string(path)?;

  match format {
    ExternFormat::Json => {
      let actual: ExternManifest = serde_json::from_str(&existing).map_err(|error| {
        XRayError::new_invalid_error(format!(
          "Cannot parse '{}' as an extern JSON manifest: {error}",
          path.display()
        ))
      })?;
      if actual != *manifest {
        return Err(XRayError::new_verify_error(format!(
          "Extern JSON artifact '{}' does not match the parsed declaration manifest.",
          path.display()
        )));
      }
    }
    ExternFormat::Xml | ExternFormat::Html => {
      let expected: String = render_extern_manifest(manifest, format, line_endings)?;
      if normalize_line_endings(&existing) != normalize_line_endings(&expected) {
        return Err(XRayError::new_verify_error(format!(
          "Extern {} artifact '{}' does not match freshly rendered output.",
          match format {
            ExternFormat::Xml => "XML",
            ExternFormat::Html => "HTML",
            ExternFormat::Json => unreachable!(),
          },
          path.display()
        )));
      }
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::ExportExternsCommand;
  use crate::generic_command::GenericCommand;
  use clap::ArgMatches;
  use std::path::PathBuf;
  use xray_export::ExternFormat;

  #[test]
  fn infers_check_format_from_extension() {
    let matches: ArgMatches = ExportExternsCommand::new()
      .init()
      .try_get_matches_from(["export-externs", "declarations", "--check", "extern.xml"])
      .unwrap();
    let check: Option<&PathBuf> = matches.get_one("check");

    assert_eq!(
      ExportExternsCommand::resolve_format(&matches, None, check).unwrap(),
      ExternFormat::Xml
    );
  }

  #[test]
  fn rejects_conflicting_output_modes() {
    assert!(
      ExportExternsCommand::new()
        .init()
        .try_get_matches_from([
          "export-externs",
          "declarations",
          "--output",
          "extern.json",
          "--check",
          "extern.json",
        ])
        .is_err()
    );
  }
}
