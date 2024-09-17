// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod archives_editor;
mod configs_editor;
mod exports_editor;
mod icons_editor;
mod spawns_editor;
mod translations_editor;

use crate::archives_editor::plugin::ArchivesEditorPlugin;
use crate::configs_editor::plugin::ConfigsEditorPlugin;
use crate::exports_editor::plugin::ExportsEditorPlugin;
use crate::icons_editor::plugin::IconsEditorPlugin;
use crate::spawns_editor::plugin::SpawnsEditorPlugin;
use crate::translations_editor::plugin::TranslationsEditorPlugin;
use std::env;

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
  if env::var("RUST_LOG").is_err() {
    unsafe {
      env::set_var(
        "RUST_LOG",
        match cfg!(debug_assertions) {
          true => "info",
          false => "error",
        },
      )
    }
  }

  env_logger::init();
}
