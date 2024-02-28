mod commands;

use crate::commands::format_ltx::format_ltx;
use crate::commands::pack_spawn_file::pack_spawn_file;
use crate::commands::repack_spawn_file::repack_spawn_file;
use crate::commands::setup::setup_commands;
use crate::commands::unpack_archive::unpack_archive;
use crate::commands::unpack_spawn_file::unpack_spawn_file;
use crate::commands::verify_spawn_file::verify_spawn_file;
use std::env;

fn main() {
  setup_logger();

  match setup_commands().get_matches().subcommand() {
    Some(("verify-spawn", matches)) => verify_spawn_file(matches),
    Some(("unpack-spawn", matches)) => unpack_spawn_file(matches).unwrap(),
    Some(("repack-spawn", matches)) => repack_spawn_file(matches).unwrap(),
    Some(("pack-spawn", matches)) => pack_spawn_file(matches).unwrap(),
    Some(("unpack-archive", matches)) => unpack_archive(matches),
    Some(("format-ltx", matches)) => format_ltx(matches),
    _ => panic!("Unexpected cli command provided, check --help for details."),
  };
}

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
