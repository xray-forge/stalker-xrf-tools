use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particles::particle_domain::ParticleDomain;
use crate::export::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionAvoid {
  pub position: ParticleDomain,
  pub look_ahead: f32,
  pub magnitude: f32,
  pub epsilon: f32,
}

impl ParticleActionReader for ParticleActionAvoid {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      position: ParticleDomain::read::<T>(reader)?,
      look_ahead: reader.read_f32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      position: read_ltx_field("position", section)?,
      look_ahead: read_ltx_field("look_ahead", section)?,
      magnitude: read_ltx_field("magnitude", section)?,
      epsilon: read_ltx_field("epsilon", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionAvoid {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.position.write::<XRayByteOrder>(writer)?;

    writer.write_f32::<XRayByteOrder>(self.look_ahead)?;
    writer.write_f32::<XRayByteOrder>(self.magnitude)?;
    writer.write_f32::<XRayByteOrder>(self.epsilon)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("position", self.position.to_string())
      .set("look_ahead", self.look_ahead.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::meta::particle_action_reader::ParticleActionReader;
  use crate::data::meta::particle_action_writer::ParticleActionWriter;
  use crate::data::particles::actions::particle_action_avoid::ParticleActionAvoid;
  use crate::data::particles::particle_domain::ParticleDomain;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: ParticleActionAvoid = ParticleActionAvoid {
      position: ParticleDomain {
        domain_type: 0,
        coordinates: (
          Vector3d {
            x: 100.50,
            y: 200.60,
            z: 300.70,
          },
          Vector3d {
            x: 10.5,
            y: 20.6,
            z: 30.7,
          },
        ),
        basis: (
          Vector3d {
            x: 2.5,
            y: 3.6,
            z: 4.7,
          },
          Vector3d {
            x: 6.5,
            y: 7.6,
            z: 8.7,
          },
        ),
        radius1: 100.0,
        radius2: 25.0,
        radius1_sqr: 10.0,
        radius2_sqr: 5.0,
      },
      look_ahead: 65.25,
      magnitude: 40.35,
      epsilon: 0.001,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 80);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 80);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 80 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      ParticleActionAvoid::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleActionAvoid = ParticleActionAvoid {
      position: ParticleDomain {
        domain_type: 0,
        coordinates: (
          Vector3d {
            x: 35.5,
            y: 32.6,
            z: 33.7,
          },
          Vector3d {
            x: 105.5,
            y: 260.6,
            z: 305.7,
          },
        ),
        basis: (
          Vector3d {
            x: 252.5,
            y: 345.6,
            z: 400.7,
          },
          Vector3d {
            x: 600.5,
            y: 700.6,
            z: 800.7,
          },
        ),
        radius1: 100.0,
        radius2: 25.0,
        radius1_sqr: 10.0,
        radius2_sqr: 5.0,
      },
      look_ahead: 65.25,
      magnitude: 40.35,
      epsilon: 0.001,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(&get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(ParticleActionAvoid::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticleActionAvoid = ParticleActionAvoid {
      position: ParticleDomain {
        domain_type: 0,
        coordinates: (
          Vector3d {
            x: 1.5,
            y: 2.6,
            z: 3.7,
          },
          Vector3d {
            x: 10.5,
            y: 20.6,
            z: 30.7,
          },
        ),
        basis: (
          Vector3d {
            x: 2.5,
            y: 3.6,
            z: 4.7,
          },
          Vector3d {
            x: 6.5,
            y: 7.6,
            z: 8.7,
          },
        ),
        radius1: 100.0,
        radius2: 25.0,
        radius1_sqr: 10.0,
        radius2_sqr: 5.0,
      },
      look_ahead: 65.25,
      magnitude: 40.35,
      epsilon: 0.001,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ParticleActionAvoid>(&serialized).unwrap()
    );

    Ok(())
  }
}
