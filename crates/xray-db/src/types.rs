use crate::data::vector_3d::Vector3d;
use crate::error::database_error::DatabaseError;
use byteorder::LittleEndian;

pub type DatabaseResult<T = ()> = Result<T, DatabaseError>;

pub type Sphere3d<T = f32> = (Vector3d<T>, T);

pub type Matrix3d<T = f32> = (Vector3d<T>, Vector3d<T>, Vector3d<T>, Vector3d<T>);

pub type U32Bytes = (u8, u8, u8, u8);

pub type SpawnByteOrder = LittleEndian;

pub type ParticlesByteOrder = LittleEndian;
