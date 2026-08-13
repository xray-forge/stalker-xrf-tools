use quick_xml::se::Serializer;
use serde::Serialize;
use serde::de::DeserializeOwned;
use xrf_error::{XRayError, XRayResult};

/// Deserialize XML into a Serde model.
///
/// # Errors
///
/// Returns a parsing error when the XML does not match the requested model.
pub fn deserialize_xml<T>(input: &str) -> XRayResult<T>
where
  T: DeserializeOwned,
{
  quick_xml::de::from_str(input).map_err(|error| XRayError::new_parsing_error(error.to_string()))
}

/// Serialize a Serde model as indented XML with expanded empty elements.
///
/// # Errors
///
/// Returns a serialization error when the model cannot be represented as XML.
pub fn serialize_xml<T>(value: &T) -> XRayResult<String>
where
  T: Serialize,
{
  let mut output: String = String::new();
  let mut serializer: Serializer<String> = Serializer::new(&mut output);

  serializer.expand_empty_elements(true);
  serializer.indent(' ', 2);
  value
    .serialize(serializer)
    .map_err(|error| XRayError::new_serialization_error(error.to_string()))?;

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
    let result: XRayResult<Fixture> = deserialize_xml("<root>");

    assert!(result.is_err());
  }
}
