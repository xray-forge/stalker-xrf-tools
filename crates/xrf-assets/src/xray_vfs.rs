use std::collections::HashMap;
use std::path::{Path, PathBuf};

use xrf_error::XrfResult;

use crate::{DirectoryAssetIndex, XrayAsset, XrayAssetIndex, XrayAssetLocation};

/// The engine's view of assets: several sources, searched in order, first hit wins.
///
/// This is `CLocatorAPI` reduced to what a tool needs. The engine merges every root and archive into one table where a
/// later registration overwrites an earlier one (`CLocatorAPI::Register` assigns over the existing entry), so the winner
/// of a name is the last-declared source holding it. Searching sources in reverse declaration order gives the same answer
/// without building the merged table, which is why lookup takes its order per call rather than fixing one at construction:
/// which sources apply depends on the asset being resolved, and a session may hold assets from several trees at once.
///
/// Sources mount lazily and stay mounted, because indexing one walks a whole tree while roots repeat far more often than
/// they differ. A source that will not mount is remembered as the reason it failed, so a broken root costs one walk and
/// stays reportable rather than silently becoming an empty one.
///
/// Archive-backed sources are the reason this exists as a type rather than as a helper: they arrive as another kind of
/// mount, and no caller changes shape when bytes start coming out of a `.db`.
#[derive(Debug, Default)]
pub struct XrayVfs {
  mounts: HashMap<PathBuf, Result<XrayAssetIndex, String>>,
}

impl XrayVfs {
  pub fn new() -> Self {
    Self::default()
  }

  /// The index for a root, mounting it if this is the first time it is asked for.
  ///
  /// @returns The index, or why this root has none.
  pub fn mount(&mut self, root: &Path) -> Result<&XrayAssetIndex, &str> {
    self
      .mounts
      .entry(root.to_path_buf())
      .or_insert_with(|| {
        log::info!("Mounting xray root: {}", root.display());

        DirectoryAssetIndex::read(root)
          .and_then(|directory| XrayAssetIndex::new(directory, &[]))
          .map_err(|error| error.to_string())
      })
      .as_ref()
      .map_err(String::as_str)
  }

  /// A texture reference, resolved as the engine spells it - `.dds` appended, or an authoring extension replaced.
  pub fn dds_texture(&mut self, order: &[PathBuf], reference: &str) -> Option<XrayAssetLocation> {
    self.find_first(order, |index| index.dds_texture(reference))
  }

  /// An asset by its full logical path, such as `textures\wpn\wpn_ak74.dds`.
  pub fn find(&mut self, order: &[PathBuf], logical_path: &str) -> Option<XrayAssetLocation> {
    self.find_first(order, |index| index.find(logical_path))
  }

  /// Roots mounted so far, successfully or not.
  pub fn mount_count(&self) -> usize {
    self.mounts.len()
  }

  /// First source in `order` that answers, as an owned location.
  ///
  /// A rejected lookup - a malformed reference rather than an absent file - is logged and treated as "not here", so one
  /// bad reference cannot stop the remaining sources from being asked.
  fn find_first<F>(&mut self, order: &[PathBuf], lookup: F) -> Option<XrayAssetLocation>
  where
    F: for<'a> Fn(&'a XrayAssetIndex) -> XrfResult<Option<XrayAsset<'a>>>,
  {
    for root in order {
      let Ok(index) = self
        .mount(root)
        .inspect_err(|error| log::warn!("Failed to mount root {}: {error}", root.display()))
      else {
        continue;
      };

      let found: Option<XrayAssetLocation> = lookup(index)
        .inspect_err(|error| log::warn!("Rejected lookup in root {}: {error}", root.display()))
        .ok()
        .flatten()
        .map(XrayAsset::into_location);

      if found.is_some() {
        return found;
      }
    }

    None
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;

  use xrf_test_utils::utils::get_absolute_generated_test_resource_path;

  use crate::{XrayAssetLocation, XrayVfs};

  /// Builds a throwaway root holding the named textures, since mounting is a filesystem fact.
  fn root(name: &str, textures: &[&str]) -> PathBuf {
    let root: PathBuf = get_absolute_generated_test_resource_path(&format!("xray_vfs/{name}"));

    let _ = fs::remove_dir_all(&root);

    for texture in textures {
      let path: PathBuf = root.join("textures").join(texture);

      fs::create_dir_all(path.parent().expect("texture sits in a directory")).expect("test tree is creatable");
      fs::write(&path, name.as_bytes()).expect("test texture is writable");
    }

    root
  }

  #[test]
  fn resolves_a_reference_without_its_extension() {
    let mut vfs: XrayVfs = XrayVfs::new();
    let root: PathBuf = root("plain", &["wpn/wpn_ak74.dds"]);

    let location: XrayAssetLocation = vfs
      .dds_texture(&[root.clone()], "wpn\\wpn_ak74")
      .expect("texture resolves");

    assert_eq!(location.absolute_path(), root.join("textures/wpn/wpn_ak74.dds"));
    assert_eq!(location.logical_path(), "textures\\wpn\\wpn_ak74.dds");
  }

  #[test]
  fn takes_the_first_source_that_holds_the_name() {
    // The engine's last-write-wins, evaluated at query time: pass sources in reverse declaration order.
    let overlay: PathBuf = root("overlay", &["wpn/wpn_ak74.dds"]);
    let base: PathBuf = root("base", &["wpn/wpn_ak74.dds", "wpn/wpn_abakan.dds"]);

    let mut vfs: XrayVfs = XrayVfs::new();
    let order: Vec<PathBuf> = vec![overlay.clone(), base.clone()];

    assert_eq!(
      vfs
        .dds_texture(&order, "wpn\\wpn_ak74")
        .map(|it| it.root().to_path_buf()),
      Some(overlay),
      "the earlier source shadows the later one"
    );

    assert_eq!(
      vfs
        .dds_texture(&order, "wpn\\wpn_abakan")
        .map(|it| it.root().to_path_buf()),
      Some(base),
      "a name only the later source holds still resolves"
    );
  }

  #[test]
  fn answers_none_when_no_source_holds_the_name() {
    let mut vfs: XrayVfs = XrayVfs::new();
    let root: PathBuf = root("sparse", &["wpn/wpn_ak74.dds"]);

    assert!(vfs.dds_texture(&[root], "wpn\\wpn_val").is_none());
  }

  #[test]
  fn skips_a_source_that_will_not_mount_rather_than_stopping() {
    let absent: PathBuf = get_absolute_generated_test_resource_path("xray_vfs/absent");
    let present: PathBuf = root("present", &["wpn/wpn_ak74.dds"]);

    let _ = fs::remove_dir_all(&absent);

    let mut vfs: XrayVfs = XrayVfs::new();

    assert_eq!(
      vfs
        .dds_texture(&[absent, present.clone()], "wpn\\wpn_ak74")
        .map(|it| it.root().to_path_buf()),
      Some(present),
      "a broken source does not hide a working one behind it"
    );
  }

  #[test]
  fn mounts_each_source_once_however_often_it_is_asked() {
    let mut vfs: XrayVfs = XrayVfs::new();
    let root: PathBuf = root("counted", &["wpn/wpn_ak74.dds"]);
    let order: Vec<PathBuf> = vec![root];

    vfs.dds_texture(&order, "wpn\\wpn_ak74");
    vfs.dds_texture(&order, "wpn\\wpn_val");
    vfs.dds_texture(&order, "wpn\\wpn_ak74");

    assert_eq!(vfs.mount_count(), 1, "the same root is not walked twice");
  }

  #[test]
  fn remembers_a_failed_mount_rather_than_rewalking_it() {
    let absent: PathBuf = get_absolute_generated_test_resource_path("xray_vfs/absent_twice");

    let _ = fs::remove_dir_all(&absent);

    let mut vfs: XrayVfs = XrayVfs::new();

    assert!(vfs.mount(&absent).is_err());
    assert!(vfs.mount(&absent).is_err());

    assert_eq!(vfs.mount_count(), 1);
  }

  #[test]
  fn finds_an_asset_by_its_full_logical_path() {
    let mut vfs: XrayVfs = XrayVfs::new();
    let root: PathBuf = root("logical", &["wpn/wpn_ak74.dds"]);

    assert!(vfs.find(&[root.clone()], "textures\\wpn\\wpn_ak74.dds").is_some());
    assert!(
      vfs.find(&[root], "wpn\\wpn_ak74.dds").is_none(),
      "a logical path is not a texture reference and is not completed like one"
    );
  }
}
