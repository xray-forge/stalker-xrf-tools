use std::path::{Path, PathBuf};

use xrf_error::XrfResult;
use xrf_utils::{decode_bytes_to_string, get_windows1251_encoder};
use xrf_vfs::{XrayScope, XrayVfs, xray_path};

use crate::Ltx;
use crate::file::include::LtxIncludeConvertor;
use crate::file::include_source::LtxIncludeSource;

/// Resolves and reads includes through a mounted VFS.
///
/// This is what lets configs be read out of an installation, where they live inside `db\configs` volumes. A wildcard include
/// cannot be answered by `read_dir` there, so it becomes prefix enumeration over the VFS instead - the same operation, asked
/// of a logical tree rather than a directory.
pub(crate) struct LtxIncludeVfsSource<'a> {
  vfs: &'a XrayVfs,
  scope: &'a XrayScope,
}

impl<'a> LtxIncludeVfsSource<'a> {
  pub fn new(vfs: &'a XrayVfs, scope: &'a XrayScope) -> Self {
    Self { scope, vfs }
  }

  /// Reads and parses one logical path, with its logical location recorded so nested includes resolve against it.
  pub fn read_ltx(&self, logical_path: &str) -> XrfResult<Ltx> {
    let bytes: Vec<u8> = self.vfs.read(self.scope, logical_path)?;
    let contents: String = decode_bytes_to_string(&bytes, get_windows1251_encoder())?;
    let mut ltx: Ltx = Ltx::read_from_str(&contents)?;
    let path: PathBuf = PathBuf::from(logical_path);

    ltx.directory = path.parent().map(Path::to_path_buf);
    ltx.path = Some(path);

    Ok(ltx)
  }

  /// A logical path as the VFS spells it.
  ///
  /// `PathBuf` may have normalized separators for the host, so this converts back rather than trusting `to_string_lossy`.
  fn to_logical(path: &Path) -> String {
    path.to_string_lossy().replace('/', "\\")
  }
}

impl LtxIncludeSource for LtxIncludeVfsSource<'_> {
  fn resolve(&self, directory: &Path, statement: &str) -> XrfResult<Vec<PathBuf>> {
    let directory: String = Self::to_logical(directory);
    let statement: String = statement.replace('/', "\\");

    let joined: String = if directory.is_empty() {
      statement.clone()
    } else {
      format!("{directory}\\{statement}")
    };

    if !statement.contains('*') {
      return Ok(vec![PathBuf::from(xray_path::normalize_logical(&joined)?)]);
    }

    let normalized: String = xray_path::normalize_logical(&joined)?;
    let (prefix, mask) = match normalized.rsplit_once('\\') {
      Some((prefix, mask)) => (prefix.to_string(), mask.to_string()),
      None => (String::new(), normalized.clone()),
    };

    // `#include "sections\*.ltx"` means that one directory, so this asks for its children rather than everything below it.
    let mut resolved: Vec<PathBuf> = self
      .vfs
      .children(self.scope, &prefix)?
      .files
      .into_iter()
      .filter(|location| {
        location
          .logical_path()
          .rsplit('\\')
          .next()
          .is_some_and(|name| LtxIncludeConvertor::matches_wildcard_mask(name, &mask))
      })
      .map(|location| PathBuf::from(location.logical_path()))
      .collect();

    // Sorted so section merging is deterministic, matching what the filesystem source guarantees.
    resolved.sort();

    Ok(resolved)
  }

  fn read(&self, path: &Path) -> XrfResult<Option<Ltx>> {
    let logical_path: String = Self::to_logical(path);

    // A wildcard include resolves only to names the VFS holds, and a named include that is absent is nothing to merge -
    // the same tolerance the filesystem source shows a config not yet generated from TypeScript.
    if self.vfs.find(self.scope, &logical_path)?.is_none() {
      return Ok(None);
    }

    self.read_ltx(&logical_path).map(Some)
  }

  fn describe(&self, path: &Path) -> String {
    format!("{} (logical)", Self::to_logical(path))
  }
}
