/// Escape a value for use as XML character data.
///
/// Only `&` and `<` are escaped, which is all XML requires between tags. Escaping more is valid but
/// not harmless: shipped game data writes apostrophes, quotes and `>` raw everywhere, so rewriting
/// them as entities changes bytes an edit never meant to touch and diverges from the file's own
/// convention.
pub fn escape_xml_text(value: &str) -> String {
  value.replace('&', "&amp;").replace('<', "&lt;")
}

/// Escape a value for use inside a double-quoted attribute.
///
/// Adds the quote character to what character data needs. The apostrophe is left raw because these
/// writers quote attributes with `"`, so `'` carries no meaning inside one.
pub fn escape_xml_attribute(value: &str) -> String {
  escape_xml_text(value).replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn text_escapes_only_what_character_data_requires() {
    assert_eq!(escape_xml_text("a & b < c"), "a &amp; b &lt; c");
  }

  #[test]
  fn text_leaves_alone_what_shipped_data_writes_raw() {
    // A dialogue line like "we'll jump that jackass" must come back out byte for byte.
    assert_eq!(escape_xml_text("we'll go > there \"now\""), "we'll go > there \"now\"");
  }

  #[test]
  fn attributes_also_escape_the_quote_they_are_wrapped_in() {
    assert_eq!(escape_xml_attribute("a \" b & c"), "a &quot; b &amp; c");
  }

  #[test]
  fn attributes_leave_the_apostrophe_raw() {
    // Valid inside a double-quoted attribute, and one fewer entity in the output.
    assert_eq!(escape_xml_attribute("it's"), "it's");
  }

  #[test]
  fn escaping_an_already_escaped_value_does_not_double_up_the_ampersand_twice() {
    // `&` is replaced first, so `&amp;` cannot be produced from the `&` of an entity written later.
    assert_eq!(escape_xml_text("&amp;"), "&amp;amp;");
  }
}
