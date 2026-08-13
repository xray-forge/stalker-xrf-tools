use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

use crate::spawn::state::SpawnFileState;

pub struct SpawnPlugin {}

impl SpawnPlugin {
  pub const NAME: &'static str = "spawn";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(SpawnFileState::new());

        Ok(())
      })
      .invoke_handler(crate::logging::warn_on_unhandled_command(
        Self::NAME,
        tauri::generate_handler![
          crate::spawn::commands::save_unpacked_directory::spawn_save_unpacked_directory,
          crate::spawn::commands::close_file::spawn_close_file,
          crate::spawn::commands::get_file::spawn_get_file,
          crate::spawn::commands::get_alife_spawns::spawn_get_alife_spawns,
          crate::spawn::commands::get_artefact_spawns::spawn_get_artefact_spawns,
          crate::spawn::commands::get_graphs::spawn_get_graphs,
          crate::spawn::commands::get_header::spawn_get_header,
          crate::spawn::commands::get_patrols::spawn_get_patrols,
          crate::spawn::commands::get_path::spawn_get_path,
          crate::spawn::commands::has_file::spawn_has_file,
          crate::spawn::commands::open_unpacked_directory::spawn_open_unpacked_directory,
          crate::spawn::commands::open_file::spawn_open_file,
          crate::spawn::commands::pack_file::spawn_pack_file,
          crate::spawn::commands::save_file::spawn_save_file,
          crate::spawn::commands::unpack_file::spawn_unpack_file,
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
        crate::spawn::commands::save_unpacked_directory::spawn_save_unpacked_directory,
        crate::spawn::commands::close_file::spawn_close_file,
        crate::spawn::commands::get_file::spawn_get_file,
        crate::spawn::commands::get_alife_spawns::spawn_get_alife_spawns,
        crate::spawn::commands::get_artefact_spawns::spawn_get_artefact_spawns,
        crate::spawn::commands::get_graphs::spawn_get_graphs,
        crate::spawn::commands::get_header::spawn_get_header,
        crate::spawn::commands::get_patrols::spawn_get_patrols,
        crate::spawn::commands::get_path::spawn_get_path,
        crate::spawn::commands::has_file::spawn_has_file,
        crate::spawn::commands::open_unpacked_directory::spawn_open_unpacked_directory,
        crate::spawn::commands::open_file::spawn_open_file,
        crate::spawn::commands::pack_file::spawn_pack_file,
        crate::spawn::commands::save_file::spawn_save_file,
        crate::spawn::commands::unpack_file::spawn_unpack_file,
      ])
      .disable_serde_phases()
      .dangerously_cast_bigints_to_number()
  }
}
