use crate::data::ogf::ogf_geometry::OgfGeometry;
use crate::ogf::chunks::ogf_bones_chunk::OgfBonesChunk;
use crate::ogf::chunks::ogf_children_chunk::OgfChildrenChunk;
use crate::ogf::chunks::ogf_description_chunk::OgfDescriptionChunk;
use crate::ogf::chunks::ogf_header_chunk::OgfHeaderChunk;
use crate::ogf::chunks::ogf_ik_data_chunk::OgfIkDataChunk;
use crate::ogf::chunks::ogf_kinematics_chunk::OgfKinematicsChunk;
use crate::ogf::chunks::ogf_lods_chunk::OgfLodsChunk;
use crate::ogf::chunks::ogf_texture_chunk::OgfTextureChunk;
use crate::ogf::chunks::ogf_user_data_chunk::OgfUserDataChunk;
use crate::omf::chunks::omf_motions_chunk::OmfMotionsChunk;
use crate::omf::chunks::omf_parameters_chunk::OmfParametersChunk;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use xray_chunk::{
  ChunkReader, find_one_of_optional_chunk_by_id, find_one_of_required_chunks_by_id,
  find_optional_chunk_by_id, find_required_chunk_by_id,
};
use xray_error::{XRayError, XRayResult};

/// FMesh in c++ codebase.
///
/// Reads only. OGF payloads cannot be fully re-serialized, so the parsed form is a view for
/// inspection and verification rather than a document: geometry keeps only its bone indices and
/// unknown chunks are skipped entirely. Editing an ogf file therefore never goes through this type,
/// see [`crate::OgfMotionRefsProcessor`] and [`crate::OgfTextureRefsProcessor`], which patch raw
/// chunks and copy everything they do not change byte for byte.
#[derive(Debug, Serialize, Deserialize)]
pub struct OgfFile {
  pub header: OgfHeaderChunk,
  pub texture: Option<OgfTextureChunk>,
  pub geometry: Option<OgfGeometry>,
  pub bones: Option<OgfBonesChunk>,
  pub children: Option<OgfChildrenChunk>,
  pub description: Option<OgfDescriptionChunk>,
  pub kinematics: Option<OgfKinematicsChunk>,
  pub ik_data: Option<OgfIkDataChunk>,
  pub user_data: Option<OgfUserDataChunk>,
  pub lods: Option<OgfLodsChunk>,
  /// Motions stored inside the visual itself rather than referenced from an omf file.
  ///
  /// Self-animated models embed the same two chunks an omf carries, under the same ids, so the omf
  /// types are reused verbatim.
  pub motions: Option<OmfMotionsChunk>,
  pub motion_parameters: Option<OmfParametersChunk>,
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
    // Bones are read up front because the ik data chunk stores no count of its own and has one record
    // per bone, so it can only be read once the bone list is known.
    let bones: Option<OgfBonesChunk> =
      match find_optional_chunk_by_id(chunks, OgfBonesChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      };

    let ik_data: Option<OgfIkDataChunk> = match (
      &bones,
      find_optional_chunk_by_id(chunks, OgfIkDataChunk::CHUNK_ID),
    ) {
      (Some(bones), Some(mut it)) => Some(OgfIkDataChunk::read::<T>(&mut it, bones.bones.len())?),
      _ => None,
    };

    Ok(Self {
      bones,
      ik_data,
      header: find_required_chunk_by_id(chunks, OgfHeaderChunk::CHUNK_ID)?.read_xr::<T, _>()?,
      texture: match find_optional_chunk_by_id(chunks, OgfTextureChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      geometry: OgfGeometry::read_from_chunks::<T, _>(chunks)?,
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
      user_data: match find_optional_chunk_by_id(chunks, OgfUserDataChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      lods: match find_optional_chunk_by_id(chunks, OgfLodsChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      motions: match find_optional_chunk_by_id(chunks, OmfMotionsChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
        None => None,
      },
      motion_parameters: match find_optional_chunk_by_id(chunks, OmfParametersChunk::CHUNK_ID) {
        Some(mut it) => Some(it.read_xr::<T, _>()?),
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

  /// Collect the texture reference of every nested child visual.
  ///
  /// Texture chunks of a skeleton live inside the children container rather than at the top level,
  /// so the top level `texture` field is empty for the models that have any.
  pub fn read_texture_refs_from_path<T: ByteOrder, P: AsRef<Path>>(
    path: &P,
  ) -> XRayResult<Vec<String>> {
    Ok(
      Self::read_from_path::<T, _>(path)?
        .children
        .map(|children| {
          children
            .nested
            .iter()
            .filter_map(|it| it.texture.as_ref().map(|it| it.texture_name.clone()))
            .collect()
        })
        .unwrap_or_default(),
    )
  }
}
