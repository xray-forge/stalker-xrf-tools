use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomDisplace {
  pub gen_disp: ParticleDomain,
}

impl ParticleActionRandomDisplace {
  /// Read particle_action random displace.
  pub fn read<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> DatabaseResult<ParticleActionRandomDisplace> {
    Ok(ParticleActionRandomDisplace {
      gen_disp: ParticleDomain::read::<T>(reader)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionRandomDisplace {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("gen_disp", self.gen_disp.to_string());

    Ok(())
  }
}
