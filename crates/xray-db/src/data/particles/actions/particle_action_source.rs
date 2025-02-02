use crate::data::generic::vector_3d::Vector3d;
use crate::data::particles::particle_domain::ParticleDomain;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSource {
  pub position: ParticleDomain,
  pub velocity: ParticleDomain,
  pub rot: ParticleDomain,
  pub size: ParticleDomain,
  pub color: ParticleDomain,
  pub alpha: f32,
  pub particle_rate: f32,
  pub age: f32,
  pub age_sigma: f32,
  pub parent_vel: Vector3d,
  pub parent_motion: f32,
}

impl ChunkReadWrite for ParticleActionSource {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionSource> {
    Ok(Self {
      position: reader.read_xr::<T, _>()?,
      velocity: reader.read_xr::<T, _>()?,
      rot: reader.read_xr::<T, _>()?,
      size: reader.read_xr::<T, _>()?,
      color: reader.read_xr::<T, _>()?,
      alpha: reader.read_f32::<T>()?,
      particle_rate: reader.read_f32::<T>()?,
      age: reader.read_f32::<T>()?,
      age_sigma: reader.read_f32::<T>()?,
      parent_vel: reader.read_xr::<T, _>()?,
      parent_motion: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.position)?;
    writer.write_xr::<T, _>(&self.velocity)?;
    writer.write_xr::<T, _>(&self.rot)?;
    writer.write_xr::<T, _>(&self.size)?;
    writer.write_xr::<T, _>(&self.color)?;
    writer.write_f32::<T>(self.alpha)?;
    writer.write_f32::<T>(self.particle_rate)?;
    writer.write_f32::<T>(self.age)?;
    writer.write_f32::<T>(self.age_sigma)?;
    writer.write_xr::<T, _>(&self.parent_vel)?;
    writer.write_f32::<T>(self.parent_motion)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionSource {
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
      velocity: read_ltx_field("velocity", section)?,
      rot: read_ltx_field("rot", section)?,
      size: read_ltx_field("size", section)?,
      color: read_ltx_field("color", section)?,
      alpha: read_ltx_field("alpha", section)?,
      particle_rate: read_ltx_field("particle_rate", section)?,
      age: read_ltx_field("age", section)?,
      age_sigma: read_ltx_field("age_sigma", section)?,
      parent_vel: read_ltx_field("parent_vel", section)?,
      parent_motion: read_ltx_field("parent_motion", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("position", self.position.to_string())
      .set("velocity", self.velocity.to_string())
      .set("rot", self.rot.to_string())
      .set("size", self.size.to_string())
      .set("color", self.color.to_string())
      .set("alpha", self.alpha.to_string())
      .set("particle_rate", self.particle_rate.to_string())
      .set("age", self.age.to_string())
      .set("age_sigma", self.age_sigma.to_string())
      .set("parent_vel", self.parent_vel.to_string())
      .set("parent_motion", self.parent_motion.to_string());

    Ok(())
  }
}
