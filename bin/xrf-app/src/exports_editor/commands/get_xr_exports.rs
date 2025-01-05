use crate::exports_editor::state::{ExportsDeclarations, ExportsEditorState};
use crate::types::TauriResult;
use serde_json::{json, Value};
use tauri::State;
use xray_export::ExportDescriptor;

#[tauri::command]
pub async fn get_xr_exports(state: State<'_, ExportsEditorState>) -> TauriResult<Option<Value>> {
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
