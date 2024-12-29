use crate::chunk::reader::ChunkReader;
use crate::data::particle::effect_action::particle_action_avoid::ParticleActionAvoid;
use crate::data::particle::effect_action::particle_action_bounce::ParticleActionBounce;
use crate::data::particle::effect_action::particle_action_copy_vertex::ParticleActionCopyVertex;
use crate::data::particle::effect_action::particle_action_damping::ParticleActionDamping;
use crate::data::particle::effect_action::particle_action_explosion::ParticleActionExplosion;
use crate::data::particle::effect_action::particle_action_follow::ParticleActionFollow;
use crate::data::particle::effect_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::effect_action::particle_action_gravitate::ParticleActionGravitate;
use crate::data::particle::effect_action::particle_action_gravity::ParticleActionGravity;
use crate::data::particle::effect_action::particle_action_jet::ParticleActionJet;
use crate::data::particle::effect_action::particle_action_kill_old::ParticleActionKillOld;
use crate::data::particle::effect_action::particle_action_match_velocity::ParticleActionMatchVelocity;
use crate::data::particle::effect_action::particle_action_move::ParticleActionMove;
use crate::data::particle::effect_action::particle_action_orbit_line::ParticleActionOrbitLine;
use crate::data::particle::effect_action::particle_action_orbit_point::ParticleActionOrbitPoint;
use crate::data::particle::effect_action::particle_action_random_acceleration::ParticleActionRandomAcceleration;
use crate::data::particle::effect_action::particle_action_random_displace::ParticleActionRandomDisplace;
use crate::data::particle::effect_action::particle_action_random_velocity::ParticleActionRandomVelocity;
use crate::data::particle::effect_action::particle_action_restore::ParticleActionRestore;
use crate::data::particle::effect_action::particle_action_scatter::ParticleActionScatter;
use crate::data::particle::effect_action::particle_action_sink::ParticleActionSink;
use crate::data::particle::effect_action::particle_action_sink_velocity::ParticleActionSinkVelocity;
use crate::data::particle::effect_action::particle_action_source::ParticleActionSource;
use crate::data::particle::effect_action::particle_action_speed_limit::ParticleActionSpeedLimit;
use crate::data::particle::effect_action::particle_action_target_color::ParticleActionTargetColor;
use crate::data::particle::effect_action::particle_action_target_rotate::ParticleActionTargetRotate;
use crate::data::particle::effect_action::particle_action_target_size::ParticleActionTargetSize;
use crate::data::particle::effect_action::particle_action_target_velocity::ParticleActionTargetVelocity;
use crate::data::particle::effect_action::particle_action_turbulence::ParticleActionTurbulence;
use crate::data::particle::effect_action::particle_action_vortex::ParticleActionVortex;
use byteorder::ByteOrder;
use enum_map::Enum;
use std::io;

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
  ) -> io::Result<Box<dyn ParticleActionGeneric>> {
    match particle_action_type {
      ParticleActionType::PAAvoidID => Ok(Box::new(ParticleActionAvoid::read::<T>(reader)?)),
      ParticleActionType::PABounceID => Ok(Box::new(ParticleActionBounce::read::<T>(reader)?)),
      ParticleActionType::PACopyVertexBID => {
        Ok(Box::new(ParticleActionCopyVertex::read::<T>(reader)?))
      }
      ParticleActionType::PADampingID => Ok(Box::new(ParticleActionDamping::read::<T>(reader)?)),
      ParticleActionType::PAExplosionID => {
        Ok(Box::new(ParticleActionExplosion::read::<T>(reader)?))
      }
      ParticleActionType::PAFollowID => Ok(Box::new(ParticleActionFollow::read::<T>(reader)?)),
      ParticleActionType::PAGravitateID => {
        Ok(Box::new(ParticleActionGravitate::read::<T>(reader)?))
      }
      ParticleActionType::PAGravityID => Ok(Box::new(ParticleActionGravity::read::<T>(reader)?)),
      ParticleActionType::PAJetID => Ok(Box::new(ParticleActionJet::read::<T>(reader)?)),
      ParticleActionType::PAKillOldID => Ok(Box::new(ParticleActionKillOld::read::<T>(reader)?)),
      ParticleActionType::PAMatchVelocityID => {
        Ok(Box::new(ParticleActionMatchVelocity::read::<T>(reader)?))
      }
      ParticleActionType::PAMoveID => Ok(Box::new(ParticleActionMove::read::<T>(reader)?)),
      ParticleActionType::PAOrbitLineID => {
        Ok(Box::new(ParticleActionOrbitLine::read::<T>(reader)?))
      }
      ParticleActionType::PAOrbitPointID => {
        Ok(Box::new(ParticleActionOrbitPoint::read::<T>(reader)?))
      }
      ParticleActionType::PARandomAccelID => Ok(Box::new(
        ParticleActionRandomAcceleration::read::<T>(reader)?,
      )),
      ParticleActionType::PARandomDisplaceID => {
        Ok(Box::new(ParticleActionRandomDisplace::read::<T>(reader)?))
      }
      ParticleActionType::PARandomVelocityID => {
        Ok(Box::new(ParticleActionRandomVelocity::read::<T>(reader)?))
      }
      ParticleActionType::PARestoreID => Ok(Box::new(ParticleActionRestore::read::<T>(reader)?)),
      ParticleActionType::PASinkID => Ok(Box::new(ParticleActionSink::read::<T>(reader)?)),
      ParticleActionType::PASinkVelocityID => {
        Ok(Box::new(ParticleActionSinkVelocity::read::<T>(reader)?))
      }
      ParticleActionType::PASourceID => Ok(Box::new(ParticleActionSource::read::<T>(reader)?)),
      ParticleActionType::PASpeedLimitID => {
        Ok(Box::new(ParticleActionSpeedLimit::read::<T>(reader)?))
      }
      ParticleActionType::PATargetColorID => {
        Ok(Box::new(ParticleActionTargetColor::read::<T>(reader)?))
      }
      ParticleActionType::PATargetSizeID => {
        Ok(Box::new(ParticleActionTargetSize::read::<T>(reader)?))
      }
      ParticleActionType::PATargetRotateID | ParticleActionType::PATargetRotateDID => {
        Ok(Box::new(ParticleActionTargetRotate::read::<T>(reader)?))
      }
      ParticleActionType::PATargetVelocityID | ParticleActionType::PATargetVelocityDID => {
        Ok(Box::new(ParticleActionTargetVelocity::read::<T>(reader)?))
      }
      ParticleActionType::PAVortexID => Ok(Box::new(ParticleActionVortex::read::<T>(reader)?)),
      ParticleActionType::PATurbulenceID => {
        Ok(Box::new(ParticleActionTurbulence::read::<T>(reader)?))
      }
      ParticleActionType::PAScatterID => Ok(Box::new(ParticleActionScatter::read::<T>(reader)?)),
      ParticleActionType::Unknown | ParticleActionType::PACallActionListID => panic!(
        "Not implemented parser for effect_action type: {:?}",
        particle_action_type
      ),
    }
  }
}
