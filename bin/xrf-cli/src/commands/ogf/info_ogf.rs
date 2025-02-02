use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{OgfFile, XRayByteOrder};

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

    println!("Read ogf file {}", path.display());

    let ogf_file: Box<OgfFile> = Box::new(OgfFile::read_from_path::<XRayByteOrder, _>(path)?);

    println!("Ogf file information");

    println!(
      "Version: {}, model_type: {}, shader_id: {}, {:?} - {:?}",
      ogf_file.header.version,
      ogf_file.header.model_type,
      ogf_file.header.shader_id,
      ogf_file.header.bounding_box,
      ogf_file.header.bounding_sphere
    );

    println!("Boundaries box: {:?}", ogf_file.header.bounding_box);
    println!("Boundaries sphere: {:?}", ogf_file.header.bounding_sphere);

    if let Some(texture) = &ogf_file.texture {
      println!("Texture name: {}", texture.texture_name);
      println!("Shader name: {}", texture.shader_name);
    }

    if let Some(description) = &ogf_file.description {
      println!("Description: {:?}", description);
    }

    if let Some(bones) = &ogf_file.bones {
      println!("Bones: {}", bones.bones.len());

      for (index, bone) in bones.bones.iter().enumerate() {
        println!("[{}] name: {}", index, bone.name);
        println!("[{}] parent: {}", index, bone.parent);
      }
    }

    if let Some(kinematics) = &ogf_file.kinematics {
      println!("Motion refs: {:?}", kinematics.motion_refs);
    }

    if let Some(children) = &ogf_file.children {
      println!("OGF children ({}):", children.nested.len());

      for (index, child) in children.nested.iter().enumerate() {
        if let Some(texture) = &child.texture {
          println!("[{}] texture name: {}", index, texture.texture_name);
          println!("[{}] shader name: {}", index, texture.shader_name);
        }
      }
    }

    Ok(())
  }
}
