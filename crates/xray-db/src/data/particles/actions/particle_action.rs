use crate::constants::META_TYPE_FIELD;
use crate::data::particles::particle_action_generic::ParticleActionGeneric;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_read, ChunkReadWrite, ChunkReadWriteList, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::assert_equal;

/// C++ src/xrParticles/particle_actions_collection_io.cpp
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleAction {
  pub action_type: u32,
  pub action_flags: u32,
  pub data: ParticleActionGeneric,
}

impl ParticleAction {
  pub const META_TYPE: &'static str = "particle_action";
}

impl ChunkReadWriteList for ParticleAction {
  /// Read list of particle action data from chunk reader.
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Vec<Self>> {
    let count: u32 = reader.read_u32::<T>()?;

    let mut actions: Vec<Self> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      actions.push(reader.read_xr::<T, _>().map_err(|error| {
        XRayError::new_parsing_error(format!("Failed to read particle effect action: {}", error))
      })?);
    }

    assert_equal(
      actions.len(),
      count as usize,
      "Should read same count of action as declared in chunk",
    )?;
    assert_chunk_read(reader, "Expect particle actions list chunk to be ended")?;

    Ok(actions)
  }

  /// Write particle action data into chunk writer.
  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, actions: &[Self]) -> XRayResult {
    writer.write_u32::<T>(actions.len() as u32)?;

    for action in actions {
      writer.write_xr::<T, _>(action)?;
    }

    Ok(())
  }
}

impl ChunkReadWrite for ParticleAction {
  /// Read effect particle action data from chunk reader.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      action_type: reader.read_u32::<T>()?,
      action_flags: reader.read_u32::<T>()?,
      data: reader.read_xr::<T, _>().map_err(|error| {
        XRayError::new_parsing_error(format!(
          "Failed to read dynamic particle action data for action: {}",
          error
        ))
      })?,
    })
  }

  /// Write particle action data into chunk writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.action_type)?;
    writer.write_u32::<T>(self.action_flags)?;
    writer.write_xr::<T, _>(&self.data)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleAction {
  /// Import particle action data from provided path.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_equal(
      meta_type.as_str(),
      Self::META_TYPE,
      "Expected corrected meta type field for particle action importing",
    )?;

    Ok(Self {
      action_flags: read_ltx_field("action_flags", section)?,
      action_type: read_ltx_field("action_type", section)?,
      data: ParticleActionGeneric::import(section_name, ltx)?,
    })
  }

  /// Export particle action data into provided path.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("action_flags", self.action_flags.to_string())
      .set("action_type", self.action_type.to_string());

    self.data.export(section_name, ltx)?;

    Ok(())
  }
}
