use crate::data::particles::particle_action_generic::ParticleActionGeneric;
use derive_more::Display;
use enum_map::Enum;

#[derive(Copy, Clone, Debug, Enum, PartialEq, Display)]
pub enum ParticleActionType {
  #[display("Avoid")]
  Avoid = 0,
  #[display("Bounce")]
  Bounce = 1,
  #[display("CallActionList")]
  CallActionList = 2,
  #[display("CopyVertex")]
  CopyVertex = 3,
  #[display("Damping")]
  Damping = 4,
  #[display("Explosion")]
  Explosion = 5,
  #[display("Follow")]
  Follow = 6,
  #[display("Gravitate")]
  Gravitate = 7,
  #[display("Gravity")]
  Gravity = 8,
  #[display("Jet")]
  Jet = 9,
  #[display("KillOld")]
  KillOld = 10,
  #[display("MatchVelocity")]
  MatchVelocity = 11,
  #[display("Move")]
  Move = 12,
  #[display("OrbitLine")]
  OrbitLine = 13,
  #[display("OrbitPoint")]
  OrbitPoint = 14,
  #[display("RandomAccel")]
  RandomAccel = 15,
  #[display("RandomDisplace")]
  RandomDisplace = 16,
  #[display("RandomVelocity")]
  RandomVelocity = 17,
  #[display("Restore")]
  Restore = 18,
  #[display("Sink")]
  Sink = 19,
  #[display("SinkVelocity")]
  SinkVelocity = 20,
  #[display("Source")]
  Source = 21,
  #[display("SpeedLimit")]
  SpeedLimit = 22,
  #[display("TargetColor")]
  TargetColor = 23,
  #[display("TargetSize")]
  TargetSize = 24,
  #[display("TargetRotate")]
  TargetRotate = 25,
  #[display("TargetRotateD")]
  TargetRotateD = 26,
  #[display("TargetVelocity")]
  TargetVelocity = 27,
  #[display("TargetVelocityD")]
  TargetVelocityD = 28,
  #[display("Vortex")]
  Vortex = 29,
  #[display("Turbulence")]
  Turbulence = 30,
  #[display("Scatter")]
  Scatter = 31,
  #[display("Unknown")]
  Unknown = -1,
}

impl ParticleActionType {
  pub fn get_action_type(action: &ParticleActionGeneric) -> Self {
    match action {
      ParticleActionGeneric::Avoid(_) => Self::Avoid,
      ParticleActionGeneric::Bounce(_) => Self::Bounce,
      ParticleActionGeneric::CopyVertex(_) => Self::CopyVertex,
      ParticleActionGeneric::Damping(_) => Self::Damping,
      ParticleActionGeneric::Explosion(_) => Self::Explosion,
      ParticleActionGeneric::Follow(_) => Self::Follow,
      ParticleActionGeneric::Gravitate(_) => Self::Gravitate,
      ParticleActionGeneric::Gravity(_) => Self::Gravity,
      ParticleActionGeneric::Jet(_) => Self::Jet,
      ParticleActionGeneric::KillOld(_) => Self::KillOld,
      ParticleActionGeneric::MatchVelocity(_) => Self::MatchVelocity,
      ParticleActionGeneric::Move(_) => Self::Move,
      ParticleActionGeneric::OrbitLine(_) => Self::OrbitLine,
      ParticleActionGeneric::OrbitPoint(_) => Self::OrbitPoint,
      ParticleActionGeneric::RandomAccel(_) => Self::RandomAccel,
      ParticleActionGeneric::RandomDisplace(_) => Self::RandomDisplace,
      ParticleActionGeneric::RandomVelocity(_) => Self::RandomVelocity,
      ParticleActionGeneric::Restore(_) => Self::Restore,
      ParticleActionGeneric::Sink(_) => Self::Sink,
      ParticleActionGeneric::SinkVelocity(_) => Self::SinkVelocity,
      ParticleActionGeneric::Source(_) => Self::Source,
      ParticleActionGeneric::SpeedLimit(_) => Self::SpeedLimit,
      ParticleActionGeneric::TargetColor(_) => Self::TargetColor,
      ParticleActionGeneric::TargetSize(_) => Self::TargetSize,
      ParticleActionGeneric::TargetRotate(_) => Self::TargetRotate,
      ParticleActionGeneric::TargetVelocity(_) => Self::TargetVelocity,
      ParticleActionGeneric::Vortex(_) => Self::Vortex,
      ParticleActionGeneric::Turbulence(_) => Self::Turbulence,
      ParticleActionGeneric::Scatter(_) => Self::Scatter,
    }
  }
}

impl<T> From<T> for ParticleActionType
where
  T: Into<u32>,
{
  fn from(action_type: T) -> Self {
    match action_type.into() {
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
}
