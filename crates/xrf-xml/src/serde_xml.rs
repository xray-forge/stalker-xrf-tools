use quick_xml::se::Serializer;
use serde::Serialize;
use serde::de::DeserializeOwned;
use xrf_error::{XrfError, XrfResult};

use crate::decode_xml_bytes;

/// Deserialize XML into a Serde model.
///
/// # Errors
///
/// Returns a parsing error when the XML does not match the requested model.
pub fn deserialize_xml<T>(input: &str) -> XrfResult<T>
where
  T: DeserializeOwned,
{
  quick_xml::de::from_str(input).map_err(|error| XrfError::new_parsing_error(error.to_string()))
}

/// Decode XML bytes according to their declaration and deserialize them into a Serde model.
pub fn deserialize_xml_bytes<T>(input: &[u8]) -> XrfResult<T>
where
  T: DeserializeOwned,
{
  deserialize_xml(&decode_xml_bytes(input)?)
}

/// Serialize a Serde model as indented XML with expanded empty elements.
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
  use super::*;
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
  #[serde(rename = "root")]
  struct Fixture {
    value: String,
  }

  #[test]
  fn round_trips_serde_models() {
    let source = Fixture {
      value: String::from("content"),
    };

    let serialized: String = serialize_xml(&source).unwrap();
    let deserialized: Fixture = deserialize_xml(&serialized).unwrap();

    assert_eq!(deserialized, source);
    assert_eq!(serialized, "<root>\n  <value>content</value>\n</root>");
  }

  #[test]
  fn returns_an_error_for_invalid_xml() {
    let result: XrfResult<Fixture> = deserialize_xml("<root>");

    assert!(result.is_err());
  }

  #[test]
  fn deserializes_declared_windows_1251_bytes() {
    use xrf_utils::{encode_string_to_bytes, get_windows1251_encoder};

    let source = "<?xml version=\"1.0\" encoding=\"windows-1251\"?><root><value>\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442}</value></root>";
    let encoded = encode_string_to_bytes(source, get_windows1251_encoder()).unwrap();

    let fixture: Fixture = deserialize_xml_bytes(&encoded).unwrap();

    assert_eq!(fixture.value, "\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442}");
  }
}
