use serde_json::{json, Value};
use std::path::PathBuf;
use xray_export::ExportsParser;

#[tauri::command]
pub async fn parse_xr_effects(path: &str) -> Result<Value, String> {
  log::info!("Parsing effects exports folder: {:?}", path);

  let parser: ExportsParser = ExportsParser::new();

  match parser.parse_effects_from_path(&PathBuf::from(path)) {
    Ok(value) => Ok(json!(value)),
    Err(error) => Err(error.to_string()),
  }
}
