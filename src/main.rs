mod commands;
mod setup;

use crate::commands::{
  format_ltx, pack_spawn_file, repack_spawn_file, unpack_archive, unpack_spawn_file,
  verify_spawn_file,
};
use crate::setup::{setup_cli, setup_logger};

fn main() {
  setup_logger();

  match setup_cli().get_matches().subcommand() {
    Some(("repack-spawn", matches)) => repack_spawn_file(matches).unwrap(),
    Some(("unpack-spawn", matches)) => unpack_spawn_file(matches).unwrap(),
    Some(("unpack-archive", matches)) => unpack_archive(matches),
    Some(("pack-spawn", matches)) => pack_spawn_file(matches),
    Some(("verify-spawn", matches)) => verify_spawn_file(matches),
    Some(("format-ltx", matches)) => format_ltx(matches),
    _ => panic!("Unexpected cli command provided, check --help for details."),
  }
}
