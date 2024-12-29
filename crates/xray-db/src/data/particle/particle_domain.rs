use crate::chunk::reader::ChunkReader;
use crate::data::vector_3d::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleDomain {
  pub domain_type: u32,
  pub coordinates: (Vector3d, Vector3d),
  pub basis: (Vector3d, Vector3d),
  pub radius1: f32,
  pub radius2: f32,
  pub radius1_sqr: f32,
  pub radius2_sqr: f32,
}

impl ParticleDomain {
  /// Read particle domain from chunk reader.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleDomain> {
    Ok(ParticleDomain {
      domain_type: reader.read_u32::<T>()?,
      coordinates: (
        reader.read_f32_3d_vector::<T>()?,
        reader.read_f32_3d_vector::<T>()?,
      ),
      basis: (
        reader.read_f32_3d_vector::<T>()?,
        reader.read_f32_3d_vector::<T>()?,
      ),
      radius1: reader.read_f32::<T>()?,
      radius2: reader.read_f32::<T>()?,
      radius1_sqr: reader.read_f32::<T>()?,
      radius2_sqr: reader.read_f32::<T>()?,
    })
  }
}
