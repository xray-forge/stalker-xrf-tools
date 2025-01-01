use crate::chunk::reader::ChunkReader;
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
use xray_ltx::Ltx;

#[derive(Clone, Debug, Enum, PartialEq)]
pub enum ParticleActionType {
  PAAvoidID = 0,
  PABounceID = 1,
  PACallActionListID = 2,
  PACopyVertexBID = 3,
  PADampingID = 4,
  PAExplosionID = 5,
  PAFollowID = 6,
  PAGravitateID = 7,
  PAGravityID = 8,
  PAJetID = 9,
  PAKillOldID = 10,
  PAMatchVelocityID = 11,
  PAMoveID = 12,
  PAOrbitLineID = 13,
  PAOrbitPointID = 14,
  PARandomAccelID = 15,
  PARandomDisplaceID = 16,
  PARandomVelocityID = 17,
  PARestoreID = 18,
  PASinkID = 19,
  PASinkVelocityID = 20,
  PASourceID = 21,
  PASpeedLimitID = 22,
  PATargetColorID = 23,
  PATargetSizeID = 24,
  PATargetRotateID = 25,
  PATargetRotateDID = 26,
  PATargetVelocityID = 27,
  PATargetVelocityDID = 28,
  PAVortexID = 29,
  PATurbulenceID = 30,
  PAScatterID = 31,
  Unknown = -1,
}

impl ParticleActionType {
  /// Transform u32 to enumeration value.
  pub fn from_u32(action_type: u32) -> ParticleActionType {
    match action_type {
      0 => ParticleActionType::PAAvoidID,
      1 => ParticleActionType::PABounceID,
      2 => ParticleActionType::PACallActionListID,
      3 => ParticleActionType::PACopyVertexBID,
      4 => ParticleActionType::PADampingID,
      5 => ParticleActionType::PAExplosionID,
      6 => ParticleActionType::PAFollowID,
      7 => ParticleActionType::PAGravitateID,
      8 => ParticleActionType::PAGravityID,
      9 => ParticleActionType::PAJetID,
      10 => ParticleActionType::PAKillOldID,
      11 => ParticleActionType::PAMatchVelocityID,
      12 => ParticleActionType::PAMoveID,
      13 => ParticleActionType::PAOrbitLineID,
      14 => ParticleActionType::PAOrbitPointID,
      15 => ParticleActionType::PARandomAccelID,
      16 => ParticleActionType::PARandomDisplaceID,
      17 => ParticleActionType::PARandomVelocityID,
      18 => ParticleActionType::PARestoreID,
      19 => ParticleActionType::PASinkID,
      20 => ParticleActionType::PASinkVelocityID,
      21 => ParticleActionType::PASourceID,
      22 => ParticleActionType::PASpeedLimitID,
      23 => ParticleActionType::PATargetColorID,
      24 => ParticleActionType::PATargetSizeID,
      25 => ParticleActionType::PATargetRotateID,
      26 => ParticleActionType::PATargetRotateDID,
      27 => ParticleActionType::PATargetVelocityID,
      28 => ParticleActionType::PATargetVelocityDID,
      29 => ParticleActionType::PAVortexID,
      30 => ParticleActionType::PATurbulenceID,
      31 => ParticleActionType::PAScatterID,
      _ => ParticleActionType::Unknown,
    }
  }

  /// Read particle action data from chunk based on action type.
  pub fn read_by_particle_type<T: ByteOrder>(
    reader: &mut ChunkReader,
    particle_action_type: ParticleActionType,
  ) -> DatabaseResult<Box<dyn ParticleActionWriter>> {
    Ok(match particle_action_type {
      ParticleActionType::PAAvoidID => Box::new(ParticleActionAvoid::read::<T>(reader)?),
      ParticleActionType::PABounceID => Box::new(ParticleActionBounce::read::<T>(reader)?),
      ParticleActionType::PACopyVertexBID => Box::new(ParticleActionCopyVertex::read::<T>(reader)?),
      ParticleActionType::PADampingID => Box::new(ParticleActionDamping::read::<T>(reader)?),
      ParticleActionType::PAExplosionID => Box::new(ParticleActionExplosion::read::<T>(reader)?),
      ParticleActionType::PAFollowID => Box::new(ParticleActionFollow::read::<T>(reader)?),
      ParticleActionType::PAGravitateID => Box::new(ParticleActionGravitate::read::<T>(reader)?),
      ParticleActionType::PAGravityID => Box::new(ParticleActionGravity::read::<T>(reader)?),
      ParticleActionType::PAJetID => Box::new(ParticleActionJet::read::<T>(reader)?),
      ParticleActionType::PAKillOldID => Box::new(ParticleActionKillOld::read::<T>(reader)?),
      ParticleActionType::PAMatchVelocityID => {
        Box::new(ParticleActionMatchVelocity::read::<T>(reader)?)
      }
      ParticleActionType::PAMoveID => Box::new(ParticleActionMove::read::<T>(reader)?),
      ParticleActionType::PAOrbitLineID => Box::new(ParticleActionOrbitLine::read::<T>(reader)?),
      ParticleActionType::PAOrbitPointID => Box::new(ParticleActionOrbitPoint::read::<T>(reader)?),
      ParticleActionType::PARandomAccelID => {
        Box::new(ParticleActionRandomAcceleration::read::<T>(reader)?)
      }
      ParticleActionType::PARandomDisplaceID => {
        Box::new(ParticleActionRandomDisplace::read::<T>(reader)?)
      }
      ParticleActionType::PARandomVelocityID => {
        Box::new(ParticleActionRandomVelocity::read::<T>(reader)?)
      }
      ParticleActionType::PARestoreID => Box::new(ParticleActionRestore::read::<T>(reader)?),
      ParticleActionType::PASinkID => Box::new(ParticleActionSink::read::<T>(reader)?),
      ParticleActionType::PASinkVelocityID => {
        Box::new(ParticleActionSinkVelocity::read::<T>(reader)?)
      }
      ParticleActionType::PASourceID => Box::new(ParticleActionSource::read::<T>(reader)?),
      ParticleActionType::PASpeedLimitID => Box::new(ParticleActionSpeedLimit::read::<T>(reader)?),
      ParticleActionType::PATargetColorID => {
        Box::new(ParticleActionTargetColor::read::<T>(reader)?)
      }
      ParticleActionType::PATargetSizeID => Box::new(ParticleActionTargetSize::read::<T>(reader)?),
      ParticleActionType::PATargetRotateID | ParticleActionType::PATargetRotateDID => {
        Box::new(ParticleActionTargetRotate::read::<T>(reader)?)
      }
      ParticleActionType::PATargetVelocityID | ParticleActionType::PATargetVelocityDID => {
        Box::new(ParticleActionTargetVelocity::read::<T>(reader)?)
      }
      ParticleActionType::PAVortexID => Box::new(ParticleActionVortex::read::<T>(reader)?),
      ParticleActionType::PATurbulenceID => Box::new(ParticleActionTurbulence::read::<T>(reader)?),
      ParticleActionType::PAScatterID => Box::new(ParticleActionScatter::read::<T>(reader)?),
      ParticleActionType::Unknown | ParticleActionType::PACallActionListID => {
        return Err(DatabaseParseError::new_database_error(format!(
          "Not implemented parser for particle action reading: {:?}",
          particle_action_type
        )));
      }
    })
  }

  /// Read particle action data from chunk based on action type.
  pub fn import_by_particle_type(
    particle_action_type: ParticleActionType,
    section_name: &str,
    ini: &Ltx,
  ) -> DatabaseResult<Box<dyn ParticleActionWriter>> {
    Ok(match particle_action_type {
      ParticleActionType::PAAvoidID => Box::new(ParticleActionAvoid::import(section_name, ini)?),
      ParticleActionType::PABounceID => Box::new(ParticleActionBounce::import(section_name, ini)?),
      ParticleActionType::PACopyVertexBID => {
        Box::new(ParticleActionCopyVertex::import(section_name, ini)?)
      }
      ParticleActionType::PADampingID => {
        Box::new(ParticleActionDamping::import(section_name, ini)?)
      }
      ParticleActionType::PAExplosionID => {
        Box::new(ParticleActionExplosion::import(section_name, ini)?)
      }
      ParticleActionType::PAFollowID => Box::new(ParticleActionFollow::import(section_name, ini)?),
      ParticleActionType::PAGravitateID => {
        Box::new(ParticleActionGravitate::import(section_name, ini)?)
      }
      ParticleActionType::PAGravityID => {
        Box::new(ParticleActionGravity::import(section_name, ini)?)
      }
      ParticleActionType::PAJetID => Box::new(ParticleActionJet::import(section_name, ini)?),
      ParticleActionType::PAKillOldID => {
        Box::new(ParticleActionKillOld::import(section_name, ini)?)
      }
      ParticleActionType::PAMatchVelocityID => {
        Box::new(ParticleActionMatchVelocity::import(section_name, ini)?)
      }
      ParticleActionType::PAMoveID => Box::new(ParticleActionMove::import(section_name, ini)?),
      ParticleActionType::PAOrbitLineID => {
        Box::new(ParticleActionOrbitLine::import(section_name, ini)?)
      }
      ParticleActionType::PAOrbitPointID => {
        Box::new(ParticleActionOrbitPoint::import(section_name, ini)?)
      }
      ParticleActionType::PARandomAccelID => {
        Box::new(ParticleActionRandomAcceleration::import(section_name, ini)?)
      }
      ParticleActionType::PARandomDisplaceID => {
        Box::new(ParticleActionRandomDisplace::import(section_name, ini)?)
      }
      ParticleActionType::PARandomVelocityID => {
        Box::new(ParticleActionRandomVelocity::import(section_name, ini)?)
      }
      ParticleActionType::PARestoreID => {
        Box::new(ParticleActionRestore::import(section_name, ini)?)
      }
      ParticleActionType::PASinkID => Box::new(ParticleActionSink::import(section_name, ini)?),
      ParticleActionType::PASinkVelocityID => {
        Box::new(ParticleActionSinkVelocity::import(section_name, ini)?)
      }
      ParticleActionType::PASourceID => Box::new(ParticleActionSource::import(section_name, ini)?),
      ParticleActionType::PASpeedLimitID => {
        Box::new(ParticleActionSpeedLimit::import(section_name, ini)?)
      }
      ParticleActionType::PATargetColorID => {
        Box::new(ParticleActionTargetColor::import(section_name, ini)?)
      }
      ParticleActionType::PATargetSizeID => {
        Box::new(ParticleActionTargetSize::import(section_name, ini)?)
      }
      ParticleActionType::PATargetRotateID | ParticleActionType::PATargetRotateDID => {
        Box::new(ParticleActionTargetRotate::import(section_name, ini)?)
      }
      ParticleActionType::PATargetVelocityID | ParticleActionType::PATargetVelocityDID => {
        Box::new(ParticleActionTargetVelocity::import(section_name, ini)?)
      }
      ParticleActionType::PAVortexID => Box::new(ParticleActionVortex::import(section_name, ini)?),
      ParticleActionType::PATurbulenceID => {
        Box::new(ParticleActionTurbulence::import(section_name, ini)?)
      }
      ParticleActionType::PAScatterID => {
        Box::new(ParticleActionScatter::import(section_name, ini)?)
      }
      ParticleActionType::Unknown | ParticleActionType::PACallActionListID => {
        return Err(DatabaseParseError::new_database_error(format!(
          "Not implemented parser for particle action importing: {:?}",
          particle_action_type
        )));
      }
    })
  }
}
