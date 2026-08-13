// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod archives_editor;
mod configs_editor;
mod exports_editor;
mod icons_editor;
mod logging;
mod spawns_editor;
mod translations_editor;
mod types;
#[cfg(all(test, feature = "typescript-bindings"))]
mod typescript_bindings;
mod utils;

use std::env;

use env_logger::Builder;
use log::LevelFilter;

use crate::archives_editor::plugin::ArchivesEditorPlugin;
use crate::configs_editor::plugin::ConfigsEditorPlugin;
use crate::exports_editor::plugin::ExportsEditorPlugin;
use crate::icons_editor::plugin::IconsEditorPlugin;
use crate::spawns_editor::plugin::SpawnsEditorPlugin;
use crate::translations_editor::plugin::TranslationsEditorPlugin;

fn main() {
  setup_logger();

  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_shell::init())
    // Custom plugins.
    .plugin(ArchivesEditorPlugin::init())
    .plugin(ExportsEditorPlugin::init())
    .plugin(SpawnsEditorPlugin::init())
    .plugin(ConfigsEditorPlugin::init())
    .plugin(IconsEditorPlugin::init())
    .plugin(TranslationsEditorPlugin::init())
    .run(tauri::generate_context!())
    .expect("Error while running tauri application")
}

/// Configure environment logger, fallback to info level.
pub fn setup_logger() {
  let mut logger: Builder = env_logger::builder();

  if let Ok(rust_log) = env::var("RUST_LOG") {
    logger.parse_filters(&rust_log);
  } else {
    // Debug builds trace every command dispatch; release keeps the info lines the commands already
    // write, which is what makes a user's log useful in a bug report. At the previous warn/error
    // levels none of them were ever emitted, so the logging in the commands did nothing at all.
    match cfg!(debug_assertions) {
      true => logger.filter_level(LevelFilter::Debug),
      false => logger.filter_level(LevelFilter::Info),
    };
  }

  logger.default_format();
  logger.init();
}
