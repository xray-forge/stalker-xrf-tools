use byteorder::ReadBytesExt;
use fileslice::FileSlice;

pub trait ArchiveDataSource: ReadBytesExt {}

impl ArchiveDataSource for FileSlice {}
