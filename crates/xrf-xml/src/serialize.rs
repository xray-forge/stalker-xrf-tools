use quick_xml::se::Serializer;
use serde::Serialize;
use xrf_error::{XrfError, XrfResult};

/// Serialize a Serde model as indented XML with expanded empty elements.
///
/// Empty elements are expanded rather than self-closed because the engine's own files write them
/// that way, and generated output that matches what ships is easier to compare against it.
///
/// # Errors
///
/// Returns a serialization error when the model cannot be represented as XML.
pub fn serialize_xml<T>(value: &T) -> XrfResult<String>
where
  T: Serialize,
{
  let mut output: String = String::new();
  let mut serializer: Serializer<String> = Serializer::new(&mut output);

  serializer.expand_empty_elements(true);
  serializer.indent(' ', 2);
  value
    .serialize(serializer)
    .map_err(|error| XrfError::new_serialization_error(error.to_string()))?;

  Ok(output)
}

#[cfg(test)]
mod tests {
  use serde::Serialize;

  use super::*;

  #[derive(Debug, Serialize)]
  #[serde(rename = "root")]
  struct Fixture {
    value: String,
  }

  #[test]
  fn serializes_an_indented_document() {
    let serialized: String = serialize_xml(&Fixture {
      value: String::from("content"),
    })
    .unwrap();

    assert_eq!(serialized, "<root>\n  <value>content</value>\n</root>");
  }

  #[test]
  fn expands_an_empty_element_rather_than_self_closing_it() {
    let serialized: String = serialize_xml(&Fixture { value: String::new() }).unwrap();

    assert_eq!(serialized, "<root>\n  <value></value>\n</root>");
  }
}
