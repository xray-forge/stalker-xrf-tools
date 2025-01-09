use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::ogf::ogf_motion_definition::OgfMotionDefinition;
use crate::data::ogf::ogf_part::OgfPart;
use crate::{DatabaseNotImplementedError, DatabaseResult};
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OmfParametersChunk {
  pub version: u16,
  pub parts: Vec<OgfPart>,
  pub motions: Vec<OgfMotionDefinition>,
}

impl OmfParametersChunk {
  pub const CHUNK_ID: u32 = 15; // 0x14, 0xF
  pub const SUPPORTED_VERSION: u16 = 4;

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading parameters chunk: {:?} bytes",
      reader.read_bytes_remain()
    );

    let version: u16 = reader.read_u16::<T>()?;

    if version != Self::SUPPORTED_VERSION {
      return Err(DatabaseNotImplementedError::new_database_error(format!(
        "Unexpected parameters version {version}, only version {} is implemented",
        Self::SUPPORTED_VERSION
      )));
    }

    let parts: Vec<OgfPart> = OgfPart::read_list::<T>(reader)?;
    let motions: Vec<OgfMotionDefinition> = OgfMotionDefinition::read_list::<T>(reader)?;

    assert!(
      reader.is_ended(),
      "Expect all data to be read from omf parameters chunk, {} remain",
      reader.read_bytes_remain()
    );

    Ok(Self {
      version,
      parts,
      motions,
    })
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}

impl OmfParametersChunk {
  pub fn get_bones_count(&self) -> usize {
    self.parts.iter().map(|it| it.bones.len()).sum()
  }
}
