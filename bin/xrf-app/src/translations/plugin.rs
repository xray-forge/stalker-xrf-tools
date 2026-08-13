use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::translations::state::TranslationProjectState;

pub struct TranslationsPlugin {}

impl TranslationsPlugin {
  pub const NAME: &'static str = "translations";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(TranslationProjectState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::translations::commands::close_project::translations_close_project,
          crate::translations::commands::get_project::translations_get_project,
          crate::translations::commands::open_project::translations_open_project,
          crate::translations::commands::read_project::translations_read_project,
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
        crate::translations::commands::close_project::translations_close_project,
        crate::translations::commands::get_project::translations_get_project,
        crate::translations::commands::open_project::translations_open_project,
        crate::translations::commands::read_project::translations_read_project,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
