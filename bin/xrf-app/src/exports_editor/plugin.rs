use crate::exports_editor::state::ExportsEditorState;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub struct ExportsEditorPlugin {}

impl ExportsEditorPlugin {
  pub const NAME: &'static str = "exports-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ExportsEditorState::new());

        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
        crate::exports_editor::commands::open_xr_effects::open_xr_effects,
        crate::exports_editor::commands::parse_xr_effects::parse_xr_effects,
        crate::exports_editor::commands::close_xr_effects::close_xr_effects,
        crate::exports_editor::commands::has_xr_effects::has_xr_effects,
        crate::exports_editor::commands::close_xr_exports::close_xr_exports,
        crate::exports_editor::commands::open_xr_exports::open_xr_exports,
        crate::exports_editor::commands::get_xr_effects::get_xr_effects,
        crate::exports_editor::commands::get_xr_exports::get_xr_exports,
      ])
      .build()
  }
}
