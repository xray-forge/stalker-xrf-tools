use crate::exports_editor::state::{ExportsDeclarations, ExportsEditorState};
use crate::types::TauriResult;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::State;
use xray_export::ExportsParser;

#[tauri::command]
pub async fn open_xr_exports(
  conditions_path: &str,
  dialogs_path: &str,
  effects_path: &str,
  state: State<'_, ExportsEditorState>,
) -> TauriResult<Value> {
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
