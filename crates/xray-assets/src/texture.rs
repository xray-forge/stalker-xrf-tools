/// Converts an X-Ray renderer texture reference into the DDS logical path it loads.
pub fn dds_logical_path(reference: &str) -> String {
  if let Some((stem, extension)) = reference.rsplit_once('.')
    && ["tga", "dds", "bmp", "ogm"]
      .iter()
      .any(|known| extension.eq_ignore_ascii_case(known))
  {
    format!("{stem}.dds")
  } else {
    format!("{reference}.dds")
  }
}
