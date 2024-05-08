use std::sync::{Arc, Mutex};
use xray_icon::RgbaImage;

pub struct IconsEditorState {
  pub equipment_sprite: Arc<Mutex<Option<RgbaImage>>>,
}
