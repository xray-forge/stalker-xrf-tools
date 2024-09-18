use crate::translations_editor::state::TranslationsEditorState;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub struct TranslationsEditorPlugin {}

impl TranslationsEditorPlugin {
  pub const NAME: &'static str = "translations-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(TranslationsEditorState::new());

        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
        crate::translations_editor::commands::close_translations_project::close_translations_project,
        crate::translations_editor::commands::get_translations_project::get_translations_project,
        crate::translations_editor::commands::open_translations_project::open_translations_project,
        crate::translations_editor::commands::read_translations_project::read_translations_project,
      ])
      .build()
  }
}
