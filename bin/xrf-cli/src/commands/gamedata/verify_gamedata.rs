use crate::generic_command::{CommandResult, GenericCommand};
use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use colored::Colorize;
use std::path::PathBuf;
use std::process;
use xray_gamedata::{
  GamedataProject, GamedataProjectReadOptions, GamedataProjectVerifyOptions,
  GamedataVerificationResult, GamedataVerificationStatus, GamedataVerificationType,
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
          .help("Path to assembled gamedata root")
          .required(true)
          .value_name("ROOT")
          .num_args(1)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("ignore")
          .help("Ignored assets in the gamedata root")
          .short('i')
          .long("ignore")
          .required(false)
          .value_delimiter(',')
          .num_args(1..=10)
          .value_parser(value_parser!(String)),
      )
      .arg(
        Arg::new("checks")
          .help("List of checks to perform")
          .long("checks")
          .value_delimiter(',')
          .num_args(0..=15)
          .value_parser(value_parser!(GamedataVerificationType)),
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
    let root: PathBuf = matches
      .get_one::<PathBuf>("root")
      .expect("Expected a valid gamedata root to be provided")
      .clone();

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

    let checks: Vec<GamedataVerificationType> = matches
      .get_many::<GamedataVerificationType>("checks")
      .map(|it| it.cloned().collect::<Vec<_>>())
      .unwrap_or_else(GamedataVerificationType::get_all);

    let is_silent: bool = matches.get_flag("silent");
    let is_verbose: bool = matches.get_flag("verbose");
    let is_strict: bool = matches.get_flag("strict");

    let open_options: GamedataProjectReadOptions = GamedataProjectReadOptions {
      root,
      ignored,
      is_verbose,
      is_silent,
      is_strict,
    };

    let verify_options: GamedataProjectVerifyOptions = GamedataProjectVerifyOptions {
      is_verbose,
      is_silent,
      is_strict,
      checks,
    };

    if open_options.is_logging_enabled() {
      println!("{}", "Opening gamedata project".green());
      println!(
        "Root: {}, ignored: [{}]",
        open_options.root.display(),
        open_options.ignored.join(", "),
      );
    }

    let mut project: Box<GamedataProject> = Box::new(GamedataProject::open(&open_options)?);
    let verify_result: GamedataVerificationResult = project.verify(&verify_options)?;
    let status: GamedataVerificationStatus = verify_result.status();

    if verify_options.is_logging_enabled() {
      match status {
        GamedataVerificationStatus::Passed => {
          println!();
          println!("{}", "Project gamedata is valid".green());
          println!(
            "Gamedata project verified in {} sec",
            (verify_result.duration as f64) / 1000.0
          );
        }
        GamedataVerificationStatus::Failed
        | GamedataVerificationStatus::Error
        | GamedataVerificationStatus::Incomplete
        | GamedataVerificationStatus::Skipped => {
          eprintln!();

          let status_message = match status {
            GamedataVerificationStatus::Failed => "Project gamedata is invalid".red(),
            GamedataVerificationStatus::Error => "Project gamedata verification has errors".red(),
            GamedataVerificationStatus::Incomplete => {
              "Project gamedata verification is incomplete".yellow()
            }
            GamedataVerificationStatus::Skipped => {
              "Project gamedata verification was skipped".yellow()
            }
            GamedataVerificationStatus::Passed => unreachable!(),
          };

          eprintln!("{status_message}");

          for message in verify_result.get_failure_messages() {
            eprintln!("- {message}");
          }

          eprintln!(
            "Gamedata project checked in {} sec",
            (verify_result.duration as f64) / 1000.0
          );
        }
      }
    }

    if status != GamedataVerificationStatus::Passed {
      process::exit(1);
    }

    Ok(())
  }
}
