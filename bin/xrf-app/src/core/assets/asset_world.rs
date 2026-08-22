use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use serde::{Deserialize, Serialize};
use xrf_vfs::{XrayProbe, XrayProbePlan, XrayProbeStep, XrayVfs};

use crate::core::types::TauriResult;

/// Where a caller wants an asset looked for, named rather than handed over.
///
/// Self-describing on purpose: a world is identified by what it is made of, never by a handle the backend issued. A
/// webview reload therefore loses nothing, and a surface that did not open a world can still address assets in it —
/// which is what lets one plugin's selection be read by another's preview.
///
/// The subject asset is not part of a spec. A command that already names one — a model being opened — passes it
/// separately, and its own tree and installation are searched ahead of these roots.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetWorldSpec {
  /// Roots searched in the order given.
  pub roots: Vec<String>,
}

/// Every mounted source the application holds, searched through per-request probes.
///
/// One VFS for the process rather than one per world, because mounting is indexed eagerly and idempotent per planned
/// path: a viewer stepping through fifty models under one root pays for one index instead of fifty. Callers never
/// receive the VFS itself, only a probe over the steps their spec asked for, so an unscoped lookup cannot silently span
/// two unrelated worlds.
///
/// Lives in `core/` because it belongs to no command domain: visuals resolves a model's textures through it, and the
/// surfaces that follow — an archive preview, a level view — mount the same world instead of indexing their own.
pub struct AssetWorldState {
  vfs: Mutex<XrayVfs>,
}

impl AssetWorldState {
  pub fn new() -> Self {
    Self {
      vfs: Mutex::new(XrayVfs::new()),
    }
  }

  /// Mounts what a spec names and hands a probe over it to `consumer`.
  ///
  /// Scoped to a closure because a probe borrows the VFS the lock protects: returning one would either leak the guard or
  /// outlive it. It also keeps mounting and searching in one critical section, so two commands opening the same root
  /// cannot both index it.
  ///
  /// `asset`, when given, is searched for beside itself first — its own X-Ray root, then the installation containing it —
  /// which is how the engine finds a texture shipped next to a model rather than in the shared tree.
  pub fn with_probe<T>(
    &self,
    spec: &AssetWorldSpec,
    asset: Option<&Path>,
    consumer: impl FnOnce(&XrayProbe) -> T,
  ) -> TauriResult<T> {
    let mut vfs: MutexGuard<XrayVfs> = self
      .vfs
      .lock()
      .map_err(|error| format!("Failed to search assets - the mounted world is unavailable: {error}"))?;

    let steps: Vec<XrayProbeStep> = Self::plan(spec, asset)?
      .mount_into(&mut vfs)
      .map_err(|error| format!("Failed to mount the asset world: {error}"))?;

    Ok(consumer(&vfs.probe().with_steps(steps)))
  }

  /// Declares the search order: the asset's own neighborhood, then the spec's roots as given.
  ///
  /// Each root is labelled with its own path, because that is what a reader recognizes in a report — the same string the
  /// outcome of a failed lookup lists as searched.
  fn plan(spec: &AssetWorldSpec, asset: Option<&Path>) -> TauriResult<XrayProbePlan> {
    let mut plan: XrayProbePlan = XrayProbePlan::new();

    if let Some(asset) = asset {
      plan = plan
        .with_asset(asset)
        .map_err(|error| format!("Failed to plan the asset's own sources: {error}"))?;
    }

    for root in &spec.roots {
      plan = plan
        .with_root(root, root)
        .map_err(|error| format!("Failed to plan the asset root '{root}': {error}"))?;
    }

    Ok(plan)
  }
}
