use std::path::{Path, PathBuf};

use xrf_assets::{XrayScope, XrayVfs};
use xrf_error::{XrfError, XrfResult};

use crate::Ltx;
use crate::file::file_configuration::constants::{LTX_EXTENSION, LTX_SCHEME_EXTENSION, LTX_SCHEME_LTX_FILENAME};
use crate::file::include_source::LtxIncludeSource;
use crate::file::include_vfs_source::LtxIncludeVfsSource;
use crate::file::types::LtxSectionSchemes;
use crate::project::ltx_project_options::LtxProjectOptions;
use crate::scheme::parser::LtxSchemeParser;

/// An LTX project assembled from one VFS scope.
///
/// Its files use the VFS's logical X-Ray paths, so callers do not depend on whether a config is loose or archived. Use
/// [`Self::path_of`] for display and [`Self::read_full`] to read a file; a logical path is not a filesystem path.
#[derive(Debug)]
pub struct LtxProject {
  /// Location shown in project output.
  pub root: PathBuf,
  /// LTX entry points not included by another config in scope.
  pub ltx_file_entries: Vec<PathBuf>,
  /// Every LTX logical path in scope.
  pub ltx_files: Vec<PathBuf>,
  /// Scheme-definition LTX paths in scope.
  pub ltx_scheme_files: Vec<PathBuf>,
  /// Scheme entry points not included by another config in scope.
  pub ltx_scheme_file_entries: Vec<PathBuf>,
  /// Section schemes declared by scheme entry points.
  pub ltx_scheme_declarations: LtxSectionSchemes,
  /// Mounted sources that resolve project files.
  vfs: XrayVfs,
  scope: XrayScope,
}

impl LtxProject {
  /// Opens a project over one directory mounted at the logical root.
  ///
  /// # Errors
  ///
  /// Returns an error when the directory cannot be indexed or its project files cannot be assembled.
  pub fn open_at_path_opt<P: AsRef<Path>>(root: P, options: LtxProjectOptions) -> XrfResult<Self> {
    let root: &Path = root.as_ref();
    let mut vfs: XrayVfs = XrayVfs::new();

    vfs.mount_directory("", root)?;

    Self::assemble(root.to_path_buf(), vfs, XrayScope::all(), options)
  }

  /// Opens a directory-backed project with default options.
  pub fn open_at_path<P: AsRef<Path>>(root: P) -> XrfResult<Self> {
    Self::open_at_path_opt(root, Default::default())
  }

  /// Opens a project from an existing VFS scope.
  ///
  /// `root` is reported in user-facing output; the mounts and scope determine which files the project can read.
  ///
  /// # Errors
  ///
  /// Returns an error when a config cannot be read, an include cannot be resolved, or a scheme declaration is invalid.
  pub fn open_at_scope_opt(
    root: impl AsRef<Path>,
    vfs: XrayVfs,
    scope: XrayScope,
    options: LtxProjectOptions,
  ) -> XrfResult<Self> {
    Self::assemble(root.as_ref().to_path_buf(), vfs, scope, options)
  }

  /// Creates an empty project for callers that need the project shape without mounted files.
  pub fn empty(root: impl AsRef<Path>) -> Self {
    Self {
      ltx_file_entries: Vec::new(),
      ltx_files: Vec::new(),
      ltx_scheme_declarations: Default::default(),
      ltx_scheme_file_entries: Vec::new(),
      ltx_scheme_files: Vec::new(),
      root: root.as_ref().to_path_buf(),
      scope: XrayScope::all(),
      vfs: XrayVfs::new(),
    }
  }

  /// Collects the project's files, works out which are entry points, and parses its schemes.
  ///
  /// An entry point is a file nothing else includes, which is why every file's include list is read before any of their
  /// contents.
  fn assemble(root: PathBuf, vfs: XrayVfs, scope: XrayScope, options: LtxProjectOptions) -> XrfResult<Self> {
    let source: LtxIncludeVfsSource = LtxIncludeVfsSource::new(&vfs, &scope);

    let mut ltx_files: Vec<PathBuf> = Vec::new();
    let mut ltx_scheme_files: Vec<PathBuf> = Vec::new();
    let mut included: Vec<PathBuf> = Vec::new();

    for logical_path in Self::collect_logical_paths(&vfs, &scope) {
      let path: PathBuf = PathBuf::from(&logical_path);
      let directory: PathBuf = path.parent().map(Path::to_path_buf).unwrap_or_default();

      for include in &Ltx::read_included_from_vfs(&vfs, &scope, &logical_path)? {
        included.extend(source.resolve(&directory, include)?);
      }

      if options.is_with_schemes_check && Self::is_ltx_scheme_path(&path) {
        ltx_scheme_files.push(path.clone());
      }

      ltx_files.push(path);
    }

    let mut ltx_file_entries: Vec<PathBuf> = Vec::new();
    let mut ltx_file_entries_failures: Vec<(PathBuf, PathBuf)> = Vec::new();

    // Filter our entries not included in other files and consider them entry-points.
    for ltx_file_path in ltx_files.iter() {
      if included.contains(ltx_file_path) {
        continue;
      }

      // To make checks more strict and consistent, verify typos with case-insensitive Windows OS.
      // Linux / sane logics fail when assuming that `ExAmPlE.TxT` is same as `example.txt`.
      // Part of strict checking because original gamedata has such failures.
      if options.is_strict_check
        && let Some(matching_path) = included.iter().find(|it| {
          it.to_str()
            .unwrap()
            .eq_ignore_ascii_case(ltx_file_path.to_str().unwrap())
        })
      {
        ltx_file_entries_failures.push((ltx_file_path.clone(), matching_path.clone()));
        continue;
      }

      ltx_file_entries.push(ltx_file_path.clone());
    }

    // Prepare big message with list of files referenced in case-insensitive check.
    if !ltx_file_entries_failures.is_empty() {
      return Err(XrfError::new_convert_error(format!(
        "Cannot read LTX project safely, detected case-insensitive #include statements:\n{}",
        ltx_file_entries_failures
          .iter()
          .map(|(first, second)| format!("  - {} incorrectly imported as {}", first.display(), second.display()))
          .collect::<Vec<_>>()
          .join("\n")
      )));
    }

    // Filter our entries not included in other files.
    let ltx_scheme_file_entries: Vec<PathBuf> = if options.is_with_schemes_check {
      ltx_scheme_files
        .iter()
        .filter_map(|it| if included.contains(it) { None } else { Some(it.clone()) })
        .collect()
    } else {
      Default::default()
    };

    let ltx_scheme_declarations: LtxSectionSchemes = if options.is_with_schemes_check {
      LtxSchemeParser::parse_from_vfs(&vfs, &scope, &ltx_scheme_file_entries)?
    } else {
      Default::default()
    };

    Ok(Self {
      ltx_file_entries,
      ltx_files,
      ltx_scheme_declarations,
      ltx_scheme_file_entries,
      ltx_scheme_files,
      root,
      scope,
      vfs,
    })
  }

  /// Every LTX logical path in scope, sorted so assembly is deterministic.
  fn collect_logical_paths(vfs: &XrayVfs, scope: &XrayScope) -> Vec<String> {
    let mut paths: Vec<String> = vfs
      .entries(scope)
      .into_iter()
      .map(|location| location.logical_path().to_string())
      .filter(|path| path.ends_with(&format!(".{LTX_EXTENSION}")))
      .collect();

    paths.sort();

    paths
  }
}

impl LtxProject {
  /// Check if provided LTX file is scheme definition file.
  pub fn is_ltx_scheme_path<P: AsRef<Path>>(path: P) -> bool {
    path
      .as_ref()
      .file_name()
      .and_then(|name| name.to_str())
      .is_some_and(|name| name == LTX_SCHEME_LTX_FILENAME || name.ends_with(LTX_SCHEME_EXTENSION))
  }

  /// Returns the VFS that resolves this project's files.
  pub fn vfs(&self) -> &XrayVfs {
    &self.vfs
  }

  pub fn scope(&self) -> &XrayScope {
    &self.scope
  }

  /// Returns the user-facing path for one logical config.
  ///
  /// Loose configs use their filesystem path. Archived or missing configs use the supplied logical path.
  pub fn path_of(&self, logical_path: &Path) -> PathBuf {
    self
      .vfs
      .find(&self.scope, &logical_path.to_string_lossy().replace('/', "\\"))
      .ok()
      .flatten()
      .and_then(|location| location.physical_path())
      .unwrap_or_else(|| logical_path.to_path_buf())
  }

  /// Returns a filesystem path when a loose config resolves.
  ///
  /// Returns `None` for archived or missing configs, so in-place operations can reject them.
  pub fn physical_path_of(&self, logical_path: &Path) -> Option<PathBuf> {
    self
      .vfs
      .find(&self.scope, &logical_path.to_string_lossy().replace('/', "\\"))
      .ok()
      .flatten()
      .and_then(|location| location.physical_path())
  }

  /// Reads one project file with included files merged and inherited sections resolved.
  ///
  /// The project owns this rather than each caller reaching for `Ltx::read_from_file_full`, because only the project knows
  /// whether its files are loose or archived.
  ///
  /// # Errors
  ///
  /// Returns an error when the file is not in scope or cannot be read or parsed.
  pub fn read_full(&self, logical_path: &Path) -> XrfResult<Ltx> {
    Ltx::read_from_vfs_full(
      &self.vfs,
      &self.scope,
      &logical_path.to_string_lossy().replace('/', "\\"),
    )
  }

  pub fn get_system_ltx_path(&self) -> PathBuf {
    PathBuf::from("system.ltx")
  }

  pub fn get_system_ltx(&self) -> XrfResult<Ltx> {
    self.read_full(&self.get_system_ltx_path())
  }
}
