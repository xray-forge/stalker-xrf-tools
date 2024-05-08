use crate::archives_editor::state::ArchivesEditorState;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::sync::MutexGuard;
use xray_archive::ArchiveProject;

#[tauri::command]
pub async fn unpack_archives_path(from: &str, destination: &str) -> Result<Value, String> {
  log::info!("Open archive folder: {:?}", from);

  let project: ArchiveProject = match ArchiveProject::new(&PathBuf::from(from)) {
    Ok(project) => project,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Unpacking archive to: {:?}", destination);

  match project
    .unpack_parallel(&PathBuf::from(destination), 32)
    .await
  {
    Ok(result) => Ok(json!(result)),
    Err(error) => Err(error.to_string()),
  }
}

#[tauri::command]
pub async fn open_archives_project(
  path: &str,
  state: tauri::State<'_, ArchivesEditorState>,
) -> Result<Value, String> {
  log::info!("Opening archives project");

  match ArchiveProject::new(Path::new(path)) {
    Ok(project) => {
      log::info!("Opened archives project");

      let json: Value = json!(project);

      *state.project.lock().unwrap() = Some(project);

      Ok(json)
    }
    Err(_) => Err(String::from("Failed to open provided archive project")),
  }
}

#[tauri::command]
pub fn close_archives_project(state: tauri::State<'_, ArchivesEditorState>) {
  log::info!("Closing archives project");

  let mut lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }
}

#[tauri::command]
pub fn has_archives_project(state: tauri::State<'_, ArchivesEditorState>) -> bool {
  state.project.lock().unwrap().is_some()
}

#[tauri::command]
pub async fn get_archives_project(
  state: tauri::State<'_, ArchivesEditorState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

#[tauri::command]
pub async fn read_archive_file(
  path: &str,
  state: tauri::State<'_, ArchivesEditorState>,
) -> Result<Value, String> {
  let lock: MutexGuard<Option<ArchiveProject>> = state.project.lock().unwrap();

  if (*lock).is_none() {
    return Err(String::from("Failed to read file - archive is not open"));
  }

  lock
    .as_ref()
    .unwrap()
    .read_file_as_string(path)
    .map(|result| json!(result))
    .map_err(|error| error.to_string())
}
