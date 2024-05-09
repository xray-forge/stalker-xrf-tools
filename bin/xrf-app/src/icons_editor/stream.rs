use http::header::*;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageEncoder};
use std::io::Write;
use std::path::PathBuf;
use tauri::http::{Request, Response, ResponseBuilder};
use xray_icon::{read_dds_by_path, RgbaImage};

pub fn get_equipment_sprite_stream_response(
  request: &Request,
) -> Result<Response, Box<dyn std::error::Error>> {
  let uri: String = percent_encoding::percent_decode(request.uri().as_bytes())
    .decode_utf8_lossy()
    .to_string();

  if !uri.ends_with("/test.png") {
    log::info!("todo Incorrect response: {uri}");

    return ResponseBuilder::new().status(404).body(Vec::new());
  }

  let path = PathBuf::from(
    "C:\\Projects\\stalker-xrf-engine\\src\\resources\\textures\\ui\\ui_icon_equipment.dds",
  );

  let image: RgbaImage = read_dds_by_path(&path)?;

  let mut buffer: Vec<u8> = Vec::new();

  PngEncoder::new(buffer.by_ref())
    .write_image(
      image.as_raw(),
      image.width(),
      image.height(),
      ColorType::Rgba8,
    )
    .expect("error encoding pixels as PNG");

  log::info!("Opened: {:?}", path);

  ResponseBuilder::new()
    .header(CONTENT_TYPE, "image/png")
    .header(CONTENT_LENGTH, image.len())
    .body(buffer)
}
