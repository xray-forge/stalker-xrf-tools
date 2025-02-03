use crate::data::alife::inherited::alife_actor::AlifeActor;
use crate::data::alife::inherited::alife_anomalous_zone::AlifeAnomalousZone;
use crate::data::alife::inherited::alife_graph_point::AlifeGraphPoint;
use crate::data::alife::inherited::alife_level_changer::AlifeLevelChanger;
use crate::data::alife::inherited::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::inherited::alife_object_breakable::AlifeObjectBreakable;
use crate::data::alife::inherited::alife_object_climable::AlifeObjectClimable;
use crate::data::alife::inherited::alife_object_hanging_lamp::AlifeObjectHangingLamp;
use crate::data::alife::inherited::alife_object_helicopter::AlifeObjectHelicopter;
use crate::data::alife::inherited::alife_object_inventory_box::AlifeObjectInventoryBox;
use crate::data::alife::inherited::alife_object_item::AlifeObjectItem;
use crate::data::alife::inherited::alife_object_item_ammo::AlifeObjectItemAmmo;
use crate::data::alife::inherited::alife_object_item_artefact::AlifeObjectItemArtefact;
use crate::data::alife::inherited::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
use crate::data::alife::inherited::alife_object_item_detector::AlifeObjectItemDetector;
use crate::data::alife::inherited::alife_object_item_explosive::AlifeObjectItemExplosive;
use crate::data::alife::inherited::alife_object_item_grenade::AlifeObjectItemGrenade;
use crate::data::alife::inherited::alife_object_item_helmet::AlifeObjectItemHelmet;
use crate::data::alife::inherited::alife_object_item_pda::AlifeObjectItemPda;
use crate::data::alife::inherited::alife_object_item_weapon::AlifeObjectItemWeapon;
use crate::data::alife::inherited::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
use crate::data::alife::inherited::alife_object_item_weapon_magazined_wgl::AlifeObjectItemWeaponMagazinedWgl;
use crate::data::alife::inherited::alife_object_item_weapon_shotgun::AlifeObjectItemWeaponShotgun;
use crate::data::alife::inherited::alife_object_physic::AlifeObjectPhysic;
use crate::data::alife::inherited::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife::inherited::alife_object_torrid_zone::AlifeObjectTorridZone;
use crate::data::alife::inherited::alife_smart_cover::AlifeSmartCover;
use crate::data::alife::inherited::alife_smart_terrain::AlifeSmartTerrain;
use crate::data::alife::inherited::alife_zone_visual::AlifeZoneVisual;
use crate::data::meta::alife_class::AlifeClass;
use crate::export::LtxImportExport;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::Ltx;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AlifeObjectInherited {
  SeActor(Box<AlifeActor>),
  CseAlifeObjectBreakable(Box<AlifeObjectBreakable>),
  CseAlifeObjectClimable(Box<AlifeObjectClimable>),
  CseAlifeGraphPoint(Box<AlifeGraphPoint>),
  CseAlifeSpaceRestrictor(Box<AlifeObjectSpaceRestrictor>),
  SeSmartCover(Box<AlifeSmartCover>),
  CseAlifeAnomalousZone(Box<AlifeObjectAnomalyZone>),
  SeZoneAnom(Box<AlifeAnomalousZone>),
  SeZoneTorrid(Box<AlifeObjectTorridZone>),
  SeSmartTerrain(Box<AlifeSmartTerrain>),
  SeLevelChanger(Box<AlifeLevelChanger>),
  SeZoneVisual(Box<AlifeZoneVisual>),
  CseAlifeObjectPhysic(Box<AlifeObjectPhysic>),
  CseAlifeHelicopter(Box<AlifeObjectHelicopter>),
  CseAlifeInventoryBox(Box<AlifeObjectInventoryBox>),
  CseAlifeObjectHangingLamp(Box<AlifeObjectHangingLamp>),
  CseAlifeItem(Box<AlifeObjectItem>),
  CseAlifeItemExplosive(Box<AlifeObjectItemExplosive>),
  CseAlifeItemPda(Box<AlifeObjectItemPda>),
  CseAlifeItemAmmo(Box<AlifeObjectItemAmmo>),
  CseAlifeItemGrenade(Box<AlifeObjectItemGrenade>),
  CseAlifeItemArtefact(Box<AlifeObjectItemArtefact>),
  CseAlifeItemWeapon(Box<AlifeObjectItemWeapon>),
  CseAlifeItemDetector(Box<AlifeObjectItemDetector>),
  CseAlifeItemHelmet(Box<AlifeObjectItemHelmet>),
  CseAlifeItemCustomOutfit(Box<AlifeObjectItemCustomOutfit>),
  CseAlifeItemWeaponShotgun(Box<AlifeObjectItemWeaponShotgun>),
  CseAlifeItemWeaponMagazined(Box<AlifeObjectItemWeaponMagazined>),
  CseAlifeItemWeaponMagazinedWGl(Box<AlifeObjectItemWeaponMagazinedWgl>),
}

impl AlifeObjectInherited {
  /// Read custom save data based on serialized clsid.
  /// Represents STATE_Read of each separate object in xray implementation.
  /// Additionally, should respect script extension.
  pub fn read<T: ByteOrder>(
    reader: &mut ChunkReader,
    alife_class: &AlifeClass,
  ) -> XRayResult<Self> {
    Ok(match alife_class {
      AlifeClass::SeActor => Self::SeActor(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::CseAlifeObjectBreakable => {
        Self::CseAlifeObjectBreakable(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeObjectClimable => {
        Self::CseAlifeObjectClimable(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeGraphPoint => {
        Self::CseAlifeGraphPoint(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeSpaceRestrictor => {
        Self::CseAlifeSpaceRestrictor(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::SeSmartCover => Self::SeSmartCover(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::CseAlifeAnomalousZone => {
        Self::CseAlifeAnomalousZone(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::SeZoneAnom => Self::SeZoneAnom(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::SeZoneTorrid => Self::SeZoneTorrid(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::SeSmartTerrain => Self::SeSmartTerrain(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::SeLevelChanger => Self::SeLevelChanger(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::SeZoneVisual => Self::SeZoneVisual(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::CseAlifeObjectPhysic => {
        Self::CseAlifeObjectPhysic(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeHelicopter => {
        Self::CseAlifeHelicopter(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeInventoryBox => {
        Self::CseAlifeInventoryBox(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeObjectHangingLamp => {
        Self::CseAlifeObjectHangingLamp(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItem => Self::CseAlifeItem(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::CseAlifeItemExplosive => {
        Self::CseAlifeItemExplosive(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemPda => Self::CseAlifeItemPda(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::CseAlifeItemAmmo => Self::CseAlifeItemAmmo(Box::new(reader.read_xr::<T, _>()?)),
      AlifeClass::CseAlifeItemGrenade => {
        Self::CseAlifeItemGrenade(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemArtefact => {
        Self::CseAlifeItemArtefact(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemWeapon => {
        Self::CseAlifeItemWeapon(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemDetector => {
        Self::CseAlifeItemDetector(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemHelmet => {
        Self::CseAlifeItemHelmet(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemCustomOutfit => {
        Self::CseAlifeItemCustomOutfit(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemWeaponShotgun => {
        Self::CseAlifeItemWeaponShotgun(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemWeaponMagazined => {
        Self::CseAlifeItemWeaponMagazined(Box::new(reader.read_xr::<T, _>()?))
      }
      AlifeClass::CseAlifeItemWeaponMagazinedWGl => {
        Self::CseAlifeItemWeaponMagazinedWGl(Box::new(reader.read_xr::<T, _>()?))
      }
      _ => {
        return Err(XRayError::new_parsing_error(format!(
          "Not implemented parser for {}",
          alife_class
        )));
      }
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    match self {
      AlifeObjectInherited::SeActor(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::CseAlifeObjectBreakable(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeObjectClimable(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeGraphPoint(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeSpaceRestrictor(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::SeSmartCover(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::CseAlifeAnomalousZone(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::SeZoneAnom(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::SeZoneTorrid(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::SeSmartTerrain(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::SeLevelChanger(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::SeZoneVisual(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::CseAlifeObjectPhysic(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeHelicopter(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeInventoryBox(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeObjectHangingLamp(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItem(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::CseAlifeItemExplosive(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemPda(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::CseAlifeItemAmmo(object) => writer.write_xr::<T, _>(object.deref())?,
      AlifeObjectInherited::CseAlifeItemGrenade(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemArtefact(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemWeapon(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemDetector(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemHelmet(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemCustomOutfit(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemWeaponShotgun(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemWeaponMagazined(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
      AlifeObjectInherited::CseAlifeItemWeaponMagazinedWGl(object) => {
        writer.write_xr::<T, _>(object.deref())?
      }
    }

    Ok(())
  }

  pub fn import(section_name: &str, ltx: &Ltx, alife_class: &AlifeClass) -> XRayResult<Self> {
    Ok(match alife_class {
      AlifeClass::SeActor => Self::SeActor(Box::new(AlifeActor::import(section_name, ltx)?)),
      AlifeClass::CseAlifeObjectBreakable => {
        Self::CseAlifeObjectBreakable(Box::new(AlifeObjectBreakable::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeObjectClimable => {
        Self::CseAlifeObjectClimable(Box::new(AlifeObjectClimable::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeGraphPoint => {
        Self::CseAlifeGraphPoint(Box::new(AlifeGraphPoint::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeSpaceRestrictor => Self::CseAlifeSpaceRestrictor(Box::new(
        AlifeObjectSpaceRestrictor::import(section_name, ltx)?,
      )),
      AlifeClass::SeSmartCover => {
        Self::SeSmartCover(Box::new(AlifeSmartCover::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeAnomalousZone => {
        Self::CseAlifeAnomalousZone(Box::new(AlifeObjectAnomalyZone::import(section_name, ltx)?))
      }
      AlifeClass::SeZoneAnom => {
        Self::SeZoneAnom(Box::new(AlifeAnomalousZone::import(section_name, ltx)?))
      }
      AlifeClass::SeZoneTorrid => {
        Self::SeZoneTorrid(Box::new(AlifeObjectTorridZone::import(section_name, ltx)?))
      }
      AlifeClass::SeSmartTerrain => {
        Self::SeSmartTerrain(Box::new(AlifeSmartTerrain::import(section_name, ltx)?))
      }
      AlifeClass::SeLevelChanger => {
        Self::SeLevelChanger(Box::new(AlifeLevelChanger::import(section_name, ltx)?))
      }
      AlifeClass::SeZoneVisual => {
        Self::SeZoneVisual(Box::new(AlifeZoneVisual::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeObjectPhysic => {
        Self::CseAlifeObjectPhysic(Box::new(AlifeObjectPhysic::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeHelicopter => {
        Self::CseAlifeHelicopter(Box::new(AlifeObjectHelicopter::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeInventoryBox => Self::CseAlifeInventoryBox(Box::new(
        AlifeObjectInventoryBox::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeObjectHangingLamp => Self::CseAlifeObjectHangingLamp(Box::new(
        AlifeObjectHangingLamp::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeItem => {
        Self::CseAlifeItem(Box::new(AlifeObjectItem::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeItemExplosive => Self::CseAlifeItemExplosive(Box::new(
        AlifeObjectItemExplosive::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeItemPda => {
        Self::CseAlifeItemPda(Box::new(AlifeObjectItemPda::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeItemAmmo => {
        Self::CseAlifeItemAmmo(Box::new(AlifeObjectItemAmmo::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeItemGrenade => {
        Self::CseAlifeItemGrenade(Box::new(AlifeObjectItemGrenade::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeItemArtefact => Self::CseAlifeItemArtefact(Box::new(
        AlifeObjectItemArtefact::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeItemWeapon => {
        Self::CseAlifeItemWeapon(Box::new(AlifeObjectItemWeapon::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeItemDetector => Self::CseAlifeItemDetector(Box::new(
        AlifeObjectItemDetector::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeItemHelmet => {
        Self::CseAlifeItemHelmet(Box::new(AlifeObjectItemHelmet::import(section_name, ltx)?))
      }
      AlifeClass::CseAlifeItemCustomOutfit => Self::CseAlifeItemCustomOutfit(Box::new(
        AlifeObjectItemCustomOutfit::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeItemWeaponShotgun => Self::CseAlifeItemWeaponShotgun(Box::new(
        AlifeObjectItemWeaponShotgun::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeItemWeaponMagazined => Self::CseAlifeItemWeaponMagazined(Box::new(
        AlifeObjectItemWeaponMagazined::import(section_name, ltx)?,
      )),
      AlifeClass::CseAlifeItemWeaponMagazinedWGl => Self::CseAlifeItemWeaponMagazinedWGl(Box::new(
        AlifeObjectItemWeaponMagazinedWgl::import(section_name, ltx)?,
      )),
      _ => {
        return Err(XRayError::new_parsing_error(format!(
          "Not implemented parser for {}",
          alife_class
        )));
      }
    })
  }

  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    match self {
      AlifeObjectInherited::SeActor(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeObjectBreakable(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeObjectClimable(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeGraphPoint(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeSpaceRestrictor(object) => object.export(section_name, ltx),
      AlifeObjectInherited::SeSmartCover(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeAnomalousZone(object) => object.export(section_name, ltx),
      AlifeObjectInherited::SeZoneAnom(object) => object.export(section_name, ltx),
      AlifeObjectInherited::SeZoneTorrid(object) => object.export(section_name, ltx),
      AlifeObjectInherited::SeSmartTerrain(object) => object.export(section_name, ltx),
      AlifeObjectInherited::SeLevelChanger(object) => object.export(section_name, ltx),
      AlifeObjectInherited::SeZoneVisual(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeObjectPhysic(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeHelicopter(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeInventoryBox(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeObjectHangingLamp(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItem(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemExplosive(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemPda(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemAmmo(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemGrenade(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemArtefact(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemWeapon(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemDetector(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemHelmet(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemCustomOutfit(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemWeaponShotgun(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemWeaponMagazined(object) => object.export(section_name, ltx),
      AlifeObjectInherited::CseAlifeItemWeaponMagazinedWGl(object) => {
        object.export(section_name, ltx)
      }
    }
  }
}
