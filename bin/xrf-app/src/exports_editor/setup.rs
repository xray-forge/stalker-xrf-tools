use crate::exports_editor::state::ExportsEditorState;
use std::sync::{Arc, Mutex};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub fn init_exports_editor<R: Runtime>() -> TauriPlugin<R> {
  tauri::plugin::Builder::new("exports_editor")
    .setup(|application| {
      application.manage(ExportsEditorState {
        conditions: Arc::new(Mutex::new(None)),
        dialogs: Arc::new(Mutex::new(None)),
        effects: Arc::new(Mutex::new(None)),
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      crate::exports_editor::commands::open_xr_effects,
      crate::exports_editor::commands::parse_xr_effects,
      crate::exports_editor::commands::close_xr_effects,
      crate::exports_editor::commands::has_xr_effects,
      crate::exports_editor::commands::close_xr_exports,
      crate::exports_editor::commands::open_xr_exports,
      crate::exports_editor::commands::get_xr_effects,
      crate::exports_editor::commands::get_xr_exports,
    ])
    .build()
}
