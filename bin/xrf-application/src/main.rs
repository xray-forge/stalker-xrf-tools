// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod spawn_file;

use crate::spawn_file::{close_spawn_file, get_spawn_file, open_spawn_file, SpawnFileState};
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
  setup_logger();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      get_spawn_file,
      open_spawn_file,
      close_spawn_file
    ])
    .manage(SpawnFileState {
      file: Arc::new(Mutex::new(None)),
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
