// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod archives_editor;
mod configs_editor;
mod exports_editor;
mod icons_editor;
mod spawns_editor;

use crate::archives_editor::setup::init_archives_editor;
use crate::configs_editor::setup::init_configs_editor;
use crate::exports_editor::setup::init_exports_editor;
use crate::icons_editor::setup::init_icons_editor;
use crate::spawns_editor::setup::init_spawns_editor;
use std::env;

fn main() {
  setup_logger();

  tauri::Builder::default()
    .plugin(init_icons_editor())
    .plugin(init_spawns_editor())
    .plugin(init_exports_editor())
    .plugin(init_configs_editor())
    .plugin(init_archives_editor())
    .run(tauri::generate_context!())
    .expect("Error while running tauri application")
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
