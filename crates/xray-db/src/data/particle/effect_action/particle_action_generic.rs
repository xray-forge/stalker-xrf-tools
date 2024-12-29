use std::fmt::Debug;

#[typetag::serde(tag = "type")]
pub trait ParticleActionGeneric: Debug + Send + Sync {}
