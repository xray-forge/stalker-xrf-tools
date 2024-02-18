use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeZoneVisual {
  pub base: AlifeObjectAnomalyZone,
  pub visual: AlifeObjectVisual,
  pub idle_animation: String,
  pub attack_animation: String,
}

impl AlifeObjectInherited<AlifeZoneVisual> for AlifeZoneVisual {
  fn from_chunk(chunk: &mut Chunk) -> AlifeZoneVisual {
    let base: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone::from_chunk(chunk);
    let visual: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);

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

    AlifeZoneVisual {
      base,
      visual,
      idle_animation,
      attack_animation,
    }
  }
}
