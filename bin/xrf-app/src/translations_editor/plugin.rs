use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::translations_editor::state::TranslationsEditorState;

pub struct TranslationsEditorPlugin {}

impl TranslationsEditorPlugin {
  pub const NAME: &'static str = "translations-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(TranslationsEditorState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::translations_editor::commands::close_translations_project::close_translations_project,
          crate::translations_editor::commands::get_translations_project::get_translations_project,
          crate::translations_editor::commands::open_translations_project::open_translations_project,
          crate::translations_editor::commands::read_translations_project::read_translations_project,
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
        crate::translations_editor::commands::close_translations_project::close_translations_project,
        crate::translations_editor::commands::get_translations_project::get_translations_project,
        crate::translations_editor::commands::open_translations_project::open_translations_project,
        crate::translations_editor::commands::read_translations_project::read_translations_project,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
