use byteorder::ReadBytesExt;
use fileslice::FileSlice;

pub trait ChunkDataSource: ReadBytesExt {}

impl ChunkDataSource for FileSlice {}
