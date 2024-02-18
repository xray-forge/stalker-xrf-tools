use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_graph_point::AlifeGraphPoint;
use crate::data::alife::alife_item_weapon::AlifeItemWeapon;
use crate::data::alife::alife_item_weapon_magazined::AlifeItemWeaponMagazined;
use crate::data::alife::alife_item_weapon_magazined_wgl::AlifeItemWeaponMagazinedWgl;
use crate::data::alife::alife_item_weapon_shotgun::AlifeItemWeaponShotgun;
use crate::data::alife::alife_object_anomalous_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_breakable::AlifeObjectBreakable;
use crate::data::alife::alife_object_climable::AlifeObjectClimable;
use crate::data::alife::alife_object_hanging_lamp::AlifeObjectHangingLamp;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife::alife_object_item_ammo::AlifeObjectItemAmmo;
use crate::data::alife::alife_object_item_artefact::AlifeItemArtefact;
use crate::data::alife::alife_object_item_grenade::AlifeObjectItemGrenade;
use crate::data::alife::alife_object_physic::AlifeObjectPhysic;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife::alife_smart_cover::AlifeSmartCover;
use crate::data::alife_class::AlifeClass::CseAlifeItemArtefact;
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
        AlifeObjectBreakable::verify(chunk);
      }
      AlifeClass::CseAlifeObjectClimable => {
        let object: AlifeObjectClimable = AlifeObjectClimable::from_chunk(chunk);
        AlifeObjectClimable::verify(chunk);
      }
      AlifeClass::CseAlifeGraphPoint => {
        let object: AlifeGraphPoint = AlifeGraphPoint::from_chunk(chunk);
        AlifeGraphPoint::verify(chunk);
      }
      AlifeClass::SeSmartCover => {
        let object: AlifeSmartCover = AlifeSmartCover::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
      }
      AlifeClass::SeZoneAnom => {
        let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::from_chunk(chunk);
        AlifeObjectAnomalyZone::verify(chunk);
      }
      AlifeClass::CseAlifeSpaceRestrictor => {
        let object: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::from_chunk(chunk);
        AlifeObjectSpaceRestrictor::verify(chunk);
      }
      AlifeClass::CseAlifeObjectPhysic => {
        let object: AlifeObjectPhysic = AlifeObjectPhysic::from_chunk(chunk);
        AlifeObjectPhysic::verify(chunk);
      }
      AlifeClass::CseAlifeObjectHangingLamp => {
        let object: AlifeObjectHangingLamp = AlifeObjectHangingLamp::from_chunk(chunk);
        AlifeObjectHangingLamp::verify(chunk)
      }
      AlifeClass::CseAlifeItem => {
        let object: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);
        AlifeObjectItem::verify(chunk);
      }
      AlifeClass::CseAlifeItemAmmo => {
        let object: AlifeObjectItemAmmo = AlifeObjectItemAmmo::from_chunk(chunk);
        AlifeObjectItemAmmo::verify(chunk);
      }
      AlifeClass::CseAlifeItemGrenade => {
        let object: AlifeObjectItemGrenade = AlifeObjectItemGrenade::from_chunk(chunk);
        AlifeObjectItemGrenade::verify(chunk);
      }
      AlifeClass::CseAlifeItemArtefact => {
        let object: AlifeItemArtefact = AlifeItemArtefact::from_chunk(chunk);
        AlifeItemArtefact::verify(chunk);
      }
      AlifeClass::CseAlifeItemWeapon => {
        let object: AlifeItemWeapon = AlifeItemWeapon::from_chunk(chunk);
        AlifeItemWeapon::verify(chunk);
      }
      AlifeClass::CseAlifeItemWeaponShotgun => {
        let object: AlifeItemWeaponShotgun = AlifeItemWeaponShotgun::from_chunk(chunk);
        AlifeItemWeaponShotgun::verify(chunk);
      }
      AlifeClass::CseAlifeItemWeaponMagazined => {
        let object: AlifeItemWeaponMagazined = AlifeItemWeaponMagazined::from_chunk(chunk);
        AlifeItemWeaponMagazined::verify(chunk);
      }
      AlifeClass::CseAlifeItemWeaponMagazinedWGl => {
        let object: AlifeItemWeaponMagazinedWgl = AlifeItemWeaponMagazinedWgl::from_chunk(chunk);
        AlifeItemWeaponMagazinedWgl::verify(chunk);
      }
      _ => {
        log::warn!("Not implemented parser for: {:?}", alife_class)
      }
    }
  }
}
