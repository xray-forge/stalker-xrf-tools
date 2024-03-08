use clap::{value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;

/// Setup CLI entrypoint.
pub fn setup_commands() -> Command {
  Command::new("xrf-tool")
    .about("XRF forge CLI tools application")
    .subcommand(
      Command::new("pack-spawn")
        .about("Command to pack unpacked spawn files into single *.spawn")
        .arg(
          Arg::new("path")
            .help("Path to unpacked spawn file folder")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("dest")
            .help("Path to resulting packed *.spawn file")
            .short('d')
            .long("dest")
            .default_value("unpacked")
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("force")
            .help("Whether existing packed spawwn should be pruned if destination folder exists")
            .short('f')
            .long("force")
            .required(false)
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("repack-spawn")
        .about("Command to repack provided *.spawn into another file")
        .arg(
          Arg::new("path")
            .help("Path to *.spawn file")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("dest")
            .help("Path to resulting *.spawn file")
            .short('d')
            .long("dest")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        ),
    )
    .subcommand(
      Command::new("unpack-spawn")
        .about("Command to unpack provided *.spawn into separate files")
        .arg(
          Arg::new("path")
            .help("Path to *.spawn file")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("dest")
            .help("Path to folder for exporting")
            .short('d')
            .long("dest")
            .default_value("unpacked")
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("force")
            .help("Whether existing unpacked data should be pruned if destination folder exists")
            .short('f')
            .long("force")
            .required(false)
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("verify-spawn")
        .about("Command to verify provided *.spawn file")
        .arg(
          Arg::new("path")
            .help("Path to *.spawn file")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        ),
    )
    .subcommand(
      Command::new("info-spawn")
        .about("Command to print information about provided *.spawn file")
        .arg(
          Arg::new("path")
            .help("Path to *.spawn file")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        ),
    )
    .subcommand(
      Command::new("format-ltx")
        .about("Command to format ltx and ini files")
        .arg(
          Arg::new("path")
            .help("Path to ltx file or folder with ltx files")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("silent")
            .help("Turn of formatter logging")
            .short('s')
            .long("silent")
            .required(false)
            .action(ArgAction::SetTrue),
        )
        .arg(
          Arg::new("check")
            .help("Turn of formatter logging")
            .short('c')
            .long("check")
            .required(false)
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("verify-ltx")
        .about("Command to verification ltx and ini files")
        .arg(
          Arg::new("path")
            .help("Path to ltx file or folder with ltx files")
            .short('p')
            .long("path")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("filter")
            .help("Path or mask to include in formatting (works with folders as path)")
            .short('f')
            .long("filter")
            .required(false)
            .value_parser(value_parser!(String)),
        ),
    )
}
