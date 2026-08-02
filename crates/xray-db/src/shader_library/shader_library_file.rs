use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use xray_chunk::{ChunkDataSource, ChunkReader, find_required_chunk_by_id};
use xray_error::{XRayError, XRayResult};
use xray_utils::encode_w1251_bytes_to_string;

/// Names of compiled blender definitions stored in `shaders.xr`.
///
/// The renderer resolves an OGF texture chunk's shader name against these
/// definitions when it creates the visual.
#[derive(Debug, Default)]
pub struct ShaderLibraryFile {
  blender_names: HashSet<String>,
}

impl ShaderLibraryFile {
  pub const BLENDERS_CHUNK_ID: u32 = 2;

  const BLENDER_CLASS_ID_SIZE: usize = 8;
  const BLENDER_NAME_SIZE: usize = 128;

  pub fn read_from_path<P: AsRef<Path>>(path: P) -> XRayResult<Self> {
    Self::read_from_file(File::open(path.as_ref()).map_err(|error| {
      XRayError::new_not_found_error(format!(
        "Shader library was not read: {}, error: {error}",
        path.as_ref().display()
      ))
    })?)
  }

  pub fn read_from_file(file: File) -> XRayResult<Self> {
    Self::read_from_chunk(&mut ChunkReader::from_file(file)?)
  }

  pub fn read_from_chunk(reader: &mut ChunkReader) -> XRayResult<Self> {
    let chunks: Vec<ChunkReader> = reader.read_children()?;
    let mut blenders: ChunkReader = find_required_chunk_by_id(&chunks, Self::BLENDERS_CHUNK_ID)?;

    Self::read_blender_names(&mut blenders)
  }

  fn read_blender_names<T: ChunkDataSource>(blenders: &mut ChunkReader<T>) -> XRayResult<Self> {
    let blender_chunks = blenders.read_children()?;
    let mut blender_names: HashSet<String> = HashSet::with_capacity(blender_chunks.len());

    for mut blender in blender_chunks {
      blender.read_bytes(Self::BLENDER_CLASS_ID_SIZE)?;
      let name_bytes: Vec<u8> = blender.read_bytes(Self::BLENDER_NAME_SIZE)?;
      let Some(name_end) = name_bytes.iter().position(|byte| *byte == 0) else {
        return Err(XRayError::new_no_terminator_error(
          "Blender name in shader library is not null terminated",
        ));
      };

      let name: String = encode_w1251_bytes_to_string(&name_bytes[..name_end])?;

      if !blender_names.insert(name.clone()) {
        return Err(XRayError::new_invalid_error(format!(
          "Shader library contains duplicate blender '{name}'"
        )));
      }
    }

    Ok(Self { blender_names })
  }

  pub fn contains_blender(&self, name: &str) -> bool {
    self.blender_names.contains(name)
  }
}

#[cfg(test)]
mod tests {
  use super::ShaderLibraryFile;
  use std::io::Write;
  use xray_chunk::{ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::FileSlice;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read.chunk");
    let contents: Vec<u8> = shader_library_contents(&["models\\model", "models\\model_pn_hm"])?;
    let mut file = overwrite_test_relative_resource_as_file(&filename)?;
    file.write_all(&contents)?;
    file.flush()?;

    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    assert_eq!(file.bytes_remaining(), contents.len());

    let library: ShaderLibraryFile =
      ShaderLibraryFile::read_from_path(get_absolute_test_sample_file_path(file!(), "read.chunk"))?;

    assert!(library.contains_blender("models\\model"));
    assert!(library.contains_blender("models\\model_pn_hm"));
    assert!(!library.contains_blender("models\\missing"));

    Ok(())
  }

  fn shader_library_contents(blender_names: &[&str]) -> XRayResult<Vec<u8>> {
    let mut blenders: ChunkWriter = ChunkWriter::new();

    for (index, name) in blender_names.iter().enumerate() {
      let mut blender: ChunkWriter = ChunkWriter::new();
      blender.write_all(&[0; ShaderLibraryFile::BLENDER_CLASS_ID_SIZE])?;

      let mut name_buffer: [u8; ShaderLibraryFile::BLENDER_NAME_SIZE] =
        [0; ShaderLibraryFile::BLENDER_NAME_SIZE];
      name_buffer[..name.len()].copy_from_slice(name.as_bytes());
      blender.write_all(&name_buffer)?;
      blender.write_all(&[0; 40])?;

      blenders.write_all(&blender.flush_chunk_into_buffer::<XRayByteOrder>(index as u32)?)?;
    }

    let mut library: ChunkWriter = ChunkWriter::new();
    library.write_all(
      &blenders.flush_chunk_into_buffer::<XRayByteOrder>(ShaderLibraryFile::BLENDERS_CHUNK_ID)?,
    )?;

    library.flush_raw_into_buffer()
  }
}
