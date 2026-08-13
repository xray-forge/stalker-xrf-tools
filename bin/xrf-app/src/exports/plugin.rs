use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::exports::state::ExportsProjectState;

pub struct ExportsPlugin {}

impl ExportsPlugin {
  pub const NAME: &'static str = "exports";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ExportsProjectState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::exports::commands::close_project::exports_close_project,
          crate::exports::commands::open_project::exports_open_project,
          crate::exports::commands::get_project::exports_get_project,
          crate::exports::commands::get_source::exports_get_source,
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
        crate::exports::commands::close_project::exports_close_project,
        crate::exports::commands::open_project::exports_open_project,
        crate::exports::commands::get_project::exports_get_project,
        crate::exports::commands::get_source::exports_get_source,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
