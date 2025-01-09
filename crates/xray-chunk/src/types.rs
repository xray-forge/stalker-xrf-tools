use crate::error::chunk_error::ChunkError;
use byteorder::LittleEndian;

pub type ChunkResult<T = ()> = Result<T, ChunkError>;

pub type XRayByteOrder = LittleEndian;

pub type U32Bytes = (u8, u8, u8, u8);
