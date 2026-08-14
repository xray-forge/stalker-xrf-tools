use std::ops::Range;

use xrf_error::{XrfError, XrfResult};

use crate::document::XmlParseOptions;

/// An element together with where it sits in the text it was parsed from.
///
/// Ranges are byte offsets into that text, which is what makes editing a document in place possible:
/// everything outside a spliced range keeps the bytes it was read with, including the comments and
/// indentation that re-serializing would drop.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XmlElementSpan {
  name: String,
  attributes: Vec<(String, String)>,
  element: Range<usize>,
  content: Option<Range<usize>>,
  text: String,
  children: Vec<XmlElementSpan>,
}

impl XmlElementSpan {
  /// Parse the root element of a document, keeping every element's position.
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

    Ok(Self::from_node(input, document.root_element()))
  }

  fn from_node(input: &str, node: roxmltree::Node<'_, '_>) -> Self {
    let name: String = node.tag_name().name().to_owned();
    let element: Range<usize> = node.range();
    let content: Option<Range<usize>> = content_range(input, &element, &name);

    Self {
      attributes: node
        .attributes()
        .map(|attribute| (attribute.name().to_owned(), attribute.value().to_owned()))
        .collect(),
      children: node
        .children()
        .filter(roxmltree::Node::is_element)
        .map(|child| Self::from_node(input, child))
        .collect(),
      text: node
        .children()
        .filter(roxmltree::Node::is_text)
        .filter_map(|child| child.text())
        .collect(),
      content,
      element,
      name,
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn attribute(&self, name: &str) -> Option<&str> {
    self
      .attributes
      .iter()
      .find(|(attribute, _)| attribute == name)
      .map(|(_, value)| value.as_str())
  }

  /// Byte range of the whole element, both tags included.
  pub fn element_range(&self) -> &Range<usize> {
    &self.element
  }

  /// Byte range of everything between the tags. `None` for a self-closing element, which has no
  /// content to replace and has to be rewritten whole instead.
  pub fn content_range(&self) -> Option<&Range<usize>> {
    self.content.as_ref()
  }

  /// Text content with entities resolved, which is not what the content range holds.
  pub fn text(&self) -> &str {
    &self.text
  }

  pub fn children(&self) -> impl Iterator<Item = &Self> {
    self.children.iter()
  }

  pub fn child_named<'a>(&'a self, name: &'a str) -> Option<&'a Self> {
    self.children.iter().find(move |child| child.name == name)
  }

  pub fn children_named<'a>(&'a self, name: &'a str) -> impl Iterator<Item = &'a Self> + 'a {
    self.children.iter().filter(move |child| child.name == name)
  }
}

/// Locate the span between an element's tags without trusting its text children.
///
/// Derived from the element's own range rather than from a text node, because an element may hold no
/// text child at all, or several once entity references split it. Both cases still have exactly one
/// stretch of source between the tags, and that stretch is what an edit replaces.
fn content_range(input: &str, element: &Range<usize>, name: &str) -> Option<Range<usize>> {
  let source: &str = input.get(element.clone())?;
  let open_end: usize = source.find('>')? + 1;

  if source[..open_end].ends_with("/>") {
    return None;
  }

  let closing: String = format!("</{name}>");

  if !source.ends_with(&closing) {
    return None;
  }

  let content_end: usize = element.end.checked_sub(closing.len())?;
  let content_start: usize = element.start + open_end;

  if content_start > content_end {
    return None;
  }

  Some(content_start..content_end)
}

#[cfg(test)]
mod tests {
  use super::*;

  const OPTIONS: XmlParseOptions = XmlParseOptions { allow_dtd: false };

  #[test]
  fn locates_content_without_disturbing_the_rest_of_the_document() {
    let input: &str = "<string_table>\n\t<!-- note -->\n\t<string id=\"a\">\n\t\t<text>first</text>\n\t</string>\n</string_table>";
    let root: XmlElementSpan = XmlElementSpan::parse(input, OPTIONS).unwrap();
    let text: &XmlElementSpan = root
      .child_named("string")
      .and_then(|string| string.child_named("text"))
      .unwrap();

    assert_eq!(&input[text.content_range().unwrap().clone()], "first");

    let mut edited: String = input.to_owned();
    edited.replace_range(text.content_range().unwrap().clone(), "second");

    // The comment and every tab survive, which is the whole reason ranges are carried around.
    assert_eq!(
      edited,
      "<string_table>\n\t<!-- note -->\n\t<string id=\"a\">\n\t\t<text>second</text>\n\t</string>\n</string_table>"
    );
  }

  #[test]
  fn gives_an_empty_content_range_for_an_empty_element() {
    let input: &str = "<string_table><string id=\"a\"><text></text></string></string_table>";
    let root: XmlElementSpan = XmlElementSpan::parse(input, OPTIONS).unwrap();
    let text: &XmlElementSpan = root
      .child_named("string")
      .and_then(|string| string.child_named("text"))
      .unwrap();
    let range: Range<usize> = text.content_range().unwrap().clone();

    assert!(range.is_empty());
    assert_eq!(&input[range], "");
  }

  #[test]
  fn reports_no_content_range_for_a_self_closing_element() {
    let input: &str = "<string_table><string id=\"a\"><text/></string></string_table>";
    let root: XmlElementSpan = XmlElementSpan::parse(input, OPTIONS).unwrap();
    let text: &XmlElementSpan = root
      .child_named("string")
      .and_then(|string| string.child_named("text"))
      .unwrap();

    assert!(text.content_range().is_none());
  }

  #[test]
  fn keeps_entities_in_the_range_while_resolving_them_in_the_text() {
    let input: &str = "<string_table><string id=\"a\"><text>a &amp; b</text></string></string_table>";
    let root: XmlElementSpan = XmlElementSpan::parse(input, OPTIONS).unwrap();
    let text: &XmlElementSpan = root
      .child_named("string")
      .and_then(|string| string.child_named("text"))
      .unwrap();

    assert_eq!(text.text(), "a & b");
    assert_eq!(&input[text.content_range().unwrap().clone()], "a &amp; b");
  }

  #[test]
  fn carries_attributes_and_element_ranges() {
    let input: &str = "<string_table><string id=\"a\"><text>x</text></string></string_table>";
    let root: XmlElementSpan = XmlElementSpan::parse(input, OPTIONS).unwrap();
    let string: &XmlElementSpan = root.child_named("string").unwrap();

    assert_eq!(string.attribute("id"), Some("a"));
    assert_eq!(
      &input[string.element_range().clone()],
      "<string id=\"a\"><text>x</text></string>"
    );
  }
}
