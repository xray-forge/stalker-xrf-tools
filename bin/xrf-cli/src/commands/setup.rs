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
      Command::new("unpack-archive")
        .about("Command to unpack provided *.db into separate files")
        .arg(
          Arg::new("path")
            .help("Path to *.db file")
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
          Arg::new("parallel")
            .help("Count of parallel threads for unpack")
            .long("parallel")
            .default_value("32")
            .value_parser(value_parser!(usize)),
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
          Arg::new("silent")
            .help("Turn of formatter logging")
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
            .help("Turn on strict checking mode")
            .short('s')
            .long("strict")
            .required(false)
            .action(ArgAction::SetTrue),
        ),
    )
    .subcommand(
      Command::new("pack-inventory-icons")
        .about("Command to pack dds icons into single element")
        .arg(
          Arg::new("system-ltx")
            .help("Path to system ltx file or root folder with ltx files")
            .long("system-ltx")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("source")
            .help("Path to source folder with section icons")
            .long("source")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("output")
            .help("Path to output dds file")
            .long("output")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("gamedata")
            .help("Path to gamedata folder for resources usage")
            .long("gamedata")
            .required(false)
            .value_parser(value_parser!(PathBuf)),
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
        ),
    )
    .subcommand(
      Command::new("unpack-inventory-icons")
        .about("Command to unpack dds icons into multiple icons")
        .arg(
          Arg::new("system-ltx")
            .help("Path to system ltx file or root folder with ltx files")
            .long("system-ltx")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("source")
            .help("Path to source dds file")
            .long("source")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("output")
            .help("Path to output folder for sections icons")
            .long("output")
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
          Arg::new("verbose")
            .help("Turn on verbose logging")
            .short('v')
            .long("verbose")
            .required(false)
            .action(ArgAction::SetTrue),
        ),
    )
}
