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
pub async fn import_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnFileState>,
) -> Result<String, String> {
  log::info!("Importing spawn file");

  match SpawnFile::import_from_path::<SpawnByteOrder>(Path::new(path)) {
    Ok(file) => {
      log::info!("Imported spawn file");

      *state.file.lock().unwrap() = Some(file);

      Ok(String::from("Imported spawn file"))
    }
    Err(_) => Err(String::from("Failed to import provided spawn file pat")),
  }
}

#[tauri::command]
pub fn save_spawn_file(path: &str, state: tauri::State<'_, SpawnFileState>) -> Result<(), String> {
  log::info!("Saving spawn file");

  let lock = state.file.lock().unwrap();

  if lock.is_some() {
    let file: &SpawnFile = lock.as_ref().unwrap();

    match file.write_to_path::<SpawnByteOrder>(Path::new(path)) {
      Ok(_) => Ok(()),
      Err(error) => Err(error.to_string()),
    }
  } else {
    Err(String::from("No spawn file open for saving"))
  }
}

#[tauri::command]
pub async fn export_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnFileState>,
) -> Result<(), String> {
  log::info!("Saving spawn file");

  let lock = state.file.lock().unwrap();

  if lock.is_some() {
    let file: &SpawnFile = lock.as_ref().unwrap();

    match file.export_to_path::<SpawnByteOrder>(Path::new(path)) {
      Ok(_) => Ok(()),
      Err(error) => Err(error.to_string()),
    }
  } else {
    Err(String::from("No spawn file open for saving"))
  }
}

#[tauri::command]
pub fn close_spawn_file(state: tauri::State<'_, SpawnFileState>) {
  log::info!("Closing spawn file");

  if state.file.lock().unwrap().is_some() {
    *state.file.lock().unwrap() = None;
  }
}
