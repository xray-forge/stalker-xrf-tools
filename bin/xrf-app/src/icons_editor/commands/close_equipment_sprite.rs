use crate::icons_editor::state::IconsEditorState;
use tauri::State;

#[tauri::command]
pub async fn close_equipment_sprite(state: State<'_, IconsEditorState>) -> Result<(), String> {
  log::info!("Closing equipment file:");

  *state.system_ltx_path.lock().unwrap() = None;
  *state.equipment_sprite_path.lock().unwrap() = None;
  *state.equipment_sprite_name.lock().unwrap() = None;
  *state.equipment_descriptors.lock().unwrap() = None;
  *state.equipment_sprite.lock().unwrap() = None;
  *state.equipment_sprite_preview.lock().unwrap() = None;

  Ok(())
}
