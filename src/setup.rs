use clap::{value_parser, Arg, Command};
use std::env;
use std::path::PathBuf;

/// Configure environment logger, fallback to info level.
pub fn setup_logger() {
  if env::var("RUST_LOG").is_err() {
    env::set_var(
      "RUST_LOG",
      match cfg!(debug_assertions) {
        true => "info",
        false => "error",
      },
    )
  }

  env_logger::init();
}

/// Setup CLI entrypoint.
pub fn setup_cli() -> Command {
  Command::new("xrf-tool")
    .about("XRF forge CLI tools application.")
    .subcommand(
      Command::new("pack")
        .about("Command to pack unpacked spawn files into single *.spawn.")
        .arg(
          Arg::new("path-spawn")
            .help("Path to unpacked spawn assets.")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        ),
    )
    .subcommand(
      Command::new("unpack-spawn")
        .about("Command to unpack provided *.spawn into separate files.")
        .arg(
          Arg::new("path")
            .help("Path to *.spawn file.")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        ),
    )
    .subcommand(
      Command::new("verify-spawn")
        .about("Command to verify provided *.spawn file.")
        .arg(
          Arg::new("path")
            .help("Path to *.spawn file.")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        ),
    )
    .subcommand(
      Command::new("format-ltx")
        .about("Command to format ltx and ini files.")
        .arg(
          Arg::new("path")
            .help("Path to ltx file or folder with ltx files.")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("filter")
            .help("Path or mask to include in formatting (works with folders as path).")
            .short('f')
            .long("filter")
            .required(false)
            .value_parser(value_parser!(String)),
        ),
    )
}
