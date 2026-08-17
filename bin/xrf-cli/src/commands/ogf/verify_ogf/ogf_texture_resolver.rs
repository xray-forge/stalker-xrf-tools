//! Resolves a visual's texture references the way the engine would, and reports what the files are.
//!
//! Answers the question the texture phase rests on: for every submesh reference across the reference trees, does it
//! resolve against the root its visual implies, and to a format a renderer can upload.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use xrf_assets::{XrayVfs, implied_asset_root};
use xrf_dds::{DdsFile, DdsFormat, DdsMetadata};

/// What resolving one reference produced.
pub enum TextureResolution {
  /// The visual sits under no directory that looks like an X-Ray root.
  NoRoot,
  /// The root exists but holds no such texture, which is what the engine answers with its dummy.
  Missing { root: PathBuf },
  /// Resolved, and its header read.
  Resolved {
    path: PathBuf,
    format: String,
    metadata: DdsMetadata,
  },
  /// Resolved, but the header would not parse.
  Unreadable { path: PathBuf, reason: String },
}

/// Caches every expensive step, because a sweep asks the same questions repeatedly.
///
/// Roots are mounted by the VFS, and headers are read once per texture file here, since a reference repeats across
/// submeshes and models while a header is only interesting to verification.
///
/// Verification deliberately searches the visual's own tree alone rather than a chain: the question it answers is whether
/// a tree is internally complete, which a fallback root would mask.
#[derive(Default)]
pub struct OgfTextureResolver {
  vfs: XrayVfs,
  headers: HashMap<PathBuf, Result<(String, DdsMetadata), String>>,
}

impl OgfTextureResolver {
  pub fn resolve(&mut self, visual: &Path, reference: &str) -> TextureResolution {
    let Some(root) = implied_asset_root(visual) else {
      return TextureResolution::NoRoot;
    };

    let order: [PathBuf; 1] = [root.clone()];

    let Some(path) = self.vfs.dds_texture(&order, reference).map(|it| it.absolute_path()) else {
      return TextureResolution::Missing { root };
    };

    match self.header(&path) {
      Ok((format, metadata)) => TextureResolution::Resolved { path, format, metadata },
      Err(reason) => TextureResolution::Unreadable { path, reason },
    }
  }

  /// Header facts for a texture file, read once and handed over by value.
  ///
  /// Cloned rather than borrowed because the caller keeps resolving against the same cache, and a borrow would hold it
  /// for the rest of the sweep. A header is a few dozen bytes of scalars.
  fn header(&mut self, path: &Path) -> Result<(String, DdsMetadata), String> {
    self
      .headers
      .entry(path.to_path_buf())
      .or_insert_with(|| {
        DdsFile::read_metadata_from_path(path)
          .map(|metadata| (format_label(&metadata), metadata))
          .map_err(|error| error.to_string())
      })
      .clone()
  }

  /// Distinct texture files a sweep touched, which is the population the format counts describe.
  pub fn distinct_textures(&self) -> usize {
    self.headers.len()
  }
}

/// Format identity as a label, keeping an unknown one's fourCC visible rather than collapsing every unknown into one.
fn format_label(metadata: &DdsMetadata) -> String {
  match metadata.format {
    DdsFormat::D3d(format) => format!("{format:?}"),
    DdsFormat::Dxgi(format) => format!("{format:?}"),
    // `DdsFormat` is non-exhaustive, so an unrecognised variant keeps its fourCC visible rather than being dropped.
    _ => match metadata.four_cc {
      Some(four_cc) => format!("unknown({four_cc:#010x})"),
      None => String::from("unknown(no fourcc)"),
    },
  }
}
