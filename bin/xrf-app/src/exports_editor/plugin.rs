use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::exports_editor::state::ExportsEditorState;

pub struct ExportsEditorPlugin {}

impl ExportsEditorPlugin {
  pub const NAME: &'static str = "exports-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ExportsEditorState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::exports_editor::commands::close_xr_exports::close_xr_exports,
          crate::exports_editor::commands::open_xr_exports::open_xr_exports,
          crate::exports_editor::commands::get_xr_exports::get_xr_exports,
          crate::exports_editor::commands::get_xr_export_source::get_xr_export_source,
        ],
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    tauri_specta::Builder::new()
      .plugin_name(Self::NAME)
      .error_handling(tauri_specta::ErrorHandlingMode::Throw)
      .commands(tauri_specta::collect_commands![
        crate::exports_editor::commands::close_xr_exports::close_xr_exports,
        crate::exports_editor::commands::open_xr_exports::open_xr_exports,
        crate::exports_editor::commands::get_xr_exports::get_xr_exports,
        crate::exports_editor::commands::get_xr_export_source::get_xr_export_source,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
