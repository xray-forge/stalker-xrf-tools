use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::plugins::translations::state::TranslationProjectState;

pub struct TranslationsPlugin {}

impl TranslationsPlugin {
  pub const NAME: &'static str = crate::registry::translations::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(TranslationProjectState::new());

        Ok(())
      })
      .invoke_handler(crate::app::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::registry::translations::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::registry::translations::specta_builder()
  }
}
