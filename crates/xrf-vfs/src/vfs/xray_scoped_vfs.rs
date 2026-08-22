use xrf_error::XrfResult;

use crate::vfs::XrayDirectoryListing;
use crate::{XrayAsset, XrayAssetType, XrayLookupScope, XrayMount, XrayPathCollision, XrayVfs};

impl XrayVfs {
  /// Views this VFS through a scope, so the scope is stated once instead of on every call.
  ///
  /// The view exposes the same lookups as the VFS itself, narrowed to the mounts and subtree the scope admits. It borrows
  /// both halves, so a long-lived holder keeps the VFS and the scope and builds the view at the call site.
  pub fn scoped<'a>(&'a self, scope: &'a XrayLookupScope) -> XrayScopedVfs<'a> {
    XrayScopedVfs { scope, vfs: self }
  }
}

/// One [`XrayVfs`] seen through one [`XrayLookupScope`]: the same lookups, restricted to what the scope admits.
///
/// Made by [`XrayVfs::scoped`]. Reads only — a write names its scope explicitly on [`XrayVfs::write`] and
/// [`XrayVfs::write_override`], because changing bytes deserves the ceremony.
#[derive(Clone, Copy, Debug)]
pub struct XrayScopedVfs<'a> {
  vfs: &'a XrayVfs,
  scope: &'a XrayLookupScope,
}

impl XrayScopedVfs<'_> {
  /// The scope this view looks through.
  pub fn get_scope(&self) -> &XrayLookupScope {
    self.scope
  }

  /// Iterates over the mounts this view admits, preserving priority order.
  pub fn list_mounts(&self) -> impl Iterator<Item = &XrayMount> {
    self.vfs.mounts_in(self.scope)
  }

  /// Like [`XrayVfs::find`], within this view's scope.
  pub fn find(&self, logical_path: &str) -> XrfResult<Option<XrayAsset>> {
    self.vfs.find_in(self.scope, logical_path)
  }

  /// Like [`XrayVfs::find_all`], within this view's scope.
  pub fn find_all(&self, logical_path: &str) -> XrfResult<Vec<XrayAsset>> {
    self.vfs.find_all_in(self.scope, logical_path)
  }

  /// Like [`XrayVfs::read`], within this view's scope.
  pub fn read(&self, logical_path: &str) -> XrfResult<Vec<u8>> {
    self.vfs.read_in(self.scope, logical_path)
  }

  /// Like [`XrayVfs::read_size`], within this view's scope.
  pub fn read_size(&self, logical_path: &str) -> Option<u64> {
    self.vfs.read_size_in(self.scope, logical_path)
  }

  /// Like [`XrayVfs::list_entries`], within this view's scope.
  pub fn list_entries(&self) -> Vec<XrayAsset> {
    self.vfs.list_entries_in(self.scope)
  }

  /// Like [`XrayVfs::list_entries_of_type`], within this view's scope.
  pub fn list_entries_of_type(&self, asset_type: XrayAssetType) -> Vec<XrayAsset> {
    self.vfs.list_entries_of_type_in(self.scope, asset_type)
  }

  /// Like [`XrayVfs::list_entries_with_suffix`], within this view's scope.
  pub fn list_entries_with_suffix(&self, suffix: &str) -> XrfResult<Vec<XrayAsset>> {
    self.vfs.list_entries_with_suffix_in(self.scope, suffix)
  }

  /// Like [`XrayVfs::list_entries_all`], within this view's scope.
  pub fn list_entries_all(&self) -> Vec<XrayAsset> {
    self.vfs.list_entries_all_in(self.scope)
  }

  /// Like [`XrayVfs::list_collisions`], within this view's scope.
  pub fn list_collisions(&self) -> Vec<XrayPathCollision> {
    self.vfs.list_collisions_in(self.scope)
  }

  /// Like [`XrayVfs::list_children`], within this view's scope.
  pub fn list_children(&self, directory: &str) -> XrfResult<XrayDirectoryListing> {
    self.vfs.list_children_in(self.scope, directory)
  }

  /// Like [`XrayVfs::resolve`], within this view's scope.
  pub fn resolve(&self, asset_type: XrayAssetType, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.vfs.resolve_in(self.scope, asset_type, reference)
  }

  /// Like [`XrayVfs::resolve_all`], within this view's scope.
  pub fn resolve_all(&self, asset_type: XrayAssetType, reference: &str) -> XrfResult<Vec<XrayAsset>> {
    self.vfs.resolve_all_in(self.scope, asset_type, reference)
  }

  /// Like [`XrayVfs::resolve_dds_texture`], within this view's scope.
  pub fn dds_texture(&self, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.vfs.resolve_dds_texture_in(self.scope, reference)
  }

  /// Like [`XrayVfs::resolve_ogf`], within this view's scope.
  pub fn ogf(&self, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.vfs.resolve_ogf_in(self.scope, reference)
  }

  /// Like [`XrayVfs::resolve_omf`], within this view's scope.
  pub fn omf(&self, reference: &str) -> XrfResult<Option<XrayAsset>> {
    self.vfs.resolve_omf_in(self.scope, reference)
  }
}
