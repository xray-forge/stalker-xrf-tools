use crate::{ChunkReader, ChunkWriter};
use byteorder::ByteOrder;
use fileslice::FileSlice;
use xray_error::XRayResult;

pub trait ChunkReadable: Sized {
  fn read<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XRayResult<Self>;
}

pub trait ChunkReadableOptional: Sized {
  fn read_optional<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XRayResult<Option<Self>>;
}

pub trait ChunkReadableList: Sized {
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XRayResult<Vec<Self>>;
}

pub trait ChunkWritable: Sized {
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult;
}

pub trait ChunkWritableOptional: Sized {
  fn write_optional<T: ByteOrder>(writer: &mut ChunkWriter, it: Option<&Self>) -> XRayResult;
}

pub trait ChunkWritableList: Sized {
  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, list: &[Self]) -> XRayResult;
}
