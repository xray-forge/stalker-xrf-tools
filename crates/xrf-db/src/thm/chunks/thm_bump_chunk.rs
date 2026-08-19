use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xrf_chunk::{ChunkDataSource, ChunkReadWrite, ChunkReader, ChunkWriter};
use xrf_error::XrfResult;

/// Bump declaration of a texture, `THM_CHUNK_BUMP` in the engine (`ETextureParams.h:190`).
///
/// The engine reads this at load time in `CTextureDescrMngr::LoadTHM` and takes [`Self::name`]
/// verbatim as the bump texture path. There is no `_bump` naming convention behind it: a texture
/// renamed on import keeps pointing wherever its thm says, so the bump fails to resolve.
///
/// A name that resolves to nothing does not disable bump mapping. `bump_exist` only tests that the
/// name is non-empty, so the renderer still picks the `_bump` shader variant and the loader
/// substitutes `ed\ed_dummy_bump`, logging `! Fallback to default bump map` once per load. The
/// surface ends up flat while paying for the bump path.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThmBumpChunk {
  pub virtual_height: f32,
  pub mode: u32,
  /// Bump texture path without extension, engine-style with backslashes. Empty when unused.
  pub name: String,
}

impl ThmBumpChunk {
  pub const CHUNK_ID: u32 = 0x0817;

  /// `STextureParams::ETBumpMode`, `ETextureParams.h:37`.
  pub const MODE_RESERVED: u32 = 0;
  pub const MODE_NONE: u32 = 1;
  pub const MODE_USE: u32 = 2;
  pub const MODE_USE_PARALLAX: u32 = 3;

  /// Whether the engine will try to resolve [`Self::name`] as a bump texture.
  pub fn is_used(&self) -> bool {
    matches!(self.mode, Self::MODE_USE | Self::MODE_USE_PARALLAX) && !self.name.is_empty()
  }
}

impl ChunkReadWrite for ThmBumpChunk {
  fn read<T: ByteOrder, D: ChunkDataSource>(reader: &mut ChunkReader<D>) -> XrfResult<Self> {
    let bump: Self = Self {
      virtual_height: reader.read_f32::<T>()?,
      mode: reader.read_u32::<T>()?,
      name: reader.read_w1251_string()?,
    };

    reader.assert_read("Expect all data to be read from thm bump chunk")?;

    Ok(bump)
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XrfResult {
    writer.write_f32::<T>(self.virtual_height)?;
    writer.write_u32::<T>(self.mode)?;
    writer.write_w1251_string(&self.name)?;

    Ok(())
  }
}
