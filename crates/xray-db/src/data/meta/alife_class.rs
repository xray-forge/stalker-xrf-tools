use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_actor::AlifeActor;
use crate::data::alife::alife_anomalous_zone::AlifeAnomalousZone;
use crate::data::alife::alife_graph_point::AlifeGraphPoint;
use crate::data::alife::alife_level_changer::AlifeLevelChanger;
use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_breakable::AlifeObjectBreakable;
use crate::data::alife::alife_object_climable::AlifeObjectClimable;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_hanging_lamp::AlifeObjectHangingLamp;
use crate::data::alife::alife_object_helicopter::AlifeObjectHelicopter;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_inventory_box::AlifeObjectInventoryBox;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife::alife_object_item_ammo::AlifeObjectItemAmmo;
use crate::data::alife::alife_object_item_artefact::AlifeObjectItemArtefact;
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
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use enum_map::Enum;
use std::io;

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
  /// Additionally, should respect script extension.
  pub fn read_by_class<T: ByteOrder>(
    chunk: &mut Chunk,
    alife_class: &AlifeClass,
  ) -> io::Result<Box<dyn AlifeObjectGeneric<Order = SpawnByteOrder>>> {
    match alife_class {
      AlifeClass::SeActor => {
        let object: AlifeActor = AlifeActor::read_from_chunk::<T>(chunk)?;
        AlifeActor::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectBreakable => {
        let object: AlifeObjectBreakable = AlifeObjectBreakable::read_from_chunk::<T>(chunk)?;
        AlifeObjectBreakable::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectClimable => {
        let object: AlifeObjectClimable = AlifeObjectClimable::read_from_chunk::<T>(chunk)?;
        AlifeObjectClimable::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeGraphPoint => {
        let object: AlifeGraphPoint = AlifeGraphPoint::read_from_chunk::<T>(chunk)?;
        AlifeGraphPoint::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeSpaceRestrictor => {
        let object: AlifeObjectSpaceRestrictor =
          AlifeObjectSpaceRestrictor::read_from_chunk::<T>(chunk)?;
        AlifeObjectSpaceRestrictor::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::SeSmartCover => {
        let object: AlifeSmartCover = AlifeSmartCover::read_from_chunk::<T>(chunk)?;
        AlifeSmartCover::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeAnomalousZone => {
        let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::read_from_chunk::<T>(chunk)?;
        AlifeObjectAnomalyZone::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::SeZoneAnom => {
        let object: AlifeAnomalousZone = AlifeAnomalousZone::read_from_chunk::<T>(chunk)?;
        AlifeAnomalousZone::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::SeZoneTorrid => {
        let object: AlifeObjectTorridZone = AlifeObjectTorridZone::read_from_chunk::<T>(chunk)?;
        AlifeObjectTorridZone::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::SeSmartTerrain => {
        let object: AlifeSmartTerrain = AlifeSmartTerrain::read_from_chunk::<T>(chunk)?;
        AlifeSmartTerrain::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::SeLevelChanger => {
        let object: AlifeLevelChanger = AlifeLevelChanger::read_from_chunk::<T>(chunk)?;
        AlifeLevelChanger::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::SeZoneVisual => {
        let object: AlifeZoneVisual = AlifeZoneVisual::read_from_chunk::<T>(chunk)?;
        AlifeZoneVisual::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectPhysic => {
        let object: AlifeObjectPhysic = AlifeObjectPhysic::read_from_chunk::<T>(chunk)?;
        AlifeObjectPhysic::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeHelicopter => {
        let object: AlifeObjectHelicopter = AlifeObjectHelicopter::read_from_chunk::<T>(chunk)?;
        AlifeObjectHelicopter::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeInventoryBox => {
        let object: AlifeObjectInventoryBox = AlifeObjectInventoryBox::read_from_chunk::<T>(chunk)?;
        AlifeObjectInventoryBox::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectHangingLamp => {
        let object: AlifeObjectHangingLamp = AlifeObjectHangingLamp::read_from_chunk::<T>(chunk)?;
        AlifeObjectHangingLamp::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItem => {
        let object: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;
        AlifeObjectItem::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemExplosive => {
        let object: AlifeObjectItemExplosive =
          AlifeObjectItemExplosive::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemExplosive::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemPda => {
        let object: AlifeObjectItemPda = AlifeObjectItemPda::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemPda::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemAmmo => {
        let object: AlifeObjectItemAmmo = AlifeObjectItemAmmo::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemAmmo::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemGrenade => {
        let object: AlifeObjectItemGrenade = AlifeObjectItemGrenade::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemGrenade::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemArtefact => {
        let object: AlifeObjectItemArtefact = AlifeObjectItemArtefact::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemArtefact::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeapon => {
        let object: AlifeObjectItemWeapon = AlifeObjectItemWeapon::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemWeapon::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemDetector => {
        let object: AlifeObjectItemDetector = AlifeObjectItemDetector::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemDetector::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemHelmet => {
        let object: AlifeObjectItemHelmet = AlifeObjectItemHelmet::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemHelmet::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemCustomOutfit => {
        let object: AlifeObjectItemCustomOutfit =
          AlifeObjectItemCustomOutfit::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemCustomOutfit::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeaponShotgun => {
        let object: AlifeObjectItemWeaponShotgun =
          AlifeObjectItemWeaponShotgun::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemWeaponShotgun::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeaponMagazined => {
        let object: AlifeObjectItemWeaponMagazined =
          AlifeObjectItemWeaponMagazined::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemWeaponMagazined::verify(chunk);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeaponMagazinedWGl => {
        let object: AlifeObjectItemWeaponMagazinedWgl =
          AlifeObjectItemWeaponMagazinedWgl::read_from_chunk::<T>(chunk)?;
        AlifeObjectItemWeaponMagazinedWgl::verify(chunk);
        Ok(Box::new(object))
      }
      _ => {
        panic!("Not implemented parser for: {:?}", alife_class)
      }
    }
  }
}
