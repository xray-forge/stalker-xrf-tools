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

#[cfg(test)]
mod tests {
  use super::dds_logical_path;

  #[test]
  fn replaces_renderer_authoring_extensions_with_dds() {
    assert_eq!(dds_logical_path("pfx\\smoke.TGA"), "pfx\\smoke.dds");
    assert_eq!(dds_logical_path("pfx\\smoke.bmp"), "pfx\\smoke.dds");
  }

  #[test]
  fn appends_dds_for_unknown_or_missing_extensions() {
    assert_eq!(dds_logical_path("pfx\\smoke"), "pfx\\smoke.dds");
    assert_eq!(dds_logical_path("pfx\\smoke.png"), "pfx\\smoke.png.dds");
  }
}
