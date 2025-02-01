use crate::archive::archive_constants::{CHUNK_ID_COMPRESSED_MASK, CHUNK_ID_MASK};
use crate::archive::archive_descriptor::ArchiveDescriptor;
use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
use crate::archive::archive_header::ArchiveHeader;
use crate::types::XRayByteOrder;
use byteorder::ReadBytesExt;
use delharc::decode::{Decoder, Lh1Decoder};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::ErrorKind::UnexpectedEof;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use xray_error::{XRayError, XRayResult};
use xray_utils::{
  assert, decode_bytes_to_string_without_bom_handling, get_utf8_encoder, XRayEncoding,
};

pub struct ArchiveReader {
  pub path: PathBuf,
  pub file: File,
  pub section_regex: Regex,
  pub variable_regex: Regex,
  pub root_regex: Regex,
  pub encoding: XRayEncoding,
}

impl ArchiveReader {
  /// Create chunk based on whole file.
  pub fn from_path<P: AsRef<Path>>(path: P, encoding: XRayEncoding) -> XRayResult<Self> {
    match File::open(path.as_ref()) {
      Ok(file) => Ok(Self {
        encoding,
        file,
        path: path.as_ref().into(),
        root_regex: Regex::new(r"^\$\w+?\$\\").unwrap(),
        section_regex: Regex::new(r"^.*\[(?P<name>\w*)\]$").unwrap(),
        variable_regex: Regex::new(r"^\s*(?P<name>\w+)\s*=\s*(?P<value>.+)\s*$").unwrap(),
      }),
      Err(error) => Err(XRayError::new_read_error(format!(
        "Failed to read archive file {}, {}",
        path.as_ref().display(),
        error
      ))),
    }
  }

  /// Create chunk based on whole file.
  pub fn from_path_utf8<P: AsRef<Path>>(path: P) -> XRayResult<Self> {
    Self::from_path(path, get_utf8_encoder())
  }
}

impl ArchiveReader {
  pub fn read_archive(&mut self) -> XRayResult<ArchiveDescriptor> {
    let header: ArchiveHeader = self.read_archive_header()?.unwrap();

    Ok(ArchiveDescriptor {
      files: header.files,
      output_root_path: header.output_root_path,
      path: header.archive_path,
    })
  }
}

impl ArchiveReader {
  fn read_archive_header(&mut self) -> XRayResult<Option<ArchiveHeader>> {
    let mut file_descriptors = None;
    let mut root_path: String = String::new();

    loop {
      let raw_chunk_id: u32 = match self.file.read_u32::<XRayByteOrder>() {
        Ok(data) => data,
        Err(error) if error.kind() == UnexpectedEof => break,
        Err(error) => return Err(XRayError::new_read_error(error.to_string())),
      };
      let chunk_size: u32 = self.file.read_u32::<XRayByteOrder>()?;
      let chunk_usize: usize = usize::try_from(chunk_size).map_err(|error| {
        XRayError::new_read_error(format!(
          "Failed to read archive header chunk size: {}",
          error
        ))
      })?;

      let chunk_id: u32 = raw_chunk_id & CHUNK_ID_MASK;
      let compressed: bool = (raw_chunk_id & CHUNK_ID_COMPRESSED_MASK) != 0;

      match chunk_id {
        // File descriptors list
        0x1 | 0x86 => {
          let chunk_data: Vec<u8> = Self::read_chunk(&mut self.file, chunk_usize, compressed)?;
          let mut reader: Cursor<&[u8]> = Cursor::new(chunk_data.as_slice());

          file_descriptors = Some(
            Self::read_file_descriptors(&mut reader, self.encoding)
              .expect("Expecting a valid file descriptors chunk"),
          );
        }
        // Metadata header
        666 | 1337 => {
          let chunk_data: Vec<u8> = Self::read_chunk(&mut self.file, chunk_usize, compressed)?;

          root_path = self
            .read_root_path(chunk_data.as_slice())?
            .expect("[header].entry_point must be specified in header chunk when it exists");
        }
        _ => {
          // Skip
          self.file.seek(SeekFrom::Current(i64::from(chunk_size)))?;
        }
      }
    }

    Ok(file_descriptors.map(|file_descriptors| ArchiveHeader {
      archive_path: self.path.clone(),
      output_root_path: root_path.into(),
      files: file_descriptors,
    }))
  }

  // Just Result instead of optional?
  fn read_root_path(&self, chunk_data: &[u8]) -> XRayResult<Option<String>> {
    // let section_regex= Regex::new(r"^.*\[(?P<name>\w*)\]$").unwrap();
    // let variable_regex= Regex::new(r"^\s*(?P<name>\w+)\s*=\s*(?P<value>.+)\s*$").unwrap();
    // let root_regex = Regex::new(r"^\$\w+?\$\\").unwrap();

    let mut last_section_name: String = String::new();

    for line in decode_bytes_to_string_without_bom_handling(chunk_data, self.encoding)?.lines() {
      let section_captures = self.section_regex.captures(line);
      match (section_captures, last_section_name.as_str()) {
        (None, "header") => {
          let variable_captures = self.variable_regex.captures(line);

          if let Some(captures) = variable_captures {
            if &captures["name"] == "entry_point" {
              let entry_point = captures["value"].to_string();
              return Ok(Some(
                self
                  .root_regex
                  .replace(entry_point.as_str(), "")
                  .to_string(),
              ));
            }
          }
        }
        (Some(capture), _) => {
          last_section_name = capture["name"].to_string();
        }
        _ => {}
      }
    }

    Ok(None)
  }

  fn read_chunk<T: Read>(
    file: &mut T,
    chunk_usize: usize,
    compressed: bool,
  ) -> XRayResult<Vec<u8>> {
    match compressed {
      true => {
        let decoded_len: u32 = file.read_u32::<XRayByteOrder>()?;
        let mut compressed_buf: Vec<u8> = vec![0u8; chunk_usize - 4usize];

        file.read_exact(compressed_buf.as_mut_slice())?;

        let mut res: Lh1Decoder<&[u8]> = Lh1Decoder::new(compressed_buf.as_slice());
        let mut decompressed_buf: Vec<u8> = vec![0u8; decoded_len as usize];

        res.fill_buffer(&mut decompressed_buf)?;

        Ok(decompressed_buf)
      }
      false => {
        let mut raw_buf: Vec<u8> = vec![0u8; chunk_usize];

        file.read_exact(raw_buf.as_mut_slice())?;

        Ok(raw_buf)
      }
    }
  }

  fn read_file_descriptors<T: Read>(
    reader: &mut T,
    encoding: XRayEncoding,
  ) -> XRayResult<HashMap<String, ArchiveFileDescriptor>> {
    let mut file_descriptors: HashMap<String, ArchiveFileDescriptor> = HashMap::new();
    let mut name_buf: [u8; 520] = [0u8; 520];

    loop {
      let header_size: u16 = match reader.read_u16::<XRayByteOrder>() {
        Ok(data) => data,
        Err(error) if error.kind() == UnexpectedEof => break,
        Err(error) => return Err(error.into()),
      };

      let size_real: u32 = reader.read_u32::<XRayByteOrder>()?;
      let size_compressed: u32 = reader.read_u32::<XRayByteOrder>()?;
      let crc: u32 = reader.read_u32::<XRayByteOrder>()?;
      let name_size: u16 = header_size - 16;

      let name_bytes = {
        assert((name_size as usize) < name_buf.len(), "Name is too long")?;

        reader
          .read_exact(&mut name_buf[..(name_size as usize)])
          .expect("Unable to read file name from header");

        &name_buf[..(name_size as usize)]
      };

      let offset: u32 = reader.read_u32::<XRayByteOrder>()?;
      let name: String = decode_bytes_to_string_without_bom_handling(name_bytes, encoding)?;

      file_descriptors.insert(
        name.clone(),
        ArchiveFileDescriptor {
          name,
          offset,
          size_real,
          size_compressed,
          crc,
        },
      );
    }

    Ok(file_descriptors)
  }
}
