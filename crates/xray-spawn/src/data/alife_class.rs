use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_actor::AlifeActor;
use crate::data::alife::alife_graph_point::AlifeGraphPoint;
use crate::data::alife::alife_level_changer::AlifeLevelChanger;
use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_breakable::AlifeObjectBreakable;
use crate::data::alife::alife_object_climable::AlifeObjectClimable;
use crate::data::alife::alife_object_hanging_lamp::AlifeObjectHangingLamp;
use crate::data::alife::alife_object_helicopter::AlifeObjectHelicopter;
use crate::data::alife::alife_object_inventory_box::AlifeObjectInventoryBox;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife::alife_object_item_ammo::AlifeObjectItemAmmo;
use crate::data::alife::alife_object_item_artefact::AlifeItemArtefact;
use crate::data::alife::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
use crate::data::alife::alife_object_item_detector::AlifeObjectItemDetector;
use crate::data::alife::alife_object_item_explosive::AlifeObjectItemExplosive;
use crate::data::alife::alife_object_item_grenade::AlifeObjectItemGrenade;
use crate::data::alife::alife_object_item_helmet::AlifeObjectItemHelmet;
use crate::data::alife::alife_object_item_pda::AlifeObjectItemPda;
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use crate::data::alife::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
use crate::data::alife::alife_object_item_weapon_magazined_wgl::AlifeObjectItemWeaponMagazinedWgl;
use crate::data::alife::alife_object_item_weapon_shotgun::AlifeObjectItemWeaponShotgun;
use crate::data::alife::alife_object_physic::AlifeObjectPhysic;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife::alife_object_torrid_zone::AlifeObjectTorridZone;
use crate::data::alife::alife_smart_cover::AlifeSmartCover;
use crate::data::alife::alife_smart_terrain::AlifeSmartTerrain;
use crate::data::alife::alife_zone_visual::AlifeZoneVisual;
use crate::data::alife_object_base::{AlifeObjectGeneric, AlifeObjectInheritedReader};
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
  pub fn read_by_class(chunk: &mut Chunk, alife_class: &AlifeClass) -> Box<dyn AlifeObjectGeneric> {
    match alife_class {
      AlifeClass::SeActor => {
        let object: AlifeActor = AlifeActor::from_chunk(chunk);
        AlifeActor::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeObjectBreakable => {
        let object: AlifeObjectBreakable = AlifeObjectBreakable::from_chunk(chunk);
        AlifeObjectBreakable::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeObjectClimable => {
        let object: AlifeObjectClimable = AlifeObjectClimable::from_chunk(chunk);
        AlifeObjectClimable::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeGraphPoint => {
        let object: AlifeGraphPoint = AlifeGraphPoint::from_chunk(chunk);
        AlifeGraphPoint::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeSpaceRestrictor => {
        let object: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::from_chunk(chunk);
        AlifeObjectSpaceRestrictor::verify(chunk);
        Box::new(object)
      }
      AlifeClass::SeSmartCover => {
        let object: AlifeSmartCover = AlifeSmartCover::from_chunk(chunk);
        AlifeSmartCover::verify(chunk);
        Box::new(object)
      }
      AlifeClass::SeZoneAnom | AlifeClass::CseAlifeAnomalousZone => {
        let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::from_chunk(chunk);
        AlifeObjectAnomalyZone::verify(chunk);
        Box::new(object)
      }
      AlifeClass::SeZoneTorrid => {
        let object: AlifeObjectTorridZone = AlifeObjectTorridZone::from_chunk(chunk);
        AlifeObjectTorridZone::verify(chunk);
        Box::new(object)
      }
      AlifeClass::SeSmartTerrain => {
        let object: AlifeSmartTerrain = AlifeSmartTerrain::from_chunk(chunk);
        AlifeSmartTerrain::verify(chunk);
        Box::new(object)
      }
      AlifeClass::SeLevelChanger => {
        let object: AlifeLevelChanger = AlifeLevelChanger::from_chunk(chunk);
        AlifeLevelChanger::verify(chunk);
        Box::new(object)
      }
      AlifeClass::SeZoneVisual => {
        let object: AlifeZoneVisual = AlifeZoneVisual::from_chunk(chunk);
        AlifeZoneVisual::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeObjectPhysic => {
        let object: AlifeObjectPhysic = AlifeObjectPhysic::from_chunk(chunk);
        AlifeObjectPhysic::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeHelicopter => {
        let object: AlifeObjectHelicopter = AlifeObjectHelicopter::from_chunk(chunk);
        AlifeObjectHelicopter::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeInventoryBox => {
        let object: AlifeObjectInventoryBox = AlifeObjectInventoryBox::from_chunk(chunk);
        AlifeObjectInventoryBox::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeObjectHangingLamp => {
        let object: AlifeObjectHangingLamp = AlifeObjectHangingLamp::from_chunk(chunk);
        AlifeObjectHangingLamp::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItem => {
        let object: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);
        AlifeObjectItem::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemExplosive => {
        let object: AlifeObjectItemExplosive = AlifeObjectItemExplosive::from_chunk(chunk);
        AlifeObjectItemExplosive::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemPda => {
        let object: AlifeObjectItemPda = AlifeObjectItemPda::from_chunk(chunk);
        AlifeObjectItemPda::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemAmmo => {
        let object: AlifeObjectItemAmmo = AlifeObjectItemAmmo::from_chunk(chunk);
        AlifeObjectItemAmmo::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemGrenade => {
        let object: AlifeObjectItemGrenade = AlifeObjectItemGrenade::from_chunk(chunk);
        AlifeObjectItemGrenade::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemArtefact => {
        let object: AlifeItemArtefact = AlifeItemArtefact::from_chunk(chunk);
        AlifeItemArtefact::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemWeapon => {
        let object: AlifeObjectItemWeapon = AlifeObjectItemWeapon::from_chunk(chunk);
        AlifeObjectItemWeapon::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemDetector => {
        let object: AlifeObjectItemDetector = AlifeObjectItemDetector::from_chunk(chunk);
        AlifeObjectItemDetector::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemHelmet => {
        let object: AlifeObjectItemHelmet = AlifeObjectItemHelmet::from_chunk(chunk);
        AlifeObjectItemHelmet::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemCustomOutfit => {
        let object: AlifeObjectItemCustomOutfit = AlifeObjectItemCustomOutfit::from_chunk(chunk);
        AlifeObjectItemCustomOutfit::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemWeaponShotgun => {
        let object: AlifeObjectItemWeaponShotgun = AlifeObjectItemWeaponShotgun::from_chunk(chunk);
        AlifeObjectItemWeaponShotgun::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemWeaponMagazined => {
        let object: AlifeObjectItemWeaponMagazined =
          AlifeObjectItemWeaponMagazined::from_chunk(chunk);
        AlifeObjectItemWeaponMagazined::verify(chunk);
        Box::new(object)
      }
      AlifeClass::CseAlifeItemWeaponMagazinedWGl => {
        let object: AlifeObjectItemWeaponMagazinedWgl =
          AlifeObjectItemWeaponMagazinedWgl::from_chunk(chunk);
        AlifeObjectItemWeaponMagazinedWgl::verify(chunk);
        Box::new(object)
      }
      _ => {
        panic!("Not implemented parser for: {:?}", alife_class)
      }
    }
  }
}
