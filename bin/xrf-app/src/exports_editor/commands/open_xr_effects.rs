use crate::exports_editor::state::ExportsEditorState;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::State;
use xray_export::ExportsParser;

#[tauri::command]
pub async fn open_xr_effects(
  path: &str,
  state: State<'_, ExportsEditorState>,
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
