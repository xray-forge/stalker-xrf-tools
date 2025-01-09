use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use std::path::PathBuf;
use xray_db::{DatabaseParseError, DatabaseResult, ParticlesFile, XRayByteOrder};

pub struct VerifyParticlesFileCommand {}

impl VerifyParticlesFileCommand {
  pub const NAME: &'static str = "verify-particles";

  /// Create command for verifying of particles file.
  pub fn init() -> Command {
    Command::new(Self::NAME)
      .about("Command to verify provided particles file")
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
  pub fn execute(matches: &ArgMatches) -> DatabaseResult {
    let path: &PathBuf = matches
      .get_one::<PathBuf>("path")
      .expect("Expected valid path to be provided");

    let unpacked: bool = matches.get_flag("unpacked");

    log::info!("Verify particles file {:?}, unpacked: {unpacked}", path);

    let particles_file_result: DatabaseResult<ParticlesFile> = if unpacked {
      ParticlesFile::import_from_path(path)
    } else {
      ParticlesFile::read_from_path::<XRayByteOrder>(path)
    };

    match particles_file_result {
      Ok(_) => {
        log::info!("Provided particles file is valid");

        Ok(())
      }
      Err(error) => {
        log::error!("Provided particles file is invalid: {}", error);

        Err(DatabaseParseError::new_database_error(format!(
          "Verification of particles file failed: {}",
          error
        )))
      }
    }
  }
}
