use crate::spawns_editor::state::SpawnsEditorState;
use serde_json::{json, Value};
use std::path::Path;
use std::sync::MutexGuard;
use xray_db::file::spawn_file::SpawnFile;
use xray_db::types::SpawnByteOrder;

#[tauri::command]
pub async fn open_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Value, String> {
  log::info!("Opening spawn file");

  match SpawnFile::read_from_path::<SpawnByteOrder>(Path::new(path)) {
    Ok(file) => {
      log::info!("Opened spawn file");

      let json: Value = json!(file);

      *state.file.lock().unwrap() = Some(file);

      Ok(json)
    }
    Err(_) => Err(String::from("Failed to open provided spawn file")),
  }
}

#[tauri::command]
pub async fn import_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnsEditorState>,
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
pub fn save_spawn_file(
  path: &str,
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<(), String> {
  log::info!("Saving spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

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
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<(), String> {
  log::info!("Saving spawn file");

  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

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
pub fn close_spawn_file(state: tauri::State<'_, SpawnsEditorState>) {
  log::info!("Closing spawn file");

  let mut lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }
}

#[tauri::command]
pub fn has_spawn_file(state: tauri::State<'_, SpawnsEditorState>) -> bool {
  state.file.lock().unwrap().is_some()
}

#[tauri::command]
pub async fn get_spawn_file(
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

#[tauri::command]
pub async fn get_spawn_file_header(
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().header)))
}

#[tauri::command]
pub async fn get_spawn_file_patrols(
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().patrols)))
}

#[tauri::command]
pub async fn get_spawn_file_artefact_spawns(
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().artefact_spawn)))
}

#[tauri::command]
pub async fn get_spawn_file_alife_spawns(
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().alife_spawn)))
}

#[tauri::command]
pub async fn get_spawn_file_graphs(
  state: tauri::State<'_, SpawnsEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<SpawnFile>> = state.file.lock().unwrap();

  if lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap().graphs)))
}
