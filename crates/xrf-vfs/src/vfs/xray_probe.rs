use xrf_error::XrfResult;

use crate::vfs::XrayResolution;
use crate::{XrayAsset, XrayAssetType, XrayLookupScope, XrayVfs};

/// One place a probe looks, and the name a report calls it by.
///
/// The label is the caller's, because only the caller knows what a scope means to a reader: `visual root`, `project
/// gamedata`, `level bundle`. It travels into the outcome so a located asset says where it came from without the reader
/// reconstructing the search.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct XrayProbeStep {
  label: String,
  scope: XrayLookupScope,
}

impl XrayProbeStep {
  pub fn new(label: impl Into<String>, scope: XrayLookupScope) -> Self {
    Self {
      label: label.into(),
      scope,
    }
  }

  pub fn get_label(&self) -> &str {
    &self.label
  }

  pub fn get_scope(&self) -> &XrayLookupScope {
    &self.scope
  }
}

impl XrayVfs {
  /// Starts a probe over this VFS, to which steps are added in the order they should be searched.
  ///
  /// The counterpart of [`XrayVfs::scoped`] for a lookup that spans several scopes in a fixed order rather than one.
  pub fn probe(&self) -> XrayProbe<'_> {
    XrayProbe {
      vfs: self,
      steps: Vec::new(),
    }
  }
}

/// An ordered search across several scopes of one VFS, with the winner naming the step it came from.
///
/// The engine resolves a reference by probing places in a fixed order, and the order differs by domain: a visual is
/// searched for beside itself, then in its installation, then in the project's gamedata, while `CRender::texture_load`
/// probes a level bundle before the shared texture tree. Expressing both as steps keeps one mechanism instead of one
/// resolver per domain, which is how the same rule came to be written four times.
///
/// Every lookup here delegates to [`crate::XrayScopedVfs`] — this composes resolution, it does not implement any, so there
/// is still exactly one place a reference becomes a path.
#[derive(Clone, Debug)]
pub struct XrayProbe<'a> {
  vfs: &'a XrayVfs,
  steps: Vec<XrayProbeStep>,
}

impl<'a> XrayProbe<'a> {
  /// Appends a step searched after the ones already added.
  pub fn with_step(mut self, label: impl Into<String>, scope: XrayLookupScope) -> Self {
    self.steps.push(XrayProbeStep::new(label, scope));

    self
  }

  /// Appends steps already planned, preserving the order they were planned in.
  ///
  /// The counterpart of [`crate::XrayProbePlan::mount_into`], which mounts a declared order and hands back its steps.
  pub fn with_steps(mut self, steps: impl IntoIterator<Item = XrayProbeStep>) -> Self {
    self.steps.extend(steps);

    self
  }

  /// Returns the steps in search order.
  pub fn get_steps(&self) -> &[XrayProbeStep] {
    &self.steps
  }

  /// Whether this probe has nothing to search, because no step selects a mounted source.
  ///
  /// A probe with steps over an empty VFS is empty too: what matters is whether a lookup could reach anything, not how
  /// many scopes were declared.
  pub fn is_empty(&self) -> bool {
    !self.steps.iter().any(|step| self.has_mounts(step))
  }

  /// Every source this probe would search, in probe order and without duplicates.
  pub fn list_roots(&self) -> Vec<String> {
    let mut roots: Vec<String> = Vec::new();

    for step in &self.steps {
      for mount in self.vfs.scoped(step.get_scope()).list_mounts() {
        let root: String = mount.get_source().get_root_path().display().to_string();

        if !roots.contains(&root) {
          roots.push(root);
        }
      }
    }

    roots
  }

  /// Resolves an engine reference of one kind, step by step, first hit winning.
  ///
  /// Mask-aware, because a reference of some kinds may name a set: a motion reference such as `wpn\wpn_ak74_*.omf` is one
  /// reference with several answers, and it is still one outcome.
  ///
  /// # Errors
  ///
  /// Returns an error when the kind has no canonical home, or when the reference cannot be normalized as an X-Ray path.
  pub fn resolve(&self, asset_type: XrayAssetType, reference: &str) -> XrfResult<XrayResolution> {
    if self.is_empty() {
      return Ok(XrayResolution::NoScope);
    }

    Ok(match self.locate(asset_type, reference)? {
      Some((step, assets)) => XrayResolution::Resolved {
        step: step.to_string(),
        assets,
      },
      None => XrayResolution::Missing {
        roots: self.list_roots(),
      },
    })
  }

  /// Resolves a reference, falling back to another reference of the same kind when it is absent.
  ///
  /// The fallback is the caller's, because substitution is a per-kind engine rule rather than a VFS one: a texture has the
  /// renderer's dummy, a motion set has nothing to stand in for it.
  ///
  /// # Errors
  ///
  /// Returns an error when the kind has no canonical home, or when either reference cannot be normalized as an X-Ray path.
  pub fn resolve_with_fallback(
    &self,
    asset_type: XrayAssetType,
    reference: &str,
    fallback: &str,
  ) -> XrfResult<XrayResolution> {
    if self.is_empty() {
      return Ok(XrayResolution::NoScope);
    }

    if let Some((step, assets)) = self.locate(asset_type, reference)? {
      return Ok(XrayResolution::Resolved {
        step: step.to_string(),
        assets,
      });
    }

    match self.locate(asset_type, fallback)? {
      Some((step, assets)) => Ok(XrayResolution::Substituted {
        step: step.to_string(),
        fallback: fallback.to_string(),
        assets,
      }),
      None => Ok(XrayResolution::Missing {
        roots: self.list_roots(),
      }),
    }
  }

  /// Reads a located asset through the VFS this probe searches.
  ///
  /// # Errors
  ///
  /// Returns an error when the asset's source cannot produce its bytes.
  pub fn read_asset(&self, asset: &XrayAsset) -> XrfResult<Vec<u8>> {
    self.vfs.read_asset(asset)
  }

  /// The first step holding the reference, with everything it holds for it.
  fn locate(&self, asset_type: XrayAssetType, reference: &str) -> XrfResult<Option<(&str, Vec<XrayAsset>)>> {
    for step in &self.steps {
      // A step selecting nothing is skipped rather than answered, so an unconfigured root does not end the search ahead of
      // a configured one behind it.
      if !self.has_mounts(step) {
        continue;
      }

      let assets: Vec<XrayAsset> = self.vfs.scoped(step.get_scope()).resolve_all(asset_type, reference)?;

      if !assets.is_empty() {
        return Ok(Some((step.get_label(), assets)));
      }
    }

    Ok(None)
  }

  fn has_mounts(&self, step: &XrayProbeStep) -> bool {
    self.vfs.scoped(step.get_scope()).list_mounts().next().is_some()
  }
}
