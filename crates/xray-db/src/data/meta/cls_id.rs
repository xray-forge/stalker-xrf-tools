#![allow(dead_code)]
use crate::data::meta::map::SECTION_TO_CLS_ID;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

/// todo: Add script to parse system ltx and read all the data from ltx/txt file instead.
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
  pub fn from_section(section_name: &str) -> Self {
    // todo: Implement with From<T> trait?
    SECTION_TO_CLS_ID
      .get(section_name)
      .cloned()
      .expect("Unexpected section provided for clsid matching")
  }
}
