use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particle::particle_action::particle_action_avoid::ParticleActionAvoid;
use crate::data::particle::particle_action::particle_action_bounce::ParticleActionBounce;
use crate::data::particle::particle_action::particle_action_copy_vertex::ParticleActionCopyVertex;
use crate::data::particle::particle_action::particle_action_damping::ParticleActionDamping;
use crate::data::particle::particle_action::particle_action_explosion::ParticleActionExplosion;
use crate::data::particle::particle_action::particle_action_follow::ParticleActionFollow;
use crate::data::particle::particle_action::particle_action_gravitate::ParticleActionGravitate;
use crate::data::particle::particle_action::particle_action_gravity::ParticleActionGravity;
use crate::data::particle::particle_action::particle_action_jet::ParticleActionJet;
use crate::data::particle::particle_action::particle_action_kill_old::ParticleActionKillOld;
use crate::data::particle::particle_action::particle_action_match_velocity::ParticleActionMatchVelocity;
use crate::data::particle::particle_action::particle_action_move::ParticleActionMove;
use crate::data::particle::particle_action::particle_action_orbit_line::ParticleActionOrbitLine;
use crate::data::particle::particle_action::particle_action_orbit_point::ParticleActionOrbitPoint;
use crate::data::particle::particle_action::particle_action_random_acceleration::ParticleActionRandomAcceleration;
use crate::data::particle::particle_action::particle_action_random_displace::ParticleActionRandomDisplace;
use crate::data::particle::particle_action::particle_action_random_velocity::ParticleActionRandomVelocity;
use crate::data::particle::particle_action::particle_action_restore::ParticleActionRestore;
use crate::data::particle::particle_action::particle_action_scatter::ParticleActionScatter;
use crate::data::particle::particle_action::particle_action_sink::ParticleActionSink;
use crate::data::particle::particle_action::particle_action_sink_velocity::ParticleActionSinkVelocity;
use crate::data::particle::particle_action::particle_action_source::ParticleActionSource;
use crate::data::particle::particle_action::particle_action_speed_limit::ParticleActionSpeedLimit;
use crate::data::particle::particle_action::particle_action_target_color::ParticleActionTargetColor;
use crate::data::particle::particle_action::particle_action_target_rotate::ParticleActionTargetRotate;
use crate::data::particle::particle_action::particle_action_target_size::ParticleActionTargetSize;
use crate::data::particle::particle_action::particle_action_target_velocity::ParticleActionTargetVelocity;
use crate::data::particle::particle_action::particle_action_turbulence::ParticleActionTurbulence;
use crate::data::particle::particle_action::particle_action_vortex::ParticleActionVortex;
use crate::error::database_parse_error::DatabaseParseError;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use enum_map::Enum;
use xray_chunk::ChunkReader;
use xray_ltx::Ltx;

#[derive(Copy, Clone, Debug, Enum, PartialEq)]
pub enum ParticleActionType {
  Avoid = 0,
  Bounce = 1,
  CallActionList = 2,
  CopyVertex = 3,
  Damping = 4,
  Explosion = 5,
  Follow = 6,
  Gravitate = 7,
  Gravity = 8,
  Jet = 9,
  KillOld = 10,
  MatchVelocity = 11,
  Move = 12,
  OrbitLine = 13,
  OrbitPoint = 14,
  RandomAccel = 15,
  RandomDisplace = 16,
  RandomVelocity = 17,
  Restore = 18,
  Sink = 19,
  SinkVelocity = 20,
  Source = 21,
  SpeedLimit = 22,
  TargetColor = 23,
  TargetSize = 24,
  TargetRotate = 25,
  TargetRotateD = 26,
  TargetVelocity = 27,
  TargetVelocityD = 28,
  Vortex = 29,
  Turbulence = 30,
  Scatter = 31,
  Unknown = -1,
}

impl ParticleActionType {
  /// Transform u32 to enumeration value.
  pub fn from_u32(action_type: u32) -> Self {
    // todo: Probably use From trait?
    match action_type {
      0 => Self::Avoid,
      1 => Self::Bounce,
      2 => Self::CallActionList,
      3 => Self::CopyVertex,
      4 => Self::Damping,
      5 => Self::Explosion,
      6 => Self::Follow,
      7 => Self::Gravitate,
      8 => Self::Gravity,
      9 => Self::Jet,
      10 => Self::KillOld,
      11 => Self::MatchVelocity,
      12 => Self::Move,
      13 => Self::OrbitLine,
      14 => Self::OrbitPoint,
      15 => Self::RandomAccel,
      16 => Self::RandomDisplace,
      17 => Self::RandomVelocity,
      18 => Self::Restore,
      19 => Self::Sink,
      20 => Self::SinkVelocity,
      21 => Self::Source,
      22 => Self::SpeedLimit,
      23 => Self::TargetColor,
      24 => Self::TargetSize,
      25 => Self::TargetRotate,
      26 => Self::TargetRotateD,
      27 => Self::TargetVelocity,
      28 => Self::TargetVelocityD,
      29 => Self::Vortex,
      30 => Self::Turbulence,
      31 => Self::Scatter,
      _ => Self::Unknown,
    }
  }

  /// Read particle action data from chunk based on action type.
  pub fn read_by_particle_type<T: ByteOrder>(
    reader: &mut ChunkReader,
    particle_action_type: Self,
  ) -> DatabaseResult<Box<dyn ParticleActionWriter>> {
    Ok(match particle_action_type {
      Self::Avoid => Box::new(ParticleActionAvoid::read::<T>(reader)?),
      Self::Bounce => Box::new(ParticleActionBounce::read::<T>(reader)?),
      Self::CopyVertex => Box::new(ParticleActionCopyVertex::read::<T>(reader)?),
      Self::Damping => Box::new(ParticleActionDamping::read::<T>(reader)?),
      Self::Explosion => Box::new(ParticleActionExplosion::read::<T>(reader)?),
      Self::Follow => Box::new(ParticleActionFollow::read::<T>(reader)?),
      Self::Gravitate => Box::new(ParticleActionGravitate::read::<T>(reader)?),
      Self::Gravity => Box::new(ParticleActionGravity::read::<T>(reader)?),
      Self::Jet => Box::new(ParticleActionJet::read::<T>(reader)?),
      Self::KillOld => Box::new(ParticleActionKillOld::read::<T>(reader)?),
      Self::MatchVelocity => Box::new(ParticleActionMatchVelocity::read::<T>(reader)?),
      Self::Move => Box::new(ParticleActionMove::read::<T>(reader)?),
      Self::OrbitLine => Box::new(ParticleActionOrbitLine::read::<T>(reader)?),
      Self::OrbitPoint => Box::new(ParticleActionOrbitPoint::read::<T>(reader)?),
      Self::RandomAccel => Box::new(ParticleActionRandomAcceleration::read::<T>(reader)?),
      Self::RandomDisplace => Box::new(ParticleActionRandomDisplace::read::<T>(reader)?),
      Self::RandomVelocity => Box::new(ParticleActionRandomVelocity::read::<T>(reader)?),
      Self::Restore => Box::new(ParticleActionRestore::read::<T>(reader)?),
      Self::Sink => Box::new(ParticleActionSink::read::<T>(reader)?),
      Self::SinkVelocity => Box::new(ParticleActionSinkVelocity::read::<T>(reader)?),
      Self::Source => Box::new(ParticleActionSource::read::<T>(reader)?),
      Self::SpeedLimit => Box::new(ParticleActionSpeedLimit::read::<T>(reader)?),
      Self::TargetColor => Box::new(ParticleActionTargetColor::read::<T>(reader)?),
      Self::TargetSize => Box::new(ParticleActionTargetSize::read::<T>(reader)?),
      Self::TargetRotate | Self::TargetRotateD => {
        Box::new(ParticleActionTargetRotate::read::<T>(reader)?)
      }
      Self::TargetVelocity | Self::TargetVelocityD => {
        Box::new(ParticleActionTargetVelocity::read::<T>(reader)?)
      }
      Self::Vortex => Box::new(ParticleActionVortex::read::<T>(reader)?),
      Self::Turbulence => Box::new(ParticleActionTurbulence::read::<T>(reader)?),
      Self::Scatter => Box::new(ParticleActionScatter::read::<T>(reader)?),
      Self::Unknown | Self::CallActionList => {
        return Err(DatabaseParseError::new_database_error(format!(
          "Not implemented parser for particle action reading: {:?}",
          particle_action_type
        )));
      }
    })
  }

  /// Read particle action data from chunk based on action type.
  pub fn import_by_particle_type(
    particle_action_type: Self,
    section_name: &str,
    ltx: &Ltx,
  ) -> DatabaseResult<Box<dyn ParticleActionWriter>> {
    Ok(match particle_action_type {
      Self::Avoid => Box::new(ParticleActionAvoid::import(section_name, ltx)?),
      Self::Bounce => Box::new(ParticleActionBounce::import(section_name, ltx)?),
      Self::CopyVertex => Box::new(ParticleActionCopyVertex::import(section_name, ltx)?),
      Self::Damping => Box::new(ParticleActionDamping::import(section_name, ltx)?),
      Self::Explosion => Box::new(ParticleActionExplosion::import(section_name, ltx)?),
      Self::Follow => Box::new(ParticleActionFollow::import(section_name, ltx)?),
      Self::Gravitate => Box::new(ParticleActionGravitate::import(section_name, ltx)?),
      Self::Gravity => Box::new(ParticleActionGravity::import(section_name, ltx)?),
      Self::Jet => Box::new(ParticleActionJet::import(section_name, ltx)?),
      Self::KillOld => Box::new(ParticleActionKillOld::import(section_name, ltx)?),
      Self::MatchVelocity => Box::new(ParticleActionMatchVelocity::import(section_name, ltx)?),
      Self::Move => Box::new(ParticleActionMove::import(section_name, ltx)?),
      Self::OrbitLine => Box::new(ParticleActionOrbitLine::import(section_name, ltx)?),
      Self::OrbitPoint => Box::new(ParticleActionOrbitPoint::import(section_name, ltx)?),
      Self::RandomAccel => Box::new(ParticleActionRandomAcceleration::import(section_name, ltx)?),
      Self::RandomDisplace => Box::new(ParticleActionRandomDisplace::import(section_name, ltx)?),
      Self::RandomVelocity => Box::new(ParticleActionRandomVelocity::import(section_name, ltx)?),
      Self::Restore => Box::new(ParticleActionRestore::import(section_name, ltx)?),
      Self::Sink => Box::new(ParticleActionSink::import(section_name, ltx)?),
      Self::SinkVelocity => Box::new(ParticleActionSinkVelocity::import(section_name, ltx)?),
      Self::Source => Box::new(ParticleActionSource::import(section_name, ltx)?),
      Self::SpeedLimit => Box::new(ParticleActionSpeedLimit::import(section_name, ltx)?),
      Self::TargetColor => Box::new(ParticleActionTargetColor::import(section_name, ltx)?),
      Self::TargetSize => Box::new(ParticleActionTargetSize::import(section_name, ltx)?),
      Self::TargetRotate | Self::TargetRotateD => {
        Box::new(ParticleActionTargetRotate::import(section_name, ltx)?)
      }
      Self::TargetVelocity | Self::TargetVelocityD => {
        Box::new(ParticleActionTargetVelocity::import(section_name, ltx)?)
      }
      Self::Vortex => Box::new(ParticleActionVortex::import(section_name, ltx)?),
      Self::Turbulence => Box::new(ParticleActionTurbulence::import(section_name, ltx)?),
      Self::Scatter => Box::new(ParticleActionScatter::import(section_name, ltx)?),
      Self::Unknown | Self::CallActionList => {
        return Err(DatabaseParseError::new_database_error(format!(
          "Not implemented parser for particle action importing: {:?}",
          particle_action_type
        )));
      }
    })
  }
}
