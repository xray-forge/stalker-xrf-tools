use crate::asset::asset_type::AssetType;
use crate::project::animations::verify_animations_result::GamedataAnimationsVerificationResult;
use crate::project::weapons::weapons_utils::{
  get_weapon_animation_name, is_player_hud_section, is_weapon_section,
};
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Instant;
use xray_db::{OgfFile, OmfFile, XRayByteOrder};
use xray_error::XRayResult;
use xray_ltx::{Ltx, Section};

impl GamedataProject {
  pub fn verify_animations(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<GamedataAnimationsVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata animations:".green(),);
    }

    let started_at: Instant = Instant::now();

    let (checked_huds_count, invalid_huds_count) = self.verify_player_hud_animations(options)?;

    let duration: u128 = started_at.elapsed().as_millis();

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata animations in {} sec",
        (duration as f64) / 1000.0,
      );
    }

    Ok(GamedataAnimationsVerificationResult {
      duration,
      checked_huds_count,
      invalid_huds_count,
    })
  }

  pub fn verify_player_hud_animations(
    &self,
    options: &GamedataProjectVerifyOptions,
  ) -> XRayResult<(u32, u32)> {
    if options.is_verbose_logging_enabled() {
      println!("Verify player hud animations");
    }

    let system_ltx: Ltx = self.ltx_project.get_system_ltx()?;

    let mut checked_huds_count: u32 = 0;
    let mut invalid_huds_count: u32 = 0;

    for (section_name, section) in &system_ltx.sections {
      if !is_player_hud_section(section) {
        continue;
      }

      if options.is_verbose_logging_enabled() {
        println!("Verify player hud config [{section_name}]");
      }

      checked_huds_count += 1;

      if !self
        .verify_player_hud_animation(options, section_name, section)
        .is_ok_and(|it| it)
      {
        if options.is_logging_enabled() {
          println!("Player hud config [{section_name}] is invalid");
        }

        invalid_huds_count += 1;
      }
    }

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata huds, {}/{checked_huds_count} valid",
        checked_huds_count - invalid_huds_count,
      );
    }

    Ok((checked_huds_count, invalid_huds_count))
  }

  pub fn verify_player_hud_animation(
    &self,
    options: &GamedataProjectVerifyOptions,
    section_name: &str,
    section: &Section,
  ) -> XRayResult<bool> {
    let mut is_valid: bool = true;

    let mut hud_motions: HashMap<String, String> = HashMap::new();

    if let Some(visual_path) = &section.get("visual").and_then(|it| self.get_ogf_path(it)) {
      if options.is_verbose_logging_enabled() {
        println!("Read player hud motion refs - [{section_name}] {visual_path:?}");
      }

      match self.read_player_hud_motion_refs(visual_path) {
        Ok(linked_visuals) => {
          if options.is_verbose_logging_enabled() {
            println!(
              "Player hud ogf [{visual_path:?} contains {} linked omf files to check",
              linked_visuals.len()
            );
          }

          for linked_visual in &linked_visuals {
            match OmfFile::read_motions_from_path::<XRayByteOrder>(linked_visual) {
              Ok(motions) => {
                if motions.is_empty() {
                  if options.is_logging_enabled() {
                    eprintln!("No motions in visual: [{section_name}] - {linked_visual:?}",);
                  }

                  is_valid = false;
                }

                for motion in motions {
                  if let Some(_existing) = hud_motions.get(&motion) {
                    if options.is_logging_enabled() {
                      /*
                      eprintln!(
                        "Hud [{section_name}] overwriting '{motion}' ({existing} -> {linked_visual:?})",
                      );
                      */
                    }
                  }

                  hud_motions.insert(motion, linked_visual.to_str().unwrap().to_string());
                }
              }
              Err(error) => {
                if options.is_logging_enabled() {
                  eprintln!(
                    "Failed to read linked visual: [{section_name}] - {linked_visual:?} - {error}",
                  );
                }

                is_valid = false;
              }
            }
          }
        }
        Err(error) => {
          if options.is_logging_enabled() {
            eprintln!(
              "Failed to read linked visuals: [{section_name}] - {visual_path:?} - {error}",
            );
          }

          is_valid = false;
        }
      }
    } else {
      if options.is_logging_enabled() {
        eprintln!(
          "Not found hud visual: [{}] - {:?}",
          section_name,
          section.get("visual")
        );
      }

      is_valid = false;
    }

    if hud_motions.is_empty() {
      if options.is_logging_enabled() {
        eprintln!("Hud [{section_name}] contains no animations");
      }

      is_valid = false;
    } else {
      // Check each weapon with each hood.
      if !self
        .verify_hud_weapons_animations(
          options,
          section_name,
          &hud_motions.keys().collect::<Vec<&String>>(),
        )
        .is_ok_and(|it| it)
      {
        if options.is_logging_enabled() {
          eprintln!("Hud [{section_name}] failed weapons check");
        }

        is_valid = false;
      }
    }

    Ok(is_valid)
  }
}

impl GamedataProject {
  pub fn verify_hud_weapons_animations(
    &self,
    options: &GamedataProjectVerifyOptions,
    section_name: &str,
    motions: &[&String],
  ) -> XRayResult<bool> {
    if options.is_verbose_logging_enabled() {
      println!("Verify weapons animations for [{section_name}]");
    }

    let mut is_valid: bool = true;

    let system_ltx: Ltx = self.ltx_project.get_system_ltx()?;

    for (weapon_section_name, weapon_section) in &system_ltx.sections {
      if !is_weapon_section(weapon_section) {
        continue;
      }

      if let Some(hud_section_name) = weapon_section.get("hud") {
        if let Some(hud_section) = system_ltx.section(hud_section_name) {
          for (field_name, field_value) in hud_section {
            if !field_name.starts_with("anm_") {
              continue;
            }

            let weapon_motion_name: String = get_weapon_animation_name(field_value);

            if !motions.contains(&&weapon_motion_name) {
              if options.is_logging_enabled() {
                eprintln!("Hud [{section_name}] weapon [{weapon_section_name}] {field_name}={weapon_motion_name} -> animation motion is not found")
              }

              is_valid = false;
            }
          }
        } else {
          // Unexpected weapon check, should be handled by another checker.
          if options.is_verbose_logging_enabled() {
            eprintln!(
              "Not able to check weapon hud section [{section_name}] -> [{weapon_section_name}] [{hud_section_name}]"
            );
          }
        }
      } else {
        // Unexpected weapon check, should be handled by another checker.
        if options.is_verbose_logging_enabled() {
          eprintln!("Not able to check weapon hud [{section_name}] -> [{weapon_section_name}] hud");
        }
      }
    }

    Ok(is_valid)
  }

  pub fn read_player_hud_motion_refs(&self, visual_path: &Path) -> XRayResult<HashSet<PathBuf>> {
    let motion_refs: Vec<String> =
      OgfFile::read_motion_refs_from_path::<XRayByteOrder>(visual_path)?;

    let mut assets: HashSet<PathBuf> = HashSet::new();

    for motion_ref in &motion_refs {
      if motion_ref.ends_with("*.omf") {
        let matching_base: String = PathBuf::from("meshes")
          .join(&motion_ref[0..motion_ref.len() - 5])
          .to_str()
          .unwrap()
          .to_string();

        let matching_omf: Vec<String> = self
          .assets
          .iter()
          .filter(|(path, descriptor)| {
            descriptor.asset_type == AssetType::Omf && path.starts_with(&matching_base)
          })
          .map(|(path, _)| path.to_string())
          .collect();

        for omf in matching_omf {
          assets.insert(
            self
              .get_absolute_asset_path(&omf)
              .expect("Defined assets from pattern matching should be existing"),
          );
        }
      } else if let Some(visual_path) = self.get_omf_path(motion_ref) {
        assets.insert(visual_path);
      }
    }

    Ok(assets)
  }
}
