use crate::icons_editor::state::IconsEditorState;
use crate::icons_editor::stream::get_equipment_sprite_stream_response;
use http::header::CONTENT_TYPE;
use http::StatusCode;
use std::sync::{Arc, Mutex};
use tauri::http::ResponseBuilder;
use tauri::plugin::TauriPlugin;
use tauri::{Manager, Runtime};

pub fn init_icons_editor<R: Runtime>() -> TauriPlugin<R> {
  tauri::plugin::Builder::new("icons_editor")
    .setup(|application| {
      application.manage(IconsEditorState {
        system_ltx_path: Arc::new(Mutex::new(None)),
        equipment_sprite_path: Arc::new(Mutex::new(None)),
        equipment_sprite_name: Arc::new(Mutex::new(None)),
        equipment_sprite: Arc::new(Mutex::new(None)),
        equipment_sprite_preview: Arc::new(Mutex::new(None)),
        equipment_descriptors: Arc::new(Mutex::new(None)),
      });

      Ok(())
    })
    .register_uri_scheme_protocol("stream", move |application, request| {
      match get_equipment_sprite_stream_response(application, request) {
        Ok(response) => Ok(response),
        Err(error) => {
          log::warn!("Failed to handle stream protocol request: {:?}", error);

          ResponseBuilder::new()
            .status(StatusCode::BAD_REQUEST)
            .header(CONTENT_TYPE, "text/plain")
            .body(error.to_string().as_bytes().to_vec())
        }
      }
    })
    .invoke_handler(tauri::generate_handler![
      crate::icons_editor::commands::open_equipment_sprite,
      crate::icons_editor::commands::reopen_equipment_sprite,
      crate::icons_editor::commands::get_equipment_sprite,
      crate::icons_editor::commands::close_equipment_sprite,
    ])
    .build()
}
