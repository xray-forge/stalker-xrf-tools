use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeZoneVisual {
  pub base: AlifeObjectAnomalyZone,
  pub visual: AlifeObjectVisual,
  pub idle_animation: String,
  pub attack_animation: String,
}

impl AlifeObjectInheritedReader<AlifeZoneVisual> for AlifeZoneVisual {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeZoneVisual> {
    let base: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::read_from_chunk::<T>(chunk)?;
    let visual: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let idle_animation: String = chunk
      .has_data()
      .then(|| chunk.read_null_terminated_string().unwrap())
      .or(Some(String::new()))
      .unwrap();

    let attack_animation: String = chunk
      .has_data()
      .then(|| chunk.read_null_terminated_string().unwrap())
      .or(Some(String::new()))
      .unwrap();

    Ok(AlifeZoneVisual {
      base,
      visual,
      idle_animation,
      attack_animation,
    })
  }
}

impl AlifeObjectGeneric for AlifeZoneVisual {}
