/// Escape text for use in XML content or attribute values.
pub fn escape_xml(value: &str) -> String {
  value
    .replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('"', "&quot;")
    .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn escapes_xml_reserved_characters() {
    assert_eq!(escape_xml("<&>\"'"), "&lt;&amp;&gt;&quot;&apos;");
  }
}
