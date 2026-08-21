use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::plugins::spawn::state::SpawnFileState;

pub struct SpawnPlugin {}

impl SpawnPlugin {
  pub const NAME: &'static str = crate::registry::spawn::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(SpawnFileState::new());

        Ok(())
      })
      .invoke_handler(crate::app::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::registry::spawn::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::registry::spawn::specta_builder()
  }
}
