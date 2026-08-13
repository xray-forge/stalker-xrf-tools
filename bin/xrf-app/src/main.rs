// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod archives;
mod configs;
mod equipment_icons;
mod exports;
mod logging;
mod spawn;
mod translations;
mod types;
#[cfg(all(test, feature = "typescript-bindings"))]
mod typescript_bindings;
mod utils;

use std::env;

use env_logger::Builder;
use log::LevelFilter;

use crate::archives::plugin::ArchivesPlugin;
use crate::configs::plugin::ConfigsPlugin;
use crate::equipment_icons::plugin::EquipmentIconsPlugin;
use crate::exports::plugin::ExportsPlugin;
use crate::spawn::plugin::SpawnPlugin;
use crate::translations::plugin::TranslationsPlugin;

fn main() {
  setup_logger();

  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_shell::init())
    // Custom plugins.
    .plugin(ArchivesPlugin::init())
    .plugin(ExportsPlugin::init())
    .plugin(SpawnPlugin::init())
    .plugin(ConfigsPlugin::init())
    .plugin(EquipmentIconsPlugin::init())
    .plugin(TranslationsPlugin::init())
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
