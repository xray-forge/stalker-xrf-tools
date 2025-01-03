use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::particles_file::particles_file::ParticlesFile;
use xray_db::types::ParticlesByteOrder;

pub struct VerifyParticlesFileCommand {}

impl VerifyParticlesFileCommand {
  pub const NAME: &'static str = "verify-particles";

  /// Create command for verifying of particles file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to verify provided particles.xr file")
      .arg(
        Arg::new("path")
          .help("Path to particles.xr file")
          .short('p')
          .long("path")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("unpacked")
          .help("Whether should verify unpacked particles")
          .short('u')
          .long("unpacked")
          .required(false)
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Verify particles file based on provided arguments.
  pub fn execute(matches: &ArgMatches) {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let unpacked: bool = matches.get_flag("unpacked");

    log::info!("Verify particles file {:?}, unpacked: {unpacked}", path);

    let particles_file = if unpacked {
      ParticlesFile::import_from_path(path)
    } else {
      ParticlesFile::read_from_path::<ParticlesByteOrder>(path)
    };

    match particles_file {
      Ok(_) => log::info!("Provided particles file is valid"),
      Err(error) => {
        log::error!("Provided particles file is invalid: {:?}", error);
        panic!("{:?}", error);
      }
    }
  }
}
