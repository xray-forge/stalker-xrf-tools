use crate::{ChunkReader, ChunkWriter};
use byteorder::ByteOrder;
use fileslice::FileSlice;
use xray_error::XRayResult;

pub trait ChunkReadWriteList: Sized {
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XRayResult<Vec<Self>>;
  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, list: &[Self]) -> XRayResult;
}

pub trait ChunkReadWrite: Sized {
  fn read<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XRayResult<Self>;
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult;
}

pub trait ChunkReadWriteOptional: Sized {
  fn read_optional<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XRayResult<Option<Self>>;
  fn write_optional<T: ByteOrder>(writer: &mut ChunkWriter, it: Option<&Self>) -> XRayResult;
}
