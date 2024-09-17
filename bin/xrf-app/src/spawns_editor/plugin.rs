use crate::spawns_editor::state::SpawnsEditorState;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub struct SpawnsEditorPlugin {}

impl SpawnsEditorPlugin {
  pub const NAME: &'static str = "spawns-editor";

  pub fn init<R: Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new(Self::NAME)
      .setup(|application, _| {
        application.manage(SpawnsEditorState::new());

        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
        crate::spawns_editor::commands::export_spawn_file::export_spawn_file,
        crate::spawns_editor::commands::close_spawn_file::close_spawn_file,
        crate::spawns_editor::commands::get_spawn_file::get_spawn_file,
        crate::spawns_editor::commands::get_spawn_file_alife_spawns::get_spawn_file_alife_spawns,
        crate::spawns_editor::commands::get_spawn_file_artefact_spawns::get_spawn_file_artefact_spawns,
        crate::spawns_editor::commands::get_spawn_file_graphs::get_spawn_file_graphs,
        crate::spawns_editor::commands::get_spawn_file_header::get_spawn_file_header,
        crate::spawns_editor::commands::get_spawn_file_patrols::get_spawn_file_patrols,
        crate::spawns_editor::commands::has_spawn_file::has_spawn_file,
        crate::spawns_editor::commands::import_spawn_file::import_spawn_file,
        crate::spawns_editor::commands::open_spawn_file::open_spawn_file,
        crate::spawns_editor::commands::save_spawn_file::save_spawn_file,
      ])
      .build()
  }
}
