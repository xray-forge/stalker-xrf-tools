use crate::asset::asset_type::AssetType;
use crate::project::animations::player_hud_animations_verification_result::GamedataPlayerHudAnimationsVerificationResult;
use crate::project::weapons::weapons_utils::{
  get_weapon_animation_name, is_player_hud_section, is_weapon_section,
};
use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationFinding,
  GamedataVerificationRule,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use xray_db::{OgfFile, OmfFile, XRayByteOrder};
use xray_error::XRayResult;
use xray_ltx::{Ltx, Section};

pub(crate) struct PlayerHudAnimationsVerifier<'a> {
  options: &'a GamedataProjectVerifyOptions,
  project: &'a GamedataProject,
}

impl<'a> PlayerHudAnimationsVerifier<'a> {
  pub(crate) fn new(
    project: &'a GamedataProject,
    options: &'a GamedataProjectVerifyOptions,
  ) -> Self {
    Self { options, project }
  }

  pub(crate) fn verify(&self) -> XRayResult<GamedataPlayerHudAnimationsVerificationResult> {
    if self.options.is_verbose_logging_enabled() {
      println!("Verify player hud animations");
    }

    let system_ltx: Ltx = self.project.ltx_project.get_system_ltx()?;
    let system_ltx_path: PathBuf = self.project.ltx_project.get_system_ltx_path();
    let mut result: GamedataPlayerHudAnimationsVerificationResult = Default::default();

    for (section_name, section) in &system_ltx.sections {
      if !is_player_hud_section(section) {
        continue;
      }

      if self.options.is_verbose_logging_enabled() {
        println!("Verify player hud config [{section_name}]");
      }

      result.checked_huds_count += 1;

      if !self
        .verify_player_hud_animation(section_name, section)
        .is_ok_and(|it| it)
      {
        if self.options.is_logging_enabled() {
          println!("Player hud config [{section_name}] is invalid");
        }

        result.findings.push(GamedataVerificationFinding::for_asset(
          GamedataVerificationRule::AnimationsPlayerHud,
          &system_ltx_path,
          format!("Player HUD section [{section_name}] has invalid animations"),
        ));
        result.invalid_huds_count += 1;
      }
    }

    if self.options.is_logging_enabled() {
      println!(
        "Verified gamedata huds, {}/{} valid",
        result.checked_huds_count - result.invalid_huds_count,
        result.checked_huds_count,
      );
    }

    result.findings.sort_by(|left, right| {
      left
        .asset_path()
        .cmp(&right.asset_path())
        .then_with(|| left.message().cmp(right.message()))
    });

    Ok(result)
  }

  fn verify_player_hud_animation(&self, section_name: &str, section: &Section) -> XRayResult<bool> {
    let mut is_valid: bool = true;
    let mut hud_motions: HashSet<String> = HashSet::new();

    if let Some(visual_path) = &section
      .get("visual")
      .and_then(|it| self.project.get_ogf_path(it))
    {
      if self.options.is_verbose_logging_enabled() {
        println!(
          "Read player hud motion refs - [{}] {}",
          section_name,
          visual_path.display()
        );
      }

      match self.read_motion_refs(visual_path) {
        Ok(linked_visuals) => {
          if self.options.is_verbose_logging_enabled() {
            println!(
              "Player hud ogf [{} contains {} linked omf files to check",
              visual_path.display(),
              linked_visuals.len()
            );
          }

          for linked_visual in &linked_visuals {
            match OmfFile::read_motions_from_path::<XRayByteOrder, &PathBuf>(linked_visual) {
              Ok(motions) => {
                if motions.is_empty() {
                  if self.options.is_logging_enabled() {
                    eprintln!(
                      "No motions in visual: [{}] - {}",
                      section_name,
                      linked_visual.display()
                    );
                  }

                  is_valid = false;
                }

                for motion in motions {
                  hud_motions.insert(motion);
                }
              }
              Err(error) => {
                if self.options.is_logging_enabled() {
                  eprintln!(
                    "Failed to read linked visual: [{}] - {} - {}",
                    section_name,
                    linked_visual.display(),
                    error
                  );
                }

                is_valid = false;
              }
            }
          }
        }
        Err(error) => {
          if self.options.is_logging_enabled() {
            eprintln!(
              "Failed to read linked visuals: [{}] - {} - {}",
              section_name,
              visual_path.display(),
              error
            );
          }

          is_valid = false;
        }
      }
    } else {
      if self.options.is_logging_enabled() {
        eprintln!(
          "Not found hud visual: [{}] - {:?}",
          section_name,
          section.get("visual")
        );
      }

      is_valid = false;
    }

    if hud_motions.is_empty() {
      if self.options.is_logging_enabled() {
        eprintln!("Hud [{section_name}] contains no animations");
      }

      is_valid = false;
    } else if !self
      .verify_weapon_animations(section_name, &hud_motions)
      .is_ok_and(|it| it)
    {
      if self.options.is_logging_enabled() {
        eprintln!("Hud [{section_name}] failed weapons check");
      }

      is_valid = false;
    }

    Ok(is_valid)
  }

  fn verify_weapon_animations(
    &self,
    section_name: &str,
    motions: &HashSet<String>,
  ) -> XRayResult<bool> {
    if self.options.is_verbose_logging_enabled() {
      println!("Verify weapons animations for [{section_name}]");
    }

    let mut is_valid: bool = true;
    let system_ltx: Ltx = self.project.ltx_project.get_system_ltx()?;

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

            if !motions.contains(&weapon_motion_name) {
              if self.options.is_logging_enabled() {
                eprintln!(
                  "Hud [{section_name}] weapon [{weapon_section_name}] {field_name}={weapon_motion_name} -> animation motion is not found"
                )
              }

              is_valid = false;
            }
          }
        } else if self.options.is_verbose_logging_enabled() {
          eprintln!(
            "Not able to check weapon hud section [{section_name}] -> [{weapon_section_name}] [{hud_section_name}]"
          );
        }
      } else if self.options.is_verbose_logging_enabled() {
        eprintln!("Not able to check weapon hud [{section_name}] -> [{weapon_section_name}] hud");
      }
    }

    Ok(is_valid)
  }

  fn read_motion_refs<P: AsRef<Path>>(&self, path: &P) -> XRayResult<HashSet<PathBuf>> {
    let motion_refs: Vec<String> = OgfFile::read_motion_refs_from_path::<XRayByteOrder, P>(path)?;
    let mut assets: HashSet<PathBuf> = HashSet::new();

    for motion_ref in &motion_refs {
      if motion_ref.ends_with("*.omf") {
        for (omf_path, descriptor) in self
          .project
          .get_prefixed_masked_assets("meshes", motion_ref)
        {
          if descriptor.asset_type == AssetType::Omf {
            assets.insert(omf_path);
          }
        }
      } else if let Some(visual_path) = self.project.get_omf_path(motion_ref) {
        assets.insert(visual_path);
      }
    }

    Ok(assets)
  }
}
