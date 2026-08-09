use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use std::path::PathBuf;
use xray_db::{OmfFile, XRayByteOrder};
use xray_output::OutputOptions;

#[derive(Default)]
pub struct InfoOmfCommand;

impl GenericCommand for InfoOmfCommand {
  fn name(&self) -> &'static str {
    "info-omf"
  }

  /// Create command for printing omf file info.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to print information about provided omf file")
      .arg(
        Arg::new("path")
          .help("Path to ogf file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("silent")
          .help("Disable any logging")
          .short('s')
          .long("silent")
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .short('v')
          .long("verbose")
          .action(ArgAction::SetTrue),
      )
  }

  /// Print information about ogf file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let output: OutputOptions =
      TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    xray_output::info!(output, "Read omf file {}", path.display());

    let omf_file: Box<OmfFile> = Box::new(OmfFile::read_from_path::<XRayByteOrder, _>(path)?);

    xray_output::info!(output, "Omf file information");

    xray_output::info!(output, "Version: {}", omf_file.parameters.version);

    xray_output::info!(
      output,
      "Motions: {} {}",
      omf_file.motions.motions.len(),
      omf_file
        .motions
        .motions
        .iter()
        .map(|it| it.name.as_str())
        .collect::<Vec<_>>()
        .join(",")
    );

    // Keyframe count and playback speed together give effective duration.
    for definition in &omf_file.parameters.motions {
      let keyframes: Option<u32> = omf_file
        .motions
        .motions
        .get(definition.motion as usize)
        .map(|it| it.count);

      xray_output::verbose!(
        output,
        "Motion '{}': keyframes {}, speed {}, power {}, accrue {}, falloff {}",
        definition.name,
        keyframes.map_or_else(|| String::from("?"), |it| it.to_string()),
        definition.speed,
        definition.power,
        definition.accrue,
        definition.falloff
      );
    }

    xray_output::info!(
      output,
      "Bones total: {}",
      omf_file.parameters.get_bones_count()
    );
    xray_output::info!(
      output,
      "Parts: {}",
      omf_file
        .parameters
        .parts
        .iter()
        .map(|it| it.name.as_str())
        .collect::<Vec<_>>()
        .join(",")
    );

    for part in &omf_file.parameters.parts {
      xray_output::info!(
        output,
        "Part '{}' bones: {}",
        part.name,
        part.get_bones().join(",")
      );
    }

    Ok(())
  }
}
