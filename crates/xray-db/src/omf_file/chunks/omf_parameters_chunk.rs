use crate::data::ogf::ogf_motion_definition::OgfMotionDefinition;
use crate::data::ogf::ogf_part::OgfPart;
use crate::{DatabaseError, DatabaseResult, OmfFile};
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct OmfParametersChunk {
  pub version: u16,
  pub parts: Vec<OgfPart>,
  pub motions: Vec<OgfMotionDefinition>,
}

impl OmfParametersChunk {
  pub const CHUNK_ID: u32 = 15; // 0x14, 0xF

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading parameters chunk: {:?} bytes",
      reader.read_bytes_remain()
    );

    let version: u16 = reader.read_u16::<T>()?;

    if !OmfFile::SUPPORTED_VERSIONS.contains(&version) {
      return Err(DatabaseError::new_not_implemented_error(format!(
        "Unexpected parameters version {version} on read, only versions {:?} is implemented",
        OmfFile::SUPPORTED_VERSIONS
      )));
    }

    let parts: Vec<OgfPart> = OgfPart::read_list::<T>(reader).map_err(|error| {
      DatabaseError::new_read_error(format!("Failed to read ogf parts: {error}"))
    })?;

    let motions: Vec<OgfMotionDefinition> = OgfMotionDefinition::read_list::<T>(reader, version)
      .map_err(|error| {
        DatabaseError::new_read_error(format!("Failed to read ogf motion definitions: {error}"))
      })?;

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
