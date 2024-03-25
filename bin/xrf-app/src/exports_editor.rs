use serde::Serialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use xray_export::{ExportDescriptor, ExportsParser};

pub struct ExportsProjectState {
  pub conditions: Arc<Mutex<Option<Vec<ExportDescriptor>>>>,
  pub dialogs: Arc<Mutex<Option<Vec<ExportDescriptor>>>>,
  pub effects: Arc<Mutex<Option<Vec<ExportDescriptor>>>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportsDeclarations {
  pub conditions: Vec<ExportDescriptor>,
  pub dialogs: Vec<ExportDescriptor>,
  pub effects: Vec<ExportDescriptor>,
}

#[tauri::command]
pub async fn open_xr_exports(
  conditions_path: &str,
  dialogs_path: &str,
  effects_path: &str,
  state: tauri::State<'_, ExportsProjectState>,
) -> Result<Value, String> {
  log::info!("Parsing exports folders: {conditions_path} + {dialogs_path} + {effects_path}");

  let parser: ExportsParser = ExportsParser::new();

  let declaration: ExportsDeclarations = ExportsDeclarations {
    conditions: parser
      .parse_conditions_from_path(&PathBuf::from(conditions_path))
      .map_err(|x| x.to_string())?,
    dialogs: parser
      .parse_dialogs_from_path(&PathBuf::from(dialogs_path))
      .map_err(|x| x.to_string())?,
    effects: parser
      .parse_effects_from_path(&PathBuf::from(effects_path))
      .map_err(|x| x.to_string())?,
  };

  let json: Value = json!(declaration);

  *state.conditions.lock().unwrap() = Some(declaration.conditions);
  *state.dialogs.lock().unwrap() = Some(declaration.dialogs);
  *state.effects.lock().unwrap() = Some(declaration.effects);

  Ok(json)
}

#[tauri::command]
pub fn close_xr_exports(state: tauri::State<'_, ExportsProjectState>) {
  log::info!("Closing xr exports");

  let mut lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }
}

#[tauri::command]
pub async fn parse_xr_effects(path: &str) -> Result<Value, String> {
  log::info!("Parsing effects exports folder: {:?}", path);

  let parser: ExportsParser = ExportsParser::new();

  match parser.parse_effects_from_path(&PathBuf::from(path)) {
    Ok(value) => Ok(json!(value)),
    Err(error) => Err(error.to_string()),
  }
}

#[tauri::command]
pub async fn open_xr_effects(
  path: &str,
  state: tauri::State<'_, ExportsProjectState>,
) -> Result<Value, String> {
  log::info!("Parsing effects exports folder: {:?}", path);

  let parser: ExportsParser = ExportsParser::new();

  match parser.parse_effects_from_path(&PathBuf::from(path)) {
    Ok(value) => {
      let json: Value = json!(value);

      *state.effects.lock().unwrap() = Some(value);

      Ok(json)
    }
    Err(error) => Err(error.to_string()),
  }
}

#[tauri::command]
pub fn close_xr_effects(state: tauri::State<'_, ExportsProjectState>) {
  log::info!("Closing xr effects project");

  let mut lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if lock.is_some() {
    *lock = None;
  }
}

#[tauri::command]
pub fn has_xr_effects(state: tauri::State<'_, ExportsProjectState>) -> bool {
  state.effects.lock().unwrap().is_some()
}

#[tauri::command]
pub async fn get_xr_effects(
  state: tauri::State<'_, ExportsProjectState>,
) -> Result<Option<Value>, String> {
  let lock: MutexGuard<Option<Vec<ExportDescriptor>>> = state.effects.lock().unwrap();

  if (*lock).is_none() {
    return Ok(None);
  }

  Ok(Some(json!(lock.as_ref().unwrap())))
}

#[tauri::command]
pub async fn get_xr_exports(
  state: tauri::State<'_, ExportsProjectState>,
) -> Result<Option<Value>, String> {
  let conditions: Option<Vec<ExportDescriptor>> =
    state.conditions.lock().unwrap().as_ref().cloned();
  let dialogs: Option<Vec<ExportDescriptor>> = state.dialogs.lock().unwrap().as_ref().cloned();
  let effects: Option<Vec<ExportDescriptor>> = state.effects.lock().unwrap().as_ref().cloned();

  if conditions.is_some() && dialogs.is_some() && effects.is_some() {
    Ok(Some(json!(ExportsDeclarations {
      conditions: conditions.unwrap(),
      dialogs: dialogs.unwrap(),
      effects: effects.unwrap(),
    })))
  } else {
    Ok(None)
  }
}
