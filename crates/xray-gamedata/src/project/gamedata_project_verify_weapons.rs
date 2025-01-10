use crate::{GamedataProject, GamedataResult};
use std::path::PathBuf;
use xray_db::{OgfFile, XRayByteOrder};
use xray_ltx::{Ltx, Section};

impl GamedataProject {
  pub fn verify_ltx_weapons(&self) -> GamedataResult {
    log::info!("Verify gamedata weapons");

    println!("Verify gamedata LTX weapons");

    let system_ltx: Ltx = self.ltx_project.get_system_ltx()?;

    let mut checked_weapons_count: usize = 0;
    let mut invalid_weapons_count: usize = 0;

    for (section_name, section) in &system_ltx.sections {
      if Self::is_weapon_section(section) {
        checked_weapons_count += 1;
      } else {
        continue;
      }

      let check_result: GamedataResult<bool> =
        self.verify_ltx_weapon(&system_ltx, section_name, section);

      if let Ok(is_valid) = check_result {
        if !is_valid {
          invalid_weapons_count += 1;
        }
      } else {
        invalid_weapons_count += 1;
      }
    }

    log::info!(
      "Verified gamedata weapons, {}/{} valid",
      checked_weapons_count - invalid_weapons_count,
      checked_weapons_count
    );

    println!(
      "Verified gamedata weapons, {}/{} valid",
      checked_weapons_count - invalid_weapons_count,
      checked_weapons_count
    );

    Ok(())
  }

  pub fn verify_ltx_weapon(
    &self,
    ltx: &Ltx,
    section_name: &str,
    section: &Section,
  ) -> GamedataResult<bool> {
    log::info!("Verify weapon ltx config [{section_name}]");

    let visual: Option<PathBuf> = self.get_section_ogf_visual(section, "visual");
    let hud_section: Option<&Section> = section.get("hud").and_then(|it| ltx.section(it));
    let hud_visual: Option<PathBuf> =
      hud_section.and_then(|it| self.get_section_ogf_visual(it, "item_visual"));

    // todo: Check animations as separate util checker for all existing meshes.
    // todo: Check textures as separate util checker for all existing meshes.

    // todo: Check sounds for weapons.

    if let Some(visual) = &visual {
      OgfFile::read_from_path::<XRayByteOrder>(visual)?;
    }

    if let Some(hud_visual) = &hud_visual {
      OgfFile::read_from_path::<XRayByteOrder>(hud_visual)?;
    }

    if visual.is_none() {
      log::info!(
        "Not found visual: [{}] - {:?}",
        section_name,
        section.get("visual")
      );
    }

    if hud_visual.is_none() {
      log::info!("Not found hud visual: [{}]", section_name);
    }

    Ok(visual.is_some() && hud_visual.is_some())
  }

  pub fn get_section_ogf_visual(&self, section: &Section, field_name: &str) -> Option<PathBuf> {
    section
      .get(field_name)
      .map(|it| {
        let mut visual_path: String = String::from(it);

        if !it.ends_with(".ogf") {
          visual_path.push_str(".ogf");
        }

        visual_path
      })
      .and_then(|it| self.get_prefixed_relative_asset_path("meshes", &it))
  }
}
