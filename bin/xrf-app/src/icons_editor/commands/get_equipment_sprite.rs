use std::sync::MutexGuard;

use serde_json::{Value, json};
use tauri::State;
use xrf_texture::InventorySpriteDescriptor;

use crate::icons_editor::state::{IconsEditorEquipmentResponse, IconsEditorState};
use crate::types::TauriResult;

#[tauri::command]
pub async fn get_equipment_sprite(state: State<'_, IconsEditorState>) -> TauriResult<Option<Value>> {
  log::debug!("Getting equipment sprite");

  let ltx_path_lock: MutexGuard<Option<String>> = state.system_ltx_path.as_ref().lock().unwrap();
  let dds_path_lock: MutexGuard<Option<String>> = state.equipment_sprite_path.lock().unwrap();
  let name_lock: MutexGuard<Option<String>> = state.equipment_sprite_name.lock().unwrap();
  let equipment_lock: MutexGuard<Option<Vec<InventorySpriteDescriptor>>> = state.equipment_descriptors.lock().unwrap();

  if ltx_path_lock.is_none() || equipment_lock.is_none() || name_lock.is_none() {
    return Ok(None);
  }

  Ok(Some(json!(IconsEditorEquipmentResponse {
    system_ltx_path: ltx_path_lock.as_ref().unwrap().clone(),
    path: dds_path_lock.as_ref().unwrap().clone(),
    name: name_lock.as_ref().unwrap().clone(),
    equipment_descriptors: equipment_lock.as_ref().unwrap().clone(),
  })))
}
