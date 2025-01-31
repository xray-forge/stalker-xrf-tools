use crate::icons_editor::state::IconsEditorState;
use std::sync::MutexGuard;
use tauri::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_LENGTH, CONTENT_TYPE, REFERER};
use tauri::http::response::Builder;
use tauri::http::Result as HttpResult;
use tauri::http::{Request, Response};
use tauri::{AppHandle, Manager, Runtime, State, UriSchemeContext};

pub fn get_equipment_sprite_stream_response<R: Runtime>(
  context: UriSchemeContext<R>,
  request: &Request<Vec<u8>>,
) -> HttpResult<Response<Vec<u8>>> {
  let handle: &AppHandle<R> = context.app_handle();
  let icons_editor_state: State<IconsEditorState> = handle.state::<IconsEditorState>();

  let sprite_lock: MutexGuard<Option<String>> =
    icons_editor_state.equipment_sprite_name.lock().unwrap();
  let preview_lock: MutexGuard<Option<Vec<u8>>> =
    icons_editor_state.equipment_sprite_preview.lock().unwrap();

  let preview: Option<&Vec<u8>> = preview_lock.as_ref();
  let sprite_name: Option<&String> = sprite_lock.as_ref();

  if preview.is_none() || sprite_name.is_none() {
    log::info!("Incorrect asset request while not existing");
    return Response::builder().status(404).body(Vec::new());
  }

  let preview: &Vec<u8> = preview.unwrap();
  let sprite_name: &String = sprite_name.unwrap();

  let uri: String = percent_encoding::percent_decode(request.uri().path().as_bytes())
    .decode_utf8_lossy()
    .to_string();

  if !uri.ends_with(&format!("/{}", sprite_name)) {
    log::info!("Incorrect asset request: {uri}");

    return Response::builder().status(404).body(Vec::new());
  }

  let mut response: Builder = Response::builder();

  if let Some(referer) = request
    .headers()
    .get(REFERER)
    .map(|header| header.to_str().unwrap())
  {
    response = response.header(ACCESS_CONTROL_ALLOW_ORIGIN, referer.trim_matches('/'))
  }

  response
    .header(CONTENT_TYPE, "image/png")
    .header(CONTENT_LENGTH, preview.len())
    .body(preview.clone())
}
