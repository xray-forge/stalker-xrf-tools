use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigInventorySectionDescriptor {
  pub name: String,
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}
