#![allow(dead_code)]

use crate::data::meta::alife_class::AlifeClass;
use crate::data::meta::map::{CLS_ID_TO_CLASS, SECTION_TO_CLS_ID};
use enum_map::Enum;
use std::ops::Index;

/// todo: Add script to parse system ini and read all the data from ini/txt file instead.
#[derive(Clone, Debug, Enum, PartialEq)]
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
  pub fn from_section(section: &String) -> ClsId {
    SECTION_TO_CLS_ID
      .get(section.as_str())
      .cloned()
      .expect("Unexpected section provided for clsid matching.")
  }
}

impl AlifeClass {
  pub fn from_cls_id(cls_id: &ClsId) -> AlifeClass {
    CLS_ID_TO_CLASS.index(cls_id.clone()).clone()
  }
}
