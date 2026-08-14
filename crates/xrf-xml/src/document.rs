use xrf_error::{XrfError, XrfResult};
use xrf_utils::{
  XRayEncoding, decode_bytes_to_string, get_utf8_encoder, get_windows1250_encoder, get_windows1251_encoder,
  get_windows1252_encoder,
};

/// Parsing behavior shared by X-Ray XML readers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XmlParseOptions {
  pub allow_dtd: bool,
}

/// A parsed XML document detached from its input buffer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlDocument {
  root: XmlElement,
}

impl XmlDocument {
  /// Parse a UTF-8 XML string.
  ///
  /// # Errors
  ///
  /// Returns a parsing error when the input is not a well-formed XML document.
  pub fn parse(input: &str, options: XmlParseOptions) -> XrfResult<Self> {
    let document: roxmltree::Document = roxmltree::Document::parse_with_options(
      input,
      roxmltree::ParsingOptions {
        allow_dtd: options.allow_dtd,
        ..roxmltree::ParsingOptions::default()
      },
    )
    .map_err(|error| XrfError::new_parsing_error(format!("Failed to parse XML: {error}")))?;

    Ok(Self {
      root: XmlElement::from_node(document.root_element()),
    })
  }

  /// Decode and parse XML bytes according to their declaration, defaulting to UTF-8.
  ///
  /// # Errors
  ///
  /// Returns an encoding error for unsupported or invalid input encodings, or a parsing error for malformed XML.
  pub fn parse_bytes(input: &[u8], options: XmlParseOptions) -> XrfResult<Self> {
    let decoded: String = decode_xml_bytes(input)?;

    Self::parse(&decoded, options)
  }

  pub fn root(&self) -> &XmlElement {
    &self.root
  }

  pub fn elements_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a XmlElement> + 'a {
    std::iter::once(&self.root)
      .chain(self.root.descendants())
      .filter(move |element| element.name() == name)
  }
}

/// One XML element with ordered attributes and child elements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlElement {
  name: String,
  attributes: Vec<XmlAttribute>,
  children: Vec<XmlElement>,
  text: String,
}

impl XmlElement {
  fn from_node(node: roxmltree::Node<'_, '_>) -> Self {
    Self {
      name: node.tag_name().name().to_string(),
      attributes: node
        .attributes()
        .map(|attribute| XmlAttribute {
          name: attribute.name().to_string(),
          value: attribute.value().to_string(),
        })
        .collect(),
      children: node
        .children()
        .filter(|child| child.is_element())
        .map(Self::from_node)
        .collect(),
      text: node
        .children()
        .filter(|child| child.is_text())
        .filter_map(|child| child.text())
        .collect(),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn attribute(&self, name: &str) -> Option<&str> {
    self
      .attributes
      .iter()
      .find(|attribute| attribute.name == name)
      .map(|attribute| attribute.value.as_str())
  }

  pub fn attributes(&self) -> impl Iterator<Item = (&str, &str)> {
    self
      .attributes
      .iter()
      .map(|attribute| (attribute.name.as_str(), attribute.value.as_str()))
  }

  pub fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Self> + 'a {
    self.children.iter().filter(move |child| child.name == name)
  }

  pub fn descendants(&self) -> impl Iterator<Item = &Self> {
    XmlDescendants {
      stack: self.children.iter().rev().collect(),
    }
  }

  pub fn descendants_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Self> + 'a {
    self.descendants().filter(move |element| element.name == name)
  }

  pub fn text(&self) -> &str {
    &self.text
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct XmlAttribute {
  name: String,
  value: String,
}

struct XmlDescendants<'a> {
  stack: Vec<&'a XmlElement>,
}

impl<'a> Iterator for XmlDescendants<'a> {
  type Item = &'a XmlElement;

  fn next(&mut self) -> Option<Self::Item> {
    let element: &XmlElement = self.stack.pop()?;
    self.stack.extend(element.children.iter().rev());

    Some(element)
  }
}

/// Return the encoding declared by an XML prolog, if present.
pub fn declared_xml_encoding(input: &[u8]) -> XrfResult<Option<XRayEncoding>> {
  let Some(label) = declared_encoding_label(input) else {
    return Ok(None);
  };
  let normalized: String = label
    .chars()
    .filter(|character| character.is_ascii_alphanumeric())
    .flat_map(char::to_lowercase)
    .collect();

  let encoding = match normalized.as_str() {
    "utf8" => get_utf8_encoder(),
    "cp1250" | "windows1250" => get_windows1250_encoder(),
    "cp1251" | "windows1251" => get_windows1251_encoder(),
    "cp1252" | "windows1252" => get_windows1252_encoder(),
    _ => Err(XrfError::new_encoding_error(format!(
      "Unsupported XML encoding '{label}'"
    )))?,
  };

  Ok(Some(encoding))
}

/// Decode XML bytes from their declaration, defaulting to UTF-8 when no declaration is present.
pub fn decode_xml_bytes(input: &[u8]) -> XrfResult<String> {
  Ok(decode_bytes_to_string(
    input,
    declared_xml_encoding(input)?.unwrap_or_else(get_utf8_encoder),
  )?)
}

fn declared_encoding_label(input: &[u8]) -> Option<String> {
  let prefix_length: usize = input.len().min(256);
  let prefix: String = String::from_utf8_lossy(&input[..prefix_length]).into_owned();
  let lowercase: String = prefix.to_ascii_lowercase();
  let declaration_start: usize = lowercase.find("<?xml")?;
  let declaration_end: usize = lowercase[declaration_start..].find("?>")? + declaration_start;
  let declaration: &str = &prefix[declaration_start..declaration_end];
  let declaration_lowercase: String = declaration.to_ascii_lowercase();
  let encoding_start: usize = declaration_lowercase.find("encoding")? + "encoding".len();
  let after_encoding: &str = declaration[encoding_start..].trim_start();
  let after_equals: &str = after_encoding.strip_prefix('=')?.trim_start();
  let quote: char = after_equals.chars().next()?;

  if quote != '\'' && quote != '"' {
    return None;
  }

  let value: &str = &after_equals[quote.len_utf8()..];
  let end: usize = value.find(quote)?;

  Some(value[..end].to_string())
}

#[cfg(test)]
mod tests {
  use super::*;
  use xrf_utils::encode_string_to_bytes;

  #[test]
  fn parses_elements_attributes_and_text() {
    let document: XmlDocument = XmlDocument::parse(
      "<root><entry id=\"first\">value</entry><group><entry id=\"second\"/></group></root>",
      XmlParseOptions::default(),
    )
    .unwrap();

    let entries: Vec<&XmlElement> = document.elements_named("entry").collect();

    assert_eq!(document.root().name(), "root");
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].attribute("id"), Some("first"));
    assert_eq!(entries[0].text(), "value");
  }

  #[test]
  fn parses_declared_windows_1251_encoding() {
    let source = "<?xml version=\"1.0\" encoding=\"windows-1251\"?><root><text>\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442}</text></root>";
    let encoded: Vec<u8> = encode_string_to_bytes(source, get_windows1251_encoder()).unwrap();

    let document: XmlDocument = XmlDocument::parse_bytes(&encoded, XmlParseOptions::default()).unwrap();

    assert_eq!(
      document.elements_named("text").next().unwrap().text(),
      "\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442}"
    );
  }

  #[test]
  fn rejects_dtd_unless_enabled() {
    let source = "<!DOCTYPE root [<!ELEMENT root EMPTY>]><root/>";

    assert!(XmlDocument::parse(source, XmlParseOptions::default()).is_err());
    assert!(XmlDocument::parse(source, XmlParseOptions { allow_dtd: true }).is_ok());
  }
}
