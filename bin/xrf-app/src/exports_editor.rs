use serde::Serialize;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use xray_export::{EffectsParser, ExportDescriptor};

pub struct ExportsProjectState {
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

  let parser: EffectsParser = match EffectsParser::new(&PathBuf::from(effects_path)) {
    Ok(parser) => parser,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Parsing xr_effects exports from folder: {:?}", effects_path);

  let declaration: ExportsDeclarations = ExportsDeclarations {
    conditions: Vec::new(),
    dialogs: Vec::new(),
    effects: match parser.parse_effects() {
      Ok(value) => value,
      Err(error) => return Err(error.to_string()),
    },
  };

  let json: Value = json!(declaration);

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

  let parser: EffectsParser = match EffectsParser::new(&PathBuf::from(path)) {
    Ok(parser) => parser,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Parsing xr_effects exports from folder: {:?}", path);

  match parser.parse_effects() {
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

  let parser: EffectsParser = match EffectsParser::new(&PathBuf::from(path)) {
    Ok(parser) => parser,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Parsing xr_effects exports from folder: {:?}", path);

  match parser.parse_effects() {
    Ok(value) => {
      let json: Value = json!(value);

      *state.effects.lock().unwrap() = Some(value);

      Ok(json)
    }
    Err(error) => return Err(error.to_string()),
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
