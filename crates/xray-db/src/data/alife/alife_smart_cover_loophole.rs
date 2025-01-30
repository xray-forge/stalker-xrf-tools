use serde::{Deserialize, Serialize};
use xray_error::{XRayError, XRayResult};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeSmartCoverLoophole {
  pub name: String,
  pub enabled: u8,
}

impl AlifeSmartCoverLoophole {
  /// Serialize list of loopholes into single string.
  pub fn list_to_string(loopholes: &[Self]) -> String {
    loopholes
      .iter()
      .map(|loophole| format!("{}:{}", loophole.name, loophole.enabled))
      .collect::<Vec<_>>()
      .join(",")
  }

  /// Read list of loopholes from string.
  pub fn string_to_list(value: &str) -> XRayResult<Vec<Self>> {
    let mut loopholes: Vec<Self> = Vec::new();

    for it in value.split(',').map(|it| it.trim()) {
      let partial: Vec<&str> = it.split(':').map(|it| it.trim()).collect::<Vec<&str>>();

      if partial.len() == 2 {
        loopholes.push(Self {
          name: String::from(*partial.first().unwrap()),
          enabled: match partial.last().unwrap().parse::<u8>() {
            Ok(parsed) => parsed,
            Err(_) => {
              return Err(XRayError::new_parsing_error(
                "Failed to parse loophole enabled status",
              ))
            }
          },
        })
      } else {
        return Err(XRayError::new_parsing_error(
          "Invalid value provided for loopholes parsing, ':' separated values expected",
        ));
      }
    }

    Ok(loopholes)
  }
}
