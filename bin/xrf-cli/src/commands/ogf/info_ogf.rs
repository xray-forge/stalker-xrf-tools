use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;
use clap::{Arg, ArgMatches, Command, value_parser};
use std::path::PathBuf;
use xray_db::{OgfFile, XRayByteOrder};
use xray_output::OutputOptions;

#[derive(Default)]
pub struct InfoOgfCommand;

impl GenericCommand for InfoOgfCommand {
  fn name(&self) -> &'static str {
    "info-ogf"
  }

  /// Create command for printing ogf file info.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to print information about provided ogf file")
      .arg(
        Arg::new("path")
          .help("Path to ogf file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Print information about ogf file.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let path: &PathBuf = matches
      .get_one::<_>("path")
      .expect("Expected valid path to be provided");

    let output: OutputOptions = TerminalOutput::from_options(false, false);

    xray_output::info!(output, "Read ogf file {}", path.display());

    let ogf_file: Box<OgfFile> = Box::new(OgfFile::read_from_path::<XRayByteOrder, _>(path)?);

    xray_output::info!(output, "Ogf file information");

    xray_output::info!(
      output,
      "Version: {}, model_type: {}, shader_id: {}, {:?} - {:?}",
      ogf_file.header.version,
      ogf_file.header.model_type,
      ogf_file.header.shader_id,
      ogf_file.header.bounding_box,
      ogf_file.header.bounding_sphere
    );

    xray_output::info!(output, "Boundaries box: {:?}", ogf_file.header.bounding_box);
    xray_output::info!(
      output,
      "Boundaries sphere: {:?}",
      ogf_file.header.bounding_sphere
    );

    if let Some(texture) = &ogf_file.texture {
      xray_output::info!(output, "Texture name: {}", texture.texture_name);
      xray_output::info!(output, "Shader name: {}", texture.shader_name);
    }

    if let Some(description) = &ogf_file.description {
      xray_output::info!(output, "Description: {:?}", description);
    }

    if let Some(bones) = &ogf_file.bones {
      xray_output::info!(output, "Bones: {}", bones.bones.len());

      for (index, bone) in bones.bones.iter().enumerate() {
        xray_output::info!(output, "[{}] name: {}", index, bone.name);
        xray_output::info!(output, "[{}] parent: {}", index, bone.parent);
      }
    }

    if let Some(kinematics) = &ogf_file.kinematics {
      xray_output::info!(output, "Motion refs: {:?}", kinematics.motion_refs);
    }

    if let Some(children) = &ogf_file.children {
      xray_output::info!(output, "OGF children ({}):", children.nested.len());

      for (index, child) in children.nested.iter().enumerate() {
        if let Some(texture) = &child.texture {
          xray_output::info!(output, "[{}] texture name: {}", index, texture.texture_name);
          xray_output::info!(output, "[{}] shader name: {}", index, texture.shader_name);
        }
      }
    }

    Ok(())
  }
}
