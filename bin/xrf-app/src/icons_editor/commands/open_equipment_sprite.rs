use crate::icons_editor::state::{IconsEditorEquipmentResponse, IconsEditorState};
use crate::types::TauriResult;
use crate::utils::error_to_string;
use serde_json::{json, Value};
use std::path::PathBuf;
use tauri::State;
use xray_ltx::Ltx;
use xray_texture::{open_dds_as_png, InventorySpriteDescriptor};

#[tauri::command]
pub async fn open_equipment_sprite(
  equipment_dds_path: &str,
  system_ltx_path: &str,
  state: State<'_, IconsEditorState>,
) -> TauriResult<Value> {
  log::info!("Opening equipment file: {equipment_dds_path} - {system_ltx_path}");

  let name: &str = "equipment.png";

  let (image, preview_buffer) = open_dds_as_png(&PathBuf::from(equipment_dds_path))
    .map_err(|error| format!("Failed to open provided image file: {:?}", error))?;

  log::info!("Opened equipment dds file");

  let descriptors: Vec<InventorySpriteDescriptor> = InventorySpriteDescriptor::new_list_from_ltx(
    &Ltx::read_from_file_full(system_ltx_path).map_err(error_to_string)?,
  );

  let response = IconsEditorEquipmentResponse {
    system_ltx_path: system_ltx_path.into(),
    path: equipment_dds_path.into(),
    name: name.into(),
    equipment_descriptors: descriptors.clone(),
  };

  *state.system_ltx_path.lock().unwrap() = Some(system_ltx_path.into());
  *state.equipment_sprite_name.lock().unwrap() = Some(name.into());
  *state.equipment_sprite_path.lock().unwrap() = Some(equipment_dds_path.into());
  *state.equipment_sprite.lock().unwrap() = Some(image);
  *state.equipment_sprite_preview.lock().unwrap() = Some(preview_buffer);
  *state.equipment_descriptors.lock().unwrap() = Some(descriptors);

  Ok(json!(response))
}
