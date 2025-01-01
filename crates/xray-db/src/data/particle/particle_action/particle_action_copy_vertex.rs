use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionCopyVertex {
  pub copy_position: u32,
}

impl ParticleActionReader for ParticleActionCopyVertex {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionCopyVertex> {
    Ok(ParticleActionCopyVertex {
      copy_position: reader.read_u32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      copy_position: read_ini_field("copy_position", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionCopyVertex {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_u32::<ParticlesByteOrder>(self.copy_position)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("copy_position", self.copy_position.to_string());

    Ok(())
  }
}
