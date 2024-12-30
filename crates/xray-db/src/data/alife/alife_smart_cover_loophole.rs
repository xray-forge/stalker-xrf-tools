use crate::error::database_parse_error::DatabaseParseError;
use crate::types::DatabaseResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeSmartCoverLoophole {
  pub name: String,
  pub enabled: u8,
}

impl AlifeSmartCoverLoophole {
  /// Serialize list of loopholes into single string.
  pub fn list_to_string(loopholes: &[AlifeSmartCoverLoophole]) -> String {
    loopholes
      .iter()
      .map(|loophole| format!("{}:{}", loophole.name, loophole.enabled))
      .collect::<Vec<_>>()
      .join(",")
  }

  /// Read list of loopholes from string.
  pub fn string_to_list(value: &str) -> DatabaseResult<Vec<AlifeSmartCoverLoophole>> {
    let mut loopholes: Vec<AlifeSmartCoverLoophole> = Vec::new();

    for it in value.split(',').map(|it| it.trim()) {
      let partial: Vec<&str> = it.split(':').map(|it| it.trim()).collect::<Vec<&str>>();

      if partial.len() == 2 {
        loopholes.push(AlifeSmartCoverLoophole {
          name: String::from(*partial.first().unwrap()),
          enabled: match partial.last().unwrap().parse::<u8>() {
            Ok(parsed) => parsed,
            Err(_) => {
              return Err(DatabaseParseError::new_database_error(
                "Failed to parse loophole enabled status",
              ))
            }
          },
        })
      } else {
        return Err(DatabaseParseError::new_database_error(
          "Invalid value provided for loopholes parsion, ':' separated values expected",
        ));
      }
    }

    Ok(loopholes)
  }
}
