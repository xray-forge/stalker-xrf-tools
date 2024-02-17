use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectPhysic {
  pub base: AlifeObjectVisual,
  pub skeleton: AlifeObjectSkeleton,
  pub physic_type: u32,
  pub mass: f32,
  pub fixed_bones: String,
}

impl AlifeObjectInherited<AlifeObjectPhysic> for AlifeObjectPhysic {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectPhysic {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::from_chunk(chunk);

    let physic_type: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let mass: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let fixed_bones: String = chunk.read_null_terminated_string().unwrap();

    AlifeObjectPhysic {
      base,
      skeleton,
      physic_type,
      mass,
      fixed_bones,
    }
  }
}
