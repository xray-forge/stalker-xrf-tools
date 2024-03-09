use crate::chunk::reader::ChunkReader;
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
use byteorder::ByteOrder;
use enum_map::Enum;
use std::io;
use xray_ltx::Section;

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
    reader: &mut ChunkReader,
    alife_class: &AlifeClass,
  ) -> io::Result<Box<dyn AlifeObjectGeneric>> {
    match alife_class {
      AlifeClass::SeActor => {
        let object: AlifeActor = AlifeActor::read::<T>(reader)?;
        AlifeActor::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectBreakable => {
        let object: AlifeObjectBreakable = AlifeObjectBreakable::read::<T>(reader)?;
        AlifeObjectBreakable::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectClimable => {
        let object: AlifeObjectClimable = AlifeObjectClimable::read::<T>(reader)?;
        AlifeObjectClimable::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeGraphPoint => {
        let object: AlifeGraphPoint = AlifeGraphPoint::read::<T>(reader)?;
        AlifeGraphPoint::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeSpaceRestrictor => {
        let object: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read::<T>(reader)?;
        AlifeObjectSpaceRestrictor::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::SeSmartCover => {
        let object: AlifeSmartCover = AlifeSmartCover::read::<T>(reader)?;
        AlifeSmartCover::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeAnomalousZone => {
        let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::read::<T>(reader)?;
        AlifeObjectAnomalyZone::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::SeZoneAnom => {
        let object: AlifeAnomalousZone = AlifeAnomalousZone::read::<T>(reader)?;
        AlifeAnomalousZone::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::SeZoneTorrid => {
        let object: AlifeObjectTorridZone = AlifeObjectTorridZone::read::<T>(reader)?;
        AlifeObjectTorridZone::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::SeSmartTerrain => {
        let object: AlifeSmartTerrain = AlifeSmartTerrain::read::<T>(reader)?;
        AlifeSmartTerrain::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::SeLevelChanger => {
        let object: AlifeLevelChanger = AlifeLevelChanger::read::<T>(reader)?;
        AlifeLevelChanger::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::SeZoneVisual => {
        let object: AlifeZoneVisual = AlifeZoneVisual::read::<T>(reader)?;
        AlifeZoneVisual::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectPhysic => {
        let object: AlifeObjectPhysic = AlifeObjectPhysic::read::<T>(reader)?;
        AlifeObjectPhysic::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeHelicopter => {
        let object: AlifeObjectHelicopter = AlifeObjectHelicopter::read::<T>(reader)?;
        AlifeObjectHelicopter::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeInventoryBox => {
        let object: AlifeObjectInventoryBox = AlifeObjectInventoryBox::read::<T>(reader)?;
        AlifeObjectInventoryBox::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeObjectHangingLamp => {
        let object: AlifeObjectHangingLamp = AlifeObjectHangingLamp::read::<T>(reader)?;
        AlifeObjectHangingLamp::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItem => {
        let object: AlifeObjectItem = AlifeObjectItem::read::<T>(reader)?;
        AlifeObjectItem::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemExplosive => {
        let object: AlifeObjectItemExplosive = AlifeObjectItemExplosive::read::<T>(reader)?;
        AlifeObjectItemExplosive::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemPda => {
        let object: AlifeObjectItemPda = AlifeObjectItemPda::read::<T>(reader)?;
        AlifeObjectItemPda::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemAmmo => {
        let object: AlifeObjectItemAmmo = AlifeObjectItemAmmo::read::<T>(reader)?;
        AlifeObjectItemAmmo::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemGrenade => {
        let object: AlifeObjectItemGrenade = AlifeObjectItemGrenade::read::<T>(reader)?;
        AlifeObjectItemGrenade::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemArtefact => {
        let object: AlifeObjectItemArtefact = AlifeObjectItemArtefact::read::<T>(reader)?;
        AlifeObjectItemArtefact::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeapon => {
        let object: AlifeObjectItemWeapon = AlifeObjectItemWeapon::read::<T>(reader)?;
        AlifeObjectItemWeapon::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemDetector => {
        let object: AlifeObjectItemDetector = AlifeObjectItemDetector::read::<T>(reader)?;
        AlifeObjectItemDetector::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemHelmet => {
        let object: AlifeObjectItemHelmet = AlifeObjectItemHelmet::read::<T>(reader)?;
        AlifeObjectItemHelmet::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemCustomOutfit => {
        let object: AlifeObjectItemCustomOutfit = AlifeObjectItemCustomOutfit::read::<T>(reader)?;
        AlifeObjectItemCustomOutfit::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeaponShotgun => {
        let object: AlifeObjectItemWeaponShotgun = AlifeObjectItemWeaponShotgun::read::<T>(reader)?;
        AlifeObjectItemWeaponShotgun::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeaponMagazined => {
        let object: AlifeObjectItemWeaponMagazined =
          AlifeObjectItemWeaponMagazined::read::<T>(reader)?;
        AlifeObjectItemWeaponMagazined::verify(reader);
        Ok(Box::new(object))
      }
      AlifeClass::CseAlifeItemWeaponMagazinedWGl => {
        let object: AlifeObjectItemWeaponMagazinedWgl =
          AlifeObjectItemWeaponMagazinedWgl::read::<T>(reader)?;
        AlifeObjectItemWeaponMagazinedWgl::verify(reader);
        Ok(Box::new(object))
      }
      _ => {
        panic!("Not implemented parser for: {:?}", alife_class)
      }
    }
  }

  /// Import custom save data based on serialized clsid.
  pub fn import_by_class(
    alife_class: &AlifeClass,
    section: &Section,
  ) -> io::Result<Box<dyn AlifeObjectGeneric>> {
    Ok(match alife_class {
      AlifeClass::SeActor => Box::new(AlifeActor::import(section)?),
      AlifeClass::CseAlifeObjectBreakable => Box::new(AlifeObjectBreakable::import(section)?),
      AlifeClass::CseAlifeObjectClimable => Box::new(AlifeObjectClimable::import(section)?),
      AlifeClass::CseAlifeGraphPoint => Box::new(AlifeGraphPoint::import(section)?),
      AlifeClass::CseAlifeSpaceRestrictor => Box::new(AlifeObjectSpaceRestrictor::import(section)?),
      AlifeClass::SeSmartCover => Box::new(AlifeSmartCover::import(section)?),
      AlifeClass::CseAlifeAnomalousZone => Box::new(AlifeObjectAnomalyZone::import(section)?),
      AlifeClass::SeZoneAnom => Box::new(AlifeAnomalousZone::import(section)?),
      AlifeClass::SeZoneTorrid => Box::new(AlifeObjectTorridZone::import(section)?),
      AlifeClass::SeSmartTerrain => Box::new(AlifeSmartTerrain::import(section)?),
      AlifeClass::SeLevelChanger => Box::new(AlifeLevelChanger::import(section)?),
      AlifeClass::SeZoneVisual => Box::new(AlifeZoneVisual::import(section)?),
      AlifeClass::CseAlifeObjectPhysic => Box::new(AlifeObjectPhysic::import(section)?),
      AlifeClass::CseAlifeHelicopter => Box::new(AlifeObjectHelicopter::import(section)?),
      AlifeClass::CseAlifeInventoryBox => Box::new(AlifeObjectInventoryBox::import(section)?),
      AlifeClass::CseAlifeObjectHangingLamp => Box::new(AlifeObjectHangingLamp::import(section)?),
      AlifeClass::CseAlifeItem => Box::new(AlifeObjectItem::import(section)?),
      AlifeClass::CseAlifeItemExplosive => Box::new(AlifeObjectItemExplosive::import(section)?),
      AlifeClass::CseAlifeItemPda => Box::new(AlifeObjectItemPda::import(section)?),
      AlifeClass::CseAlifeItemAmmo => Box::new(AlifeObjectItemAmmo::import(section)?),
      AlifeClass::CseAlifeItemGrenade => Box::new(AlifeObjectItemGrenade::import(section)?),
      AlifeClass::CseAlifeItemArtefact => Box::new(AlifeObjectItemArtefact::import(section)?),
      AlifeClass::CseAlifeItemWeapon => Box::new(AlifeObjectItemWeapon::import(section)?),
      AlifeClass::CseAlifeItemDetector => Box::new(AlifeObjectItemDetector::import(section)?),
      AlifeClass::CseAlifeItemHelmet => Box::new(AlifeObjectItemHelmet::import(section)?),
      AlifeClass::CseAlifeItemCustomOutfit => {
        Box::new(AlifeObjectItemCustomOutfit::import(section)?)
      }
      AlifeClass::CseAlifeItemWeaponShotgun => {
        Box::new(AlifeObjectItemWeaponShotgun::import(section)?)
      }
      AlifeClass::CseAlifeItemWeaponMagazined => {
        Box::new(AlifeObjectItemWeaponMagazined::import(section)?)
      }
      AlifeClass::CseAlifeItemWeaponMagazinedWGl => {
        Box::new(AlifeObjectItemWeaponMagazinedWgl::import(section)?)
      }
      _ => {
        panic!("Not implemented parser for: {:?}", alife_class)
      }
    })
  }
}
