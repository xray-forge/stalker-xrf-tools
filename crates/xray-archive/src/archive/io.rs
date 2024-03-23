use crate::archive::reader::ArchiveReader;
use std::io::Read;

impl Read for ArchiveReader {
  fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
    self.file.read(buf)
  }
}
