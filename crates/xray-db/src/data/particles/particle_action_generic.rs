use crate::data::particles::actions::particle_action_avoid::ParticleActionAvoid;
use crate::data::particles::actions::particle_action_bounce::ParticleActionBounce;
use crate::data::particles::actions::particle_action_copy_vertex::ParticleActionCopyVertex;
use crate::data::particles::actions::particle_action_damping::ParticleActionDamping;
use crate::data::particles::actions::particle_action_explosion::ParticleActionExplosion;
use crate::data::particles::actions::particle_action_follow::ParticleActionFollow;
use crate::data::particles::actions::particle_action_gravitate::ParticleActionGravitate;
use crate::data::particles::actions::particle_action_gravity::ParticleActionGravity;
use crate::data::particles::actions::particle_action_jet::ParticleActionJet;
use crate::data::particles::actions::particle_action_kill_old::ParticleActionKillOld;
use crate::data::particles::actions::particle_action_match_velocity::ParticleActionMatchVelocity;
use crate::data::particles::actions::particle_action_move::ParticleActionMove;
use crate::data::particles::actions::particle_action_orbit_line::ParticleActionOrbitLine;
use crate::data::particles::actions::particle_action_orbit_point::ParticleActionOrbitPoint;
use crate::data::particles::actions::particle_action_random_acceleration::ParticleActionRandomAcceleration;
use crate::data::particles::actions::particle_action_random_displace::ParticleActionRandomDisplace;
use crate::data::particles::actions::particle_action_random_velocity::ParticleActionRandomVelocity;
use crate::data::particles::actions::particle_action_restore::ParticleActionRestore;
use crate::data::particles::actions::particle_action_scatter::ParticleActionScatter;
use crate::data::particles::actions::particle_action_sink::ParticleActionSink;
use crate::data::particles::actions::particle_action_sink_velocity::ParticleActionSinkVelocity;
use crate::data::particles::actions::particle_action_source::ParticleActionSource;
use crate::data::particles::actions::particle_action_speed_limit::ParticleActionSpeedLimit;
use crate::data::particles::actions::particle_action_target_color::ParticleActionTargetColor;
use crate::data::particles::actions::particle_action_target_rotate::ParticleActionTargetRotate;
use crate::data::particles::actions::particle_action_target_size::ParticleActionTargetSize;
use crate::data::particles::actions::particle_action_target_velocity::ParticleActionTargetVelocity;
use crate::data::particles::actions::particle_action_turbulence::ParticleActionTurbulence;
use crate::data::particles::actions::particle_action_vortex::ParticleActionVortex;
use crate::data::particles::particle_action_type::ParticleActionType;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ParticleActionGeneric {
  Avoid(Box<ParticleActionAvoid>),
  Bounce(Box<ParticleActionBounce>),
  CopyVertex(Box<ParticleActionCopyVertex>),
  Damping(Box<ParticleActionDamping>),
  Explosion(Box<ParticleActionExplosion>),
  Follow(Box<ParticleActionFollow>),
  Gravitate(Box<ParticleActionGravitate>),
  Gravity(Box<ParticleActionGravity>),
  Jet(Box<ParticleActionJet>),
  KillOld(Box<ParticleActionKillOld>),
  MatchVelocity(Box<ParticleActionMatchVelocity>),
  Move(Box<ParticleActionMove>),
  OrbitLine(Box<ParticleActionOrbitLine>),
  OrbitPoint(Box<ParticleActionOrbitPoint>),
  RandomAccel(Box<ParticleActionRandomAcceleration>),
  RandomDisplace(Box<ParticleActionRandomDisplace>),
  RandomVelocity(Box<ParticleActionRandomVelocity>),
  Restore(Box<ParticleActionRestore>),
  Sink(Box<ParticleActionSink>),
  SinkVelocity(Box<ParticleActionSinkVelocity>),
  Source(Box<ParticleActionSource>),
  SpeedLimit(Box<ParticleActionSpeedLimit>),
  TargetColor(Box<ParticleActionTargetColor>),
  TargetSize(Box<ParticleActionTargetSize>),
  TargetRotate(Box<ParticleActionTargetRotate>),
  TargetVelocity(Box<ParticleActionTargetVelocity>),
  Vortex(Box<ParticleActionVortex>),
  Turbulence(Box<ParticleActionTurbulence>),
  Scatter(Box<ParticleActionScatter>),
}

impl ChunkReadWrite for ParticleActionGeneric {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let action_type_raw: u32 = reader.read_u32::<T>()?;
    let action_type: ParticleActionType = ParticleActionType::from(action_type_raw);

    Ok(match action_type {
      ParticleActionType::Avoid => Self::Avoid(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Bounce => Self::Bounce(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::CopyVertex => Self::CopyVertex(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Damping => Self::Damping(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Explosion => Self::Explosion(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Follow => Self::Follow(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Gravitate => Self::Gravitate(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Gravity => Self::Gravity(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Jet => Self::Jet(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::KillOld => Self::KillOld(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::MatchVelocity => Self::MatchVelocity(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Move => Self::Move(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::OrbitLine => Self::OrbitLine(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::OrbitPoint => Self::OrbitPoint(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::RandomAccel => Self::RandomAccel(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::RandomDisplace => {
        Self::RandomDisplace(Box::new(reader.read_xr::<T, _>()?))
      }
      ParticleActionType::RandomVelocity => {
        Self::RandomVelocity(Box::new(reader.read_xr::<T, _>()?))
      }
      ParticleActionType::Restore => Self::Restore(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Sink => Self::Sink(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::SinkVelocity => Self::SinkVelocity(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Source => Self::Source(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::SpeedLimit => Self::SpeedLimit(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::TargetColor => Self::TargetColor(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::TargetSize => Self::TargetSize(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::TargetRotate | ParticleActionType::TargetRotateD => {
        Self::TargetRotate(Box::new(reader.read_xr::<T, _>()?))
      }
      ParticleActionType::TargetVelocity | ParticleActionType::TargetVelocityD => {
        Self::TargetVelocity(Box::new(reader.read_xr::<T, _>()?))
      }
      ParticleActionType::Vortex => Self::Vortex(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Turbulence => Self::Turbulence(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Scatter => Self::Scatter(Box::new(reader.read_xr::<T, _>()?)),
      ParticleActionType::Unknown | ParticleActionType::CallActionList => {
        return Err(XRayError::new_unexpected_error(format!(
          "Unexpected action type provided for reading: {}",
          action_type
        )))
      }
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(ParticleActionType::get_action_type(self) as u32)?;

    match self {
      ParticleActionGeneric::Avoid(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Bounce(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::CopyVertex(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Damping(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Explosion(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Follow(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Gravitate(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Gravity(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Jet(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::KillOld(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::MatchVelocity(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Move(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::OrbitLine(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::OrbitPoint(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::RandomAccel(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::RandomDisplace(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::RandomVelocity(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Restore(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Sink(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::SinkVelocity(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Source(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::SpeedLimit(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::TargetColor(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::TargetSize(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::TargetRotate(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::TargetVelocity(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Vortex(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Turbulence(action) => writer.write_xr::<T, _>(action.deref()),
      ParticleActionGeneric::Scatter(action) => writer.write_xr::<T, _>(action.deref()),
    }
  }
}

impl LtxImportExport for ParticleActionGeneric {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;
    let action_type_raw: u32 = read_ltx_field("action_type", section)?;
    let action_type: ParticleActionType = ParticleActionType::from(action_type_raw);

    Ok(match action_type {
      ParticleActionType::Avoid => {
        Self::Avoid(Box::new(ParticleActionAvoid::import(section_name, ltx)?))
      }
      ParticleActionType::Bounce => {
        Self::Bounce(Box::new(ParticleActionBounce::import(section_name, ltx)?))
      }
      ParticleActionType::CopyVertex => Self::CopyVertex(Box::new(
        ParticleActionCopyVertex::import(section_name, ltx)?,
      )),
      ParticleActionType::Damping => {
        Self::Damping(Box::new(ParticleActionDamping::import(section_name, ltx)?))
      }
      ParticleActionType::Explosion => Self::Explosion(Box::new(ParticleActionExplosion::import(
        section_name,
        ltx,
      )?)),
      ParticleActionType::Follow => {
        Self::Follow(Box::new(ParticleActionFollow::import(section_name, ltx)?))
      }
      ParticleActionType::Gravitate => Self::Gravitate(Box::new(ParticleActionGravitate::import(
        section_name,
        ltx,
      )?)),
      ParticleActionType::Gravity => {
        Self::Gravity(Box::new(ParticleActionGravity::import(section_name, ltx)?))
      }
      ParticleActionType::Jet => Self::Jet(Box::new(ParticleActionJet::import(section_name, ltx)?)),
      ParticleActionType::KillOld => {
        Self::KillOld(Box::new(ParticleActionKillOld::import(section_name, ltx)?))
      }
      ParticleActionType::MatchVelocity => Self::MatchVelocity(Box::new(
        ParticleActionMatchVelocity::import(section_name, ltx)?,
      )),
      ParticleActionType::Move => {
        Self::Move(Box::new(ParticleActionMove::import(section_name, ltx)?))
      }
      ParticleActionType::OrbitLine => Self::OrbitLine(Box::new(ParticleActionOrbitLine::import(
        section_name,
        ltx,
      )?)),
      ParticleActionType::OrbitPoint => Self::OrbitPoint(Box::new(
        ParticleActionOrbitPoint::import(section_name, ltx)?,
      )),
      ParticleActionType::RandomAccel => Self::RandomAccel(Box::new(
        ParticleActionRandomAcceleration::import(section_name, ltx)?,
      )),
      ParticleActionType::RandomDisplace => Self::RandomDisplace(Box::new(
        ParticleActionRandomDisplace::import(section_name, ltx)?,
      )),
      ParticleActionType::RandomVelocity => Self::RandomVelocity(Box::new(
        ParticleActionRandomVelocity::import(section_name, ltx)?,
      )),
      ParticleActionType::Restore => {
        Self::Restore(Box::new(ParticleActionRestore::import(section_name, ltx)?))
      }
      ParticleActionType::Sink => {
        Self::Sink(Box::new(ParticleActionSink::import(section_name, ltx)?))
      }
      ParticleActionType::SinkVelocity => Self::SinkVelocity(Box::new(
        ParticleActionSinkVelocity::import(section_name, ltx)?,
      )),
      ParticleActionType::Source => {
        Self::Source(Box::new(ParticleActionSource::import(section_name, ltx)?))
      }
      ParticleActionType::SpeedLimit => Self::SpeedLimit(Box::new(
        ParticleActionSpeedLimit::import(section_name, ltx)?,
      )),
      ParticleActionType::TargetColor => Self::TargetColor(Box::new(
        ParticleActionTargetColor::import(section_name, ltx)?,
      )),
      ParticleActionType::TargetSize => Self::TargetSize(Box::new(
        ParticleActionTargetSize::import(section_name, ltx)?,
      )),
      ParticleActionType::TargetRotate | ParticleActionType::TargetRotateD => Self::TargetRotate(
        Box::new(ParticleActionTargetRotate::import(section_name, ltx)?),
      ),
      ParticleActionType::TargetVelocity | ParticleActionType::TargetVelocityD => {
        Self::TargetVelocity(Box::new(ParticleActionTargetVelocity::import(
          section_name,
          ltx,
        )?))
      }
      ParticleActionType::Vortex => {
        Self::Vortex(Box::new(ParticleActionVortex::import(section_name, ltx)?))
      }
      ParticleActionType::Turbulence => Self::Turbulence(Box::new(
        ParticleActionTurbulence::import(section_name, ltx)?,
      )),
      ParticleActionType::Scatter => {
        Self::Scatter(Box::new(ParticleActionScatter::import(section_name, ltx)?))
      }
      ParticleActionType::Unknown | ParticleActionType::CallActionList => {
        return Err(XRayError::new_unexpected_error(format!(
          "Unexpected action type provided for reading: {}",
          action_type
        )))
      }
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    match self {
      ParticleActionGeneric::Avoid(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Bounce(action) => action.export(section_name, ltx),
      ParticleActionGeneric::CopyVertex(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Damping(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Explosion(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Follow(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Gravitate(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Gravity(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Jet(action) => action.export(section_name, ltx),
      ParticleActionGeneric::KillOld(action) => action.export(section_name, ltx),
      ParticleActionGeneric::MatchVelocity(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Move(action) => action.export(section_name, ltx),
      ParticleActionGeneric::OrbitLine(action) => action.export(section_name, ltx),
      ParticleActionGeneric::OrbitPoint(action) => action.export(section_name, ltx),
      ParticleActionGeneric::RandomAccel(action) => action.export(section_name, ltx),
      ParticleActionGeneric::RandomDisplace(action) => action.export(section_name, ltx),
      ParticleActionGeneric::RandomVelocity(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Restore(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Sink(action) => action.export(section_name, ltx),
      ParticleActionGeneric::SinkVelocity(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Source(action) => action.export(section_name, ltx),
      ParticleActionGeneric::SpeedLimit(action) => action.export(section_name, ltx),
      ParticleActionGeneric::TargetColor(action) => action.export(section_name, ltx),
      ParticleActionGeneric::TargetSize(action) => action.export(section_name, ltx),
      ParticleActionGeneric::TargetRotate(action) => action.export(section_name, ltx),
      ParticleActionGeneric::TargetVelocity(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Vortex(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Turbulence(action) => action.export(section_name, ltx),
      ParticleActionGeneric::Scatter(action) => action.export(section_name, ltx),
    }
  }
}
