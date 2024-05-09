#[tauri::command]
pub async fn open_equipment_sprite(equipment_dds_path: &str) -> Result<u32, String> {
  log::info!("Opening equipment file: {equipment_dds_path}");

  // let json: Value = json!(1);

  Ok(1)
}

#[tauri::command]
pub async fn get_equipment_sprite_uri() -> &'static str {
  log::info!("Getting equipment sprite uri");

  "test.png"
}
