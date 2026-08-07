use crate::ogf::chunks::ogf_bones_chunk::OgfBonesChunk;
use crate::ogf::chunks::ogf_children_chunk::OgfChildrenChunk;
use crate::ogf::chunks::ogf_description_chunk::OgfDescriptionChunk;
use crate::ogf::chunks::ogf_geometry::OgfGeometry;
use crate::ogf::chunks::ogf_header_chunk::OgfHeaderChunk;
use crate::ogf::chunks::ogf_kinematics_chunk::OgfKinematicsChunk;
use crate::ogf::chunks::ogf_texture_chunk::OgfTextureChunk;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xray_chunk::{
  ChunkReader, ChunkWriter, find_one_of_optional_chunk_by_id, find_one_of_required_chunks_by_id,
  find_optional_chunk_by_id, find_required_chunk_by_id,
};
use xray_error::{XRayError, XRayResult};
use xray_utils::open_export_file;

/// FMesh in c++ codebase.
#[derive(Debug, Serialize, Deserialize)]
pub struct OgfFile {
  pub header: OgfHeaderChunk,
  pub texture: Option<OgfTextureChunk>,
  pub geometry: Option<OgfGeometry>,
  pub bones: Option<OgfBonesChunk>,
  pub children: Option<OgfChildrenChunk>,
  pub description: Option<OgfDescriptionChunk>,
  pub kinematics: Option<OgfKinematicsChunk>,
}

impl OgfFile {
  pub fn read_from_path<T: ByteOrder, P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    Self::read_from_file::<T>(File::open(path).map_err(|error| {
      XRayError::new_not_found_error(format!(
        "OGF file was not read: {}, error: {}",
        path.as_ref().display(),
        error
      ))
    })?)
  }

  pub fn read_from_file<T: ByteOrder>(file: File) -> XRayResult<Self> {
    Self::read_from_chunk::<T>(&mut ChunkReader::from_file(file)?)
  }

  pub fn read_from_chunk<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let chunks: Vec<ChunkReader> = reader.read_children()?;

    Self::read_from_chunks::<T>(&chunks)
  }

  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> XRayResult<Self> {
    Ok(Self {
      header: find_required_chunk_by_id(chunks, OgfHeaderChunk::CHUNK_ID)?.read_xr::<T, _>()?,
      texture: match find_optional_chunk_by_id(chunks, OgfTextureChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      geometry: OgfGeometry::read_from_chunks::<T, _>(chunks)?,
      bones: match find_optional_chunk_by_id(chunks, OgfBonesChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      children: match find_optional_chunk_by_id(chunks, OgfChildrenChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      description: match find_optional_chunk_by_id(chunks, OgfDescriptionChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      kinematics: match find_one_of_optional_chunk_by_id(
        chunks,
        &[
          OgfKinematicsChunk::CHUNK_ID,
          OgfKinematicsChunk::CHUNK_ID_OLD,
        ],
      ) {
        Some((id, mut it)) => Some(OgfKinematicsChunk::read::<T>(&mut it, id)?),
        None => None,
      },
    })
  }

  /// Read only list of motion refs specifically and skip other data parts.
  pub fn read_motion_refs_from_path<T: ByteOrder, P: AsRef<Path>>(
    path: &P,
  ) -> XRayResult<Vec<String>> {
    Self::read_motion_refs_from_file::<T>(File::open(path)?)
  }

  /// Read only list of motion refs specifically and skip other data parts.
  pub fn read_motion_refs_from_file<T: ByteOrder>(file: File) -> XRayResult<Vec<String>> {
    let mut reader: ChunkReader = ChunkReader::from_file(file)?;
    let chunks: Vec<ChunkReader> = reader.read_children()?;

    log::info!(
      "Reading ogf file motion refs, {} chunks, {} bytes",
      chunks.len(),
      reader.read_bytes_len(),
    );

    let (chunk_id, mut chunk) = find_one_of_required_chunks_by_id(
      &chunks,
      &[
        OgfKinematicsChunk::CHUNK_ID,
        OgfKinematicsChunk::CHUNK_ID_OLD,
      ],
    )?;

    Ok(OgfKinematicsChunk::read::<T>(&mut chunk, chunk_id)?.motion_refs)
  }
}

impl OgfFile {
  /// Rewrite motion refs of an ogf file and store the result in destination path.
  pub fn write_motion_refs_to_path<T: ByteOrder>(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    motion_refs: &[String],
  ) -> XRayResult {
    let buffer: Vec<u8> = Self::write_motion_refs_to_buffer::<T>(
      File::open(source.as_ref()).map_err(|error| {
        XRayError::new_not_found_error(format!(
          "OGF file was not read: {}, error: {}",
          source.as_ref().display(),
          error
        ))
      })?,
      motion_refs,
    )?;

    if let Some(parent) = destination.as_ref().parent() {
      fs::create_dir_all(parent)?;
    }

    open_export_file(destination.as_ref())?.write_all(&buffer)?;

    Ok(())
  }

  /// Rewrite motion refs of an ogf file, copying every other chunk verbatim.
  ///
  /// OGF payloads cannot be fully re-serialized yet, so this deliberately works at chunk level
  /// instead of going through [`OgfFile`]: every chunk is copied as raw bytes and only the motion
  /// refs chunk payload is rebuilt. Geometry, bones and ik data are therefore preserved exactly,
  /// and writing the refs a file already has reproduces that file byte for byte.
  ///
  /// The chunk id of the source file is preserved, so a file using the older single string chunk
  /// keeps using it rather than being silently upgraded.
  pub fn write_motion_refs_to_buffer<T: ByteOrder>(
    file: File,
    motion_refs: &[String],
  ) -> XRayResult<Vec<u8>> {
    let mut chunks: Vec<ChunkReader> = ChunkReader::from_file(file)?.read_children()?;
    let mut buffer: Vec<u8> = Vec::new();
    let mut patched_count: u32 = 0;

    for chunk in &mut chunks {
      let payload: Vec<u8> = if chunk.id == OgfKinematicsChunk::CHUNK_ID
        || chunk.id == OgfKinematicsChunk::CHUNK_ID_OLD
      {
        patched_count += 1;

        let mut kinematics_writer: ChunkWriter = ChunkWriter::new();

        OgfKinematicsChunk {
          source_chunk_id: chunk.id,
          motion_refs: motion_refs.to_vec(),
        }
        .write::<T>(&mut kinematics_writer)?;

        kinematics_writer.flush_raw_into_buffer()?
      } else {
        chunk.reset_pos()?;
        chunk.read_remaining()?
      };

      let mut chunk_writer: ChunkWriter = ChunkWriter::new();

      chunk_writer.write_all(&payload)?;
      chunk_writer.flush_chunk_into::<T>(&mut buffer, chunk.id)?;
    }

    if patched_count != 1 {
      return Err(XRayError::new_invalid_error(format!(
        "Expected exactly one ogf motion refs chunk to rewrite, got {patched_count}"
      )));
    }

    Ok(buffer)
  }
}

#[cfg(test)]
mod tests {
  use crate::ogf::chunks::ogf_kinematics_chunk::OgfKinematicsChunk;
  use crate::ogf::ogf_file::OgfFile;
  use std::fs;
  use std::fs::File;
  use std::io::Write;
  use std::path::PathBuf;
  use xray_chunk::{ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    overwrite_test_relative_resource_as_file,
  };

  /// Payload standing in for a chunk the writer must copy verbatim, such as geometry.
  const OPAQUE_PAYLOAD: [u8; 12] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 255, 128];

  fn write_sample(
    filename: &str,
    refs_chunk_id: u32,
    motion_refs: &[String],
  ) -> XRayResult<PathBuf> {
    let mut opaque_writer: ChunkWriter = ChunkWriter::new();
    opaque_writer.write_all(&OPAQUE_PAYLOAD)?;

    let mut refs_writer: ChunkWriter = ChunkWriter::new();
    OgfKinematicsChunk {
      source_chunk_id: refs_chunk_id,
      motion_refs: motion_refs.to_vec(),
    }
    .write::<XRayByteOrder>(&mut refs_writer)?;

    let mut file: File = overwrite_test_relative_resource_as_file(filename)?;

    opaque_writer.flush_chunk_into::<XRayByteOrder>(&mut file, 9)?;
    refs_writer.flush_chunk_into::<XRayByteOrder>(&mut file, refs_chunk_id)?;

    Ok(get_absolute_test_resource_path(filename))
  }

  #[test]
  fn test_write_motion_refs_reproduces_source_when_unchanged() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "unchanged.ogf");
    let refs: Vec<String> = vec![String::from("dynamics\\weapons\\wpn_ak74\\anim")];
    let path: PathBuf = write_sample(&filename, OgfKinematicsChunk::CHUNK_ID, &refs)?;

    let rewritten: Vec<u8> =
      OgfFile::write_motion_refs_to_buffer::<XRayByteOrder>(File::open(&path)?, &refs)?;

    assert_eq!(
      rewritten,
      fs::read(&path)?,
      "Expect rewriting existing refs to reproduce source bytes"
    );

    Ok(())
  }

  #[test]
  fn test_write_motion_refs_preserves_other_chunks() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "preserves_chunks.ogf");
    let path: PathBuf = write_sample(
      &filename,
      OgfKinematicsChunk::CHUNK_ID,
      &[String::from("old\\ref")],
    )?;

    let patched: Vec<u8> = OgfFile::write_motion_refs_to_buffer::<XRayByteOrder>(
      File::open(&path)?,
      &[
        String::from("new\\much\\longer\\ref"),
        String::from("second"),
      ],
    )?;

    // Opaque chunk header and payload must survive untouched at the head of the file.
    let original: Vec<u8> = fs::read(&path)?;
    let opaque_len: usize = 8 + OPAQUE_PAYLOAD.len();

    assert_eq!(
      patched[..opaque_len],
      original[..opaque_len],
      "Expect leading opaque chunk to be copied verbatim"
    );

    fs::write(&path, &patched)?;

    assert_eq!(
      OgfFile::read_motion_refs_from_path::<XRayByteOrder, _>(&path)?,
      vec![
        String::from("new\\much\\longer\\ref"),
        String::from("second")
      ],
      "Expect patched file to read back new refs"
    );

    Ok(())
  }

  #[test]
  fn test_write_motion_refs_preserves_old_chunk_id() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "old_chunk_id.ogf");
    let path: PathBuf = write_sample(
      &filename,
      OgfKinematicsChunk::CHUNK_ID_OLD,
      &[String::from("old\\ref")],
    )?;

    let patched: Vec<u8> = OgfFile::write_motion_refs_to_buffer::<XRayByteOrder>(
      File::open(&path)?,
      &[String::from("replacement")],
    )?;

    fs::write(&path, &patched)?;

    let chunk_id: u32 = u32::from_le_bytes(
      patched[patched.len() - 8 - "replacement".len() - 1..][..4]
        .try_into()
        .expect("Chunk id bytes"),
    );

    assert_eq!(
      chunk_id,
      OgfKinematicsChunk::CHUNK_ID_OLD,
      "Expect source chunk id form to be preserved"
    );

    assert_eq!(
      OgfFile::read_motion_refs_from_path::<XRayByteOrder, _>(&path)?,
      vec![String::from("replacement")]
    );

    Ok(())
  }

  #[test]
  fn test_write_motion_refs_requires_refs_chunk() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "without_refs.ogf");
    let mut opaque_writer: ChunkWriter = ChunkWriter::new();

    opaque_writer.write_all(&OPAQUE_PAYLOAD)?;
    opaque_writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      9,
    )?;

    let path: PathBuf = get_absolute_test_resource_path(&filename);

    assert!(
      OgfFile::write_motion_refs_to_buffer::<XRayByteOrder>(
        File::open(&path)?,
        &[String::from("any")]
      )
      .is_err(),
      "Expect rewrite to be refused when file has no motion refs chunk"
    );

    Ok(())
  }
}
