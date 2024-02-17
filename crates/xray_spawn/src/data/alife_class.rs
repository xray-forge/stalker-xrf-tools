use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_graph_point::AlifeGraphPoint;
use crate::data::alife::alife_object_anomalous_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_breakable::AlifeObjectBreakable;
use crate::data::alife::alife_object_climable::AlifeObjectClimable;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife::alife_smart_cover::AlifeSmartCover;
use crate::data::alife_object::AlifeObjectInherited;
use enum_map::Enum;

#[derive(Clone, Debug, Enum, PartialEq)]
pub enum AlifeClass {
  CseAlifeAnomalousZone,
  CseAlifeCar,
  CseAlifeCreatureCrow,
  CseAlifeDynamicObjectVisual,
  CseAlifeFleshGroup,
  CseAlifeGraphPoint,
  CseAlifeHelicopter,
  CseAlifeInventoryBox,
  CseAlifeItem,
  CseAlifeItemAmmo,
  CseAlifeItemArtefact,
  CseAlifeItemBolt,
  CseAlifeItemCustomOutfit,
  CseAlifeItemDetector,
  CseAlifeItemDocument,
  CseAlifeItemExplosive,
  CseAlifeItemGrenade,
  CseAlifeItemHelmet,
  CseAlifeItemPda,
  CseAlifeItemTorch,
  CseAlifeItemWeapon,
  CseAlifeItemWeaponMagazined,
  CseAlifeItemWeaponMagazinedWGl,
  CseAlifeItemWeaponShotgun,
  CseAlifeMonsterBase,
  CseAlifeMountedWeapon,
  CseAlifeObjectBreakable,
  CseAlifeObjectClimable,
  CseAlifeObjectHangingLamp,
  CseAlifeObjectPhysic,
  CseAlifeObjectProjector,
  CseAlifePhSkeletonObject,
  CseAlifeRatGroup,
  CseAlifeSpaceRestrictor,
  CseAlifeSpawnGroup,
  CseAlifeStationaryMGun,
  CseAlifeTeamBaseZone,
  CseAlifeTrader,
  CseSpectator,
  SeActor,
  SeLevelChanger,
  SeMonster,
  SeSimFaction,
  SeSmartCover,
  SeSmartTerrain,
  SeStalker,
  SeZoneAnom, // cse_anomalous_zone
  SeZoneTorrid,
  SeZoneVisual,
  SimSquadScripted,
  Unknown,
}

impl AlifeClass {
  /// Read custom save data based on serialized clsid.
  /// Represents STATE_Read of each separate object in xray implementation.
  /// Additionally should respect script extension.
  pub fn read_by_class(chunk: &mut Chunk, alife_class: &AlifeClass) -> () {
    match alife_class {
      AlifeClass::CseAlifeObjectBreakable => {
        let object: AlifeObjectBreakable = AlifeObjectBreakable::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
      }
      AlifeClass::CseAlifeObjectClimable => {
        let object: AlifeObjectClimable = AlifeObjectClimable::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
      }
      AlifeClass::CseAlifeGraphPoint => {
        let object: AlifeGraphPoint = AlifeGraphPoint::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
      }
      AlifeClass::SeSmartCover => {
        let object: AlifeSmartCover = AlifeSmartCover::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
      }
      AlifeClass::SeZoneAnom => {
        let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
      }
      AlifeClass::CseAlifeSpaceRestrictor => {
        let object: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
      }
      _ => {
        log::warn!("Not implemented parser for: {:?}", alife_class)
      }
    }
  }
}
