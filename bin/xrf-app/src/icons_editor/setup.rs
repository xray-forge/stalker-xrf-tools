use crate::icons_editor::state::IconsEditorState;
use std::sync::{Arc, Mutex};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub fn init_icons_editor<R: Runtime>() -> TauriPlugin<R> {
  tauri::plugin::Builder::new("icons_editor")
    .setup(|application| {
      application.manage(IconsEditorState {
        equipment_sprite: Arc::new(Mutex::new(None)),
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      crate::icons_editor::commands::open_equipment_sprite
    ])
    .build()
}
