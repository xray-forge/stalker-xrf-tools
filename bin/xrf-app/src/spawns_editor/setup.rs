use crate::spawns_editor::state::SpawnsEditorState;
use std::sync::{Arc, Mutex};
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub fn init_spawns_editor<R: Runtime>() -> TauriPlugin<R> {
  tauri::plugin::Builder::new("spawns_editor")
    .setup(|application| {
      application.manage(SpawnsEditorState {
        file: Arc::new(Mutex::new(None)),
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      crate::spawns_editor::commands::export_spawn_file,
      crate::spawns_editor::commands::close_spawn_file,
      crate::spawns_editor::commands::get_spawn_file,
      crate::spawns_editor::commands::get_spawn_file_alife_spawns,
      crate::spawns_editor::commands::get_spawn_file_artefact_spawns,
      crate::spawns_editor::commands::get_spawn_file_graphs,
      crate::spawns_editor::commands::get_spawn_file_header,
      crate::spawns_editor::commands::get_spawn_file_patrols,
      crate::spawns_editor::commands::has_spawn_file,
      crate::spawns_editor::commands::import_spawn_file,
      crate::spawns_editor::commands::open_spawn_file,
      crate::spawns_editor::commands::save_spawn_file,
    ])
    .build()
}
