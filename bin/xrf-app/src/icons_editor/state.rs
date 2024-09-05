use serde::Serialize;
use std::sync::{Arc, Mutex};
use xray_icon::{ConfigInventorySectionDescriptor, RgbaImage};

pub struct IconsEditorState {
  pub system_ltx_path: Arc<Mutex<Option<String>>>,
  pub equipment_sprite_path: Arc<Mutex<Option<String>>>,
  pub equipment_sprite_name: Arc<Mutex<Option<String>>>,
  pub equipment_sprite: Arc<Mutex<Option<RgbaImage>>>,
  pub equipment_sprite_preview: Arc<Mutex<Option<Vec<u8>>>>,
  pub equipment_descriptors: Arc<Mutex<Option<Vec<ConfigInventorySectionDescriptor>>>>,
}

impl IconsEditorState {
  pub fn new() -> IconsEditorState {
    IconsEditorState {
      system_ltx_path: Arc::new(Mutex::new(None)),
      equipment_sprite_path: Arc::new(Mutex::new(None)),
      equipment_sprite_name: Arc::new(Mutex::new(None)),
      equipment_sprite: Arc::new(Mutex::new(None)),
      equipment_sprite_preview: Arc::new(Mutex::new(None)),
      equipment_descriptors: Arc::new(Mutex::new(None)),
    }
  }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IconsEditorEquipmentResponse {
  pub path: String,
  pub name: String,
  pub system_ltx_path: String,
  pub equipment_descriptors: Vec<ConfigInventorySectionDescriptor>,
}
