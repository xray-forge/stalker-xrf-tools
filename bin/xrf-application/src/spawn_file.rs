use std::path::Path;
use std::sync::{Arc, Mutex};
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

pub struct SpawnFileState {
  pub file: Arc<Mutex<Option<SpawnFile>>>,
}

#[tauri::command]
pub fn get_spawn_file(state: tauri::State<'_, SpawnFileState>) -> Option<String> {
  let lock = state.file.lock().unwrap();

  if (*lock).is_none() {
    return None;
  }

  Some(String::from("existing"))
}

#[tauri::command]
pub async fn open_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnFileState>,
) -> Result<String, String> {
  log::info!("Opening spawn file");

  match SpawnFile::read_from_path::<SpawnByteOrder>(Path::new(path)) {
    Ok(file) => {
      log::info!("Opened spawn file");

      *state.file.lock().unwrap() = Some(file);

      Ok(String::from("Opened spawn file"))
    }
    Err(_) => Err(String::from("Failed to open provided spawn file")),
  }
}

#[tauri::command]
pub fn close_spawn_file(state: tauri::State<'_, SpawnFileState>) {
  log::info!("Closing spawn file");

  if state.file.lock().unwrap().is_some() {
    *state.file.lock().unwrap() = None;
  }
}
