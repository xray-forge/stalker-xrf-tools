use crate::ArchiveError;
use byteorder::LittleEndian;

pub type ArchiveByteOrder = LittleEndian;

pub type ArchiveResult<T = ()> = Result<T, ArchiveError>;
