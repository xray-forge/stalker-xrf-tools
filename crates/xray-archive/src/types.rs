use crate::ArchiveError;
use byteorder::LittleEndian;

pub type ArchiveResult<T = ()> = Result<T, ArchiveError>;

pub type XRayByteOrder = LittleEndian;
