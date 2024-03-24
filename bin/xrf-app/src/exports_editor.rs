use serde_json::{json, Value};
use std::path::PathBuf;
use xray_export::EffectsParser;

#[tauri::command]
pub async fn get_xr_effects(path: &str) -> Result<Value, String> {
  log::info!("Open exports folder: {:?}", path);

  let parser: EffectsParser = match EffectsParser::new(&PathBuf::from(path)) {
    Ok(parser) => parser,
    Err(error) => return Err(error.to_string()),
  };

  log::info!("Parsing xr_effects exports from folder: {:?}", path);

  match parser.parse_effects() {
    Ok(value) => Ok(json!(value)),
    Err(error) => Err(error.to_string()),
  }
}
