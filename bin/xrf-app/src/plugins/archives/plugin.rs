use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::plugins::archives::state::ArchiveProjectState;

pub struct ArchivesPlugin {}

impl ArchivesPlugin {
  pub const NAME: &'static str = crate::registry::archives::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(ArchiveProjectState::new());

        Ok(())
      })
      .invoke_handler(crate::app::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::registry::archives::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::registry::archives::specta_builder()
  }
}
