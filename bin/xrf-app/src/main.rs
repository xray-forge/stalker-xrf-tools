// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod archive_editor;
mod configs_editor;
mod spawn_editor;

use crate::archive_editor::{
  close_archives_project, get_archives_project, has_archives_project, open_archives_project,
  unpack_archives_path, ArchivesProjectState,
};
use crate::configs_editor::{check_format_configs_path, format_configs_path, verify_configs_path};
use crate::spawn_editor::{
  close_spawn_file, export_spawn_file, get_spawn_file, get_spawn_file_alife_spawns,
  get_spawn_file_artefact_spawns, get_spawn_file_graphs, get_spawn_file_header,
  get_spawn_file_patrols, has_spawn_file, import_spawn_file, open_spawn_file, save_spawn_file,
  SpawnFileState,
};
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
  setup_logger();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      check_format_configs_path,
      close_archives_project,
      close_spawn_file,
      export_spawn_file,
      format_configs_path,
      get_archives_project,
      get_spawn_file,
      get_spawn_file_alife_spawns,
      get_spawn_file_artefact_spawns,
      get_spawn_file_graphs,
      get_spawn_file_header,
      get_spawn_file_patrols,
      has_archives_project,
      has_spawn_file,
      import_spawn_file,
      open_archives_project,
      open_spawn_file,
      save_spawn_file,
      unpack_archives_path,
      verify_configs_path,
    ])
    .manage(SpawnFileState {
      file: Arc::new(Mutex::new(None)),
    })
    .manage(ArchivesProjectState {
      project: Arc::new(Mutex::new(None)),
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
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
