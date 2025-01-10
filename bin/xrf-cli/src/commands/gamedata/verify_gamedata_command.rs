use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgMatches, Command};
use std::path::PathBuf;
use xray_gamedata::{GamedataProject, GamedataProjectOpenOptions};

#[derive(Default)]
pub struct VerifyGamedataCommand;

impl GenericCommand for VerifyGamedataCommand {
  fn name(&self) -> &'static str {
    "verify-gamedata"
  }

  /// Create command to verify gamedata.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to gamedata root")
      .arg(
        Arg::new("root")
          .help("Path gamedata folder")
          .short('r')
          .long("root")
          .required(true)
          .value_delimiter(',')
          .num_args(1..=10)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("configs")
          .help("Path gamedata folder")
          .short('c')
          .long("configs")
          .required(false)
          .value_parser(value_parser!(PathBuf)),
      )
  }

  /// Unpack xray engine database archive.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let roots: Vec<PathBuf> = matches
      .get_many::<PathBuf>("root")
      .expect("Expected valid comma-separated roots to be provided")
      .cloned()
      .collect();

    let config: PathBuf = matches
      .get_one::<PathBuf>("configs")
      .cloned()
      .unwrap_or_else(|| {
        roots
          .first()
          .expect("Expected valid first root item to be provided")
          .join("configs")
      });

    println!("Verifying gamedata");
    println!("Roots: {:?}", roots);
    println!("Configs: {:?}", config);

    let project: Box<GamedataProject> = Box::new(GamedataProject::open(
      &GamedataProjectOpenOptions::new(roots, config),
    )?);

    project.verify()?;

    Ok(())
  }
}
