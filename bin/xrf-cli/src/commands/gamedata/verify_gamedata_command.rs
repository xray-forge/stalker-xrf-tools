use crate::generic_command::{CommandResult, GenericCommand};
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;
use std::path::PathBuf;
use std::process;
use xray_gamedata::{
  GamedataProject, GamedataProjectReadOptions, GamedataProjectVerifyOptions,
  GamedataVerificationResult,
};

#[derive(Default)]
pub struct VerifyGamedataCommand;

impl GenericCommand for VerifyGamedataCommand {
  fn name(&self) -> &'static str {
    "verify-gamedata"
  }

  /// Create command to verify gamedata.
  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to verify gamedata")
      .arg(
        Arg::new("root")
          .help("Paths to gamedata root(s)")
          .short('r')
          .long("root")
          .required(true)
          .value_delimiter(',')
          .num_args(1..=10)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("ignore")
          .help("Ignored assets in gamedata roots")
          .short('i')
          .long("ignore")
          .required(false)
          .value_delimiter(',')
          .num_args(1..=10)
          .value_parser(value_parser!(String)),
      )
      .arg(
        Arg::new("configs")
          .help("Path gamedata folder")
          .short('c')
          .long("configs")
          .required(false)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("silent")
          .help("Turn of logging")
          .long("silent")
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .short('v')
          .long("verbose")
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("strict")
          .help("Turn on strict mode")
          .short('s')
          .long("strict")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  /// Unpack xray engine database archive.
  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let roots: Vec<PathBuf> = matches
      .get_many::<PathBuf>("root")
      .expect("Expected valid comma-separated roots to be provided")
      .cloned()
      .collect();

    let ignored: Vec<String> = matches
      .get_many::<String>("ignore")
      .map(|it| it.cloned().collect::<Vec<String>>())
      .unwrap_or_else(|| {
        vec![
          String::from(".git"),
          String::from(".idea"),
          String::from("particles_unpacked"),
          String::from("textures_unpacked"),
          String::from(".gitignore"),
          String::from(".gitattributes"),
          String::from("README.md"),
          String::from("LICENSE"),
        ]
      });

    let configs: PathBuf = matches
      .get_one::<PathBuf>("configs")
      .cloned()
      .unwrap_or_else(|| {
        roots
          .first()
          .expect("Expected valid first root item to be provided")
          .join("configs")
      });

    let is_silent: bool = matches.get_flag("silent");
    let is_verbose: bool = matches.get_flag("verbose");
    let is_strict: bool = matches.get_flag("strict");

    // todo: Parallel argument, only in silent mode.
    // todo: Modular selection to verify only some elements.

    let open_options: GamedataProjectReadOptions = GamedataProjectReadOptions {
      roots,
      ignored,
      configs,
      is_verbose,
      is_silent,
      is_strict,
    };

    let verify_options: GamedataProjectVerifyOptions = GamedataProjectVerifyOptions {
      is_verbose,
      is_silent,
      is_strict,
    };

    if open_options.is_logging_enabled() {
      println!("{}", "Opening gamedata project".green());
      println!(
        "Roots: {:?}, ignored: {:?}",
        open_options.roots, open_options.ignored,
      );
      println!("Configs: {:?}", open_options.configs);
    }

    let mut project: Box<GamedataProject> = Box::new(GamedataProject::open(&open_options)?);
    let verify_result: GamedataVerificationResult = project.verify(&verify_options)?;

    if verify_result.is_valid() {
      if verify_options.is_logging_enabled() {
        println!(
          "Gamedata project verified in {} sec",
          (verify_result.duration as f64) / 1000.0
        );
        println!("{}", "Project gamedata is valid".green());
      }
    } else {
      if verify_options.is_logging_enabled() {
        println!(
          "Gamedata project checked in {} sec",
          (verify_result.duration as f64) / 1000.0
        );
        println!("{}", "Project gamedata is invalid".red());

        for message in verify_result.get_failure_messages() {
          println!("- {}", message);
        }
      }

      process::exit(1);
    }

    Ok(())
  }
}
