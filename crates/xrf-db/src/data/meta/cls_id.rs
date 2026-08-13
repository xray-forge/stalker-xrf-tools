#![allow(dead_code)]
use enum_map::Enum;
use serde::{Deserialize, Serialize};
use xrf_error::{XRayError, XRayResult};

use crate::data::meta::map::SECTION_TO_CLS_ID;

/// todo: Add script to parse system ltx and read all the data from ltx/txt file instead.
#[cfg_attr(
  feature = "typescript-bindings",
  derive(ts_rs::TS),
  ts(export, export_to = "xrf-db.ts")
)]
#[derive(Clone, Debug, Enum, PartialEq, Serialize, Deserialize, Eq)]
pub enum ClsId {
  AiCrow,
  AiFleG,
  AiGraph,
  AiPhant,
  AiRat,
  AiRatG,
  AiSpGrp,
  AmmoS,
  Artefact,
  CHlcpS,
  DFlare,
  DPda,
  DetAdva,
  DetElit,
  DetSimp,
  DetScie,
  EHlmet,
  EStlk,
  GF1S,
  GFake,
  GRgd5S,
  GRpg7,
  IIAttch,
  IIBolt,
  IIBttch,
  IIDoc,
  LvlChng,
  NwAttch,
  OBrkbl,
  OClmbl,
  ODstrS,
  OPhysS,
  OSearch,
  PSkelet,
  SActor,
  SExplo,
  SFaction,
  SFood,
  SInvBox,
  SM209,
  SOG7B,
  SPda,
  SVog25,
  ScriptZn,
  ScrptArt,
  ScrptCar,
  ScrptObj,
  SmBlood,
  SmBoarW,
  SmBurer,
  SmChims,
  SmContr,
  SmDogF,
  SmDogP,
  SmDogS,
  SmFlesh,
  SmGiant,
  SmPDog,
  SmPoltr,
  SmSnork,
  SmTushk,
  SmrtCS,
  SmrtTrrn,
  SoHLamp,
  SpcRsS,
  Spect,
  TorchS,
  WMountd,
  WSTMGun,
  WpAk74,
  WpAshTG,
  WpBM16,
  WpBinoc,
  WpGLaun,
  WpGroza,
  WpHPSA,
  WpKnife,
  WpLR300,
  WpPM,
  WpRG6,
  WpRPG7,
  WpSVD,
  WpSVU,
  WpScope,
  WpSilen,
  WpVAL,
  ZCFire,
  ZMbald,
  ZNoGrav,
  ZRadio,
  ZTeamBs,
  ZsBFuzz,
  ZsGalan,
  ZsMBald,
  ZsMince,
  ZsRadio,
  ZsTorrd,
}

impl ClsId {
  pub fn from_section(section_name: &str) -> XRayResult<Self> {
    SECTION_TO_CLS_ID.get(section_name).cloned().ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Unknown ALife object section '{section_name}', no CLSID mapping exists"
      ))
    })
  }
}

#[cfg(test)]
mod tests {
  use xrf_error::XRayError;

  use super::ClsId;

  #[test]
  fn maps_known_section_to_clsid() {
    assert_eq!(ClsId::from_section("actor").unwrap(), ClsId::SActor);
  }

  #[test]
  fn returns_error_for_unknown_section() {
    let error: XRayError = ClsId::from_section("unknown_section").unwrap_err();

    assert_eq!(
      error.to_string(),
      "Parsing error: Unknown ALife object section 'unknown_section', no CLSID mapping exists"
    );
  }
}
