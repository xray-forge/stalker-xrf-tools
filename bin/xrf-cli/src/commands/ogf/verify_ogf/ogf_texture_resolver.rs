//! Resolves and inspects textures referenced by OGF visuals.
//!
//! Each reference is resolved against the visual's implied X-Ray root. DDS header results, including failures, are
//! cached by resolved texture path.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use xrf_dds::{DdsFile, DdsFormat, DdsMetadata};
use xrf_vfs::{XrayLookupScope, XrayVfs, implied_asset_root};

/// The outcome of resolving and reading one texture reference.
pub enum TextureResolution {
  /// The visual sits under no directory that looks like an X-Ray root.
  NoRoot,
  /// The implied root could not be mounted or produced no readable physical texture path.
  Missing { root: PathBuf },
  /// The texture resolved and its DDS header was read.
  Resolved {
    path: PathBuf,
    format: String,
    metadata: DdsMetadata,
  },
  /// The texture resolved, but its DDS header could not be read or parsed.
  Unreadable { path: PathBuf, reason: String },
}

/// Caches mounted roots and DDS header results across a verification sweep.
///
/// Repeated references reuse the success or failure cached for their resolved texture path.
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

    let Ok(mount) = self
      .vfs
      .mount_directory("", &root)
      .inspect_err(|error| log::warn!("Failed to mount root {}: {error}", root.display()))
    else {
      return TextureResolution::Missing { root };
    };

    // Search only the visual's tree; a fallback mount would hide gaps that verification must report.
    //
    // Do not widen this scope to installation archives: archive reads load complete entries, while verification needs
    // only DDS metadata. Add header-only archive reads before including them.
    let scope: XrayLookupScope = XrayLookupScope::only([mount]);

    let located: Option<PathBuf> = self
      .vfs
      .dds_texture(&scope, reference)
      .ok()
      .flatten()
      .and_then(|location| location.physical_path());

    let Some(path) = located else {
      return TextureResolution::Missing { root };
    };

    match self.header(&path) {
      Ok((format, metadata)) => TextureResolution::Resolved { path, format, metadata },
      Err(reason) => TextureResolution::Unreadable { path, reason },
    }
  }

  /// Returns cached DDS header facts by value.
  ///
  /// The owned result does not hold a borrow across subsequent resolver calls.
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

  /// Returns the number of distinct texture files inspected by the sweep.
  pub fn distinct_textures(&self) -> usize {
    self.headers.len()
  }
}

/// Formats a DDS format name while preserving an unknown format's FourCC.
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
