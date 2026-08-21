use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

use crate::plugins::visuals::state::VisualState;

pub struct VisualsPlugin {}

impl VisualsPlugin {
  pub const NAME: &'static str = crate::registry::visuals::NAME;

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(VisualState::new());

        Ok(())
      })
      .invoke_handler(crate::app::logging::warn_on_unhandled_command(
        Self::NAME,
        crate::registry::visuals::handler(),
      ))
      .build()
  }

  #[cfg(feature = "typescript-bindings")]
  pub(crate) fn specta_builder<R: Runtime>() -> tauri_specta::Builder<R> {
    crate::registry::visuals::specta_builder()
  }
}
