use byteorder::ByteOrder;
use fileslice::FileSlice;
use xrf_error::XrfResult;

use crate::{ChunkReader, ChunkWriter};

pub trait ChunkReadWriteList: Sized {
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XrfResult<Vec<Self>>;
  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, list: &[Self]) -> XrfResult;
}

pub trait ChunkReadWrite: Sized {
  fn read<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XrfResult<Self>;
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XrfResult;
}

pub trait ChunkReadWriteOptional: Sized {
  fn read_optional<T: ByteOrder>(reader: &mut ChunkReader<FileSlice>) -> XrfResult<Option<Self>>;
  fn write_optional<T: ByteOrder>(writer: &mut ChunkWriter, it: Option<&Self>) -> XrfResult;
}
