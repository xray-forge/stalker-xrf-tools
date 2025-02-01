use crate::icons_editor::state::{IconsEditorEquipmentResponse, IconsEditorState};
use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use std::sync::MutexGuard;
use tauri::State;
use xray_ltx::Ltx;
use xray_texture::{open_dds_as_png, InventorySpriteDescriptor};

#[tauri::command]
pub async fn reopen_equipment_sprite(state: State<'_, IconsEditorState>) -> TauriResult<Value> {
  let ltx_path_lock: MutexGuard<Option<String>> = state.system_ltx_path.as_ref().lock().unwrap();
  let dds_path_lock: MutexGuard<Option<String>> = state.equipment_sprite_path.lock().unwrap();
  let dds_name_lock: MutexGuard<Option<String>> =
    state.equipment_sprite_name.as_ref().lock().unwrap();

  if ltx_path_lock.is_none() || dds_path_lock.is_none() || dds_name_lock.is_none() {
    return Err(String::from(
      "Failed to reopen equipment sprites - no active sprite open now",
    ));
  }

  let dds_name: &String = dds_name_lock.as_ref().unwrap();
  let ltx_path: &String = ltx_path_lock.as_ref().unwrap();
  let dds_path: &String = dds_path_lock.as_ref().unwrap();

  let (image, preview_buffer) = open_dds_as_png(dds_path)
    .map_err(|error| format!("Failed to open provided image file: {}", error))?;

  let descriptors: Vec<InventorySpriteDescriptor> = InventorySpriteDescriptor::new_list_from_ltx(
    &Ltx::read_from_file_full(ltx_path).map_err(error_to_string)?,
  );

  let response = IconsEditorEquipmentResponse {
    system_ltx_path: ltx_path.into(),
    path: dds_path.into(),
    name: dds_name.into(),
    equipment_descriptors: descriptors.clone(),
  };

  *state.equipment_sprite.lock().unwrap() = Some(image);
  *state.equipment_sprite_preview.lock().unwrap() = Some(preview_buffer);
  *state.equipment_descriptors.lock().unwrap() = Some(descriptors);

  Ok(json!(response))
}
