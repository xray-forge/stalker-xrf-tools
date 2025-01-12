use crate::constants::NO_SOUND;
use crate::{
  GamedataProject, GamedataProjectVerifyOptions, GamedataProjectWeaponVerificationResult,
  GamedataResult,
};
use colored::Colorize;
use regex::Regex;
use std::path::PathBuf;
use xray_db::{OgfFile, OmfFile, XRayByteOrder};
use xray_ltx::{Ltx, Section};

impl GamedataProject {
  pub fn verify_ltx_weapons(
    &mut self,
    options: &GamedataProjectVerifyOptions,
  ) -> GamedataResult<GamedataProjectWeaponVerificationResult> {
    if options.is_logging_enabled() {
      println!("{}", "Verify gamedata LTX weapons:".green());
    }

    let system_ltx: Ltx = self.ltx_project.get_system_ltx()?;

    let mut checked_weapons_count: usize = 0;
    let mut invalid_weapons_count: usize = 0;

    for (section_name, section) in &system_ltx.sections {
      if Self::is_weapon_section(section) {
        checked_weapons_count += 1;
      } else {
        continue;
      }

      match self.verify_ltx_weapon(options, &system_ltx, section_name, section) {
        Ok(is_valid) => {
          if !is_valid {
            if options.is_logging_enabled() {
              eprintln!("Invalid weapon section: [{section_name}]");
            }

            invalid_weapons_count += 1;
          }
        }
        Err(error) => {
          if options.is_logging_enabled() {
            eprintln!("Invalid weapon section: [{section_name}], failure: {error:?}");
          }

          invalid_weapons_count += 1;
        }
      }
    }

    if options.is_logging_enabled() {
      println!(
        "Verified gamedata weapons, {}/{} valid",
        checked_weapons_count - invalid_weapons_count,
        checked_weapons_count
      );
    }

    Ok(GamedataProjectWeaponVerificationResult {
      is_valid: invalid_weapons_count == 0,
    })
  }

  pub fn verify_ltx_weapon(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ltx: &Ltx,
    section_name: &str,
    section: &Section,
  ) -> GamedataResult<bool> {
    if options.is_verbose_logging_enabled() {
      println!("Verify weapon ltx config [{section_name}]");
    }

    let mut is_weapon_valid: bool = true;

    // todo: Check animations as separate util checker for all existing meshes.
    // todo: Check textures as separate util checker for all existing meshes.

    if !self
      .verify_weapon_hud(options, ltx, section_name, section)
      .is_ok_and(|it| it)
    {
      is_weapon_valid = false;
    }

    if !self
      .verify_weapon_sounds(options, ltx, section_name, section)
      .is_ok_and(|it| it)
    {
      is_weapon_valid = false;
    }

    Ok(is_weapon_valid)
  }

  pub fn verify_weapon_hud(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ltx: &Ltx,
    section_name: &str,
    section: &Section,
  ) -> GamedataResult<bool> {
    let mut is_valid: bool = true;

    if let Some(visual) = &self.get_section_ogf_visual(section, "visual") {
      // Motion refs are not included in check?
      OgfFile::read_from_path::<XRayByteOrder>(visual)?;
    } else {
      if options.is_logging_enabled() {
        eprintln!(
          "Not found visual: [{section_name}] - {:?}",
          section.get("visual")
        );
      }

      is_valid = false;
    }

    let hud_section: &Section = match section.get("hud").and_then(|it| ltx.section(it)) {
      Some(it) => it,
      None => {
        if options.is_logging_enabled() {
          eprintln!(
            "Not found hud section: [{section_name}] - {:?}",
            section.get("hud")
          );
        }

        return Ok(false);
      }
    };

    if let Some(visual_path) = self.get_section_ogf_visual(hud_section, "item_visual") {
      if let Ok(hud_visual) = OgfFile::read_from_path::<XRayByteOrder>(&visual_path) {
        if let Some(motion_refs) = hud_visual.kinematics.map(|it| it.motion_refs) {
          let mut ref_animations: Vec<String> = Vec::new();

          for motion_ref in &motion_refs {
            match OmfFile::read_motions_from_path::<XRayByteOrder>(
              &self
                .get_omf_visual(motion_ref)
                .expect("Motion file for weapon not found in project assets"),
            ) {
              Ok(motions) => ref_animations.extend(motions),
              Err(error) => {
                if options.is_logging_enabled() {
                  eprintln!(
                    "Error reading OMF motions for weapon hud: [{section_name}] : {visual_path:?} - {error:}"
                  );
                }

                is_valid = false;
              }
            }
          }

          for (field_name, field_value) in hud_section {
            if !field_name.starts_with("anm_") {
              continue;
            }

            let animation_name: String = String::from(
              *field_value
                .split(",")
                .collect::<Vec<&str>>()
                .first()
                .unwrap_or(&field_value),
            );

            if !ref_animations.contains(&animation_name) {
              // todo: Check available motions from outfit sections here.
            }
          }
        } else {
          if options.is_logging_enabled() {
            eprintln!("Missing motion refs for weapon hud: [{section_name}] : {visual_path:?}");
          }

          is_valid = false;
        }
      } else {
        if options.is_logging_enabled() {
          eprintln!("Could not read hud visual: [{section_name}] : {visual_path:?}");
        }

        is_valid = false;
      }
    } else {
      if options.is_logging_enabled() {
        eprintln!("Not found hud visual definition: [{section_name}]");
      }

      is_valid = false;
    }

    Ok(is_valid)
  }

  pub fn verify_weapon_sounds(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    ltx: &Ltx,
    section_name: &str,
    section: &Section,
  ) -> GamedataResult<bool> {
    let mut are_sounds_valid: bool = true;

    for sound_section in [
      "snd_draw",
      "snd_empty",
      "snd_holster",
      "snd_reload",
      "snd_shoot",
    ] {
      if !section.contains_key(sound_section) {
        if options.is_logging_enabled() {
          eprintln!("Missing section required weapon sound: [{section_name}] : {sound_section}");
        }

        are_sounds_valid = false;
      }
    }

    for (field_name, field_value) in section {
      if !field_name.starts_with("snd_") {
        continue;
      }

      if field_value == NO_SOUND {
        continue;
      }

      // Layered sounds from OXR/COC.
      if let Some(section) = ltx.section(field_value) {
        if !self
          .verify_weapon_sound_layer(options, ltx, field_value, section)
          .is_ok_and(|it| it)
        {
          are_sounds_valid = false;
        }

        continue;
      }

      if !self
        .verify_weapon_sound_asset(options, section_name, field_name, field_value)
        .is_ok_and(|it| it)
      {
        are_sounds_valid = false
      }
    }

    Ok(are_sounds_valid)
  }

  pub fn verify_weapon_sound_layer(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    _: &Ltx,
    section_name: &str,
    section: &Section,
  ) -> GamedataResult<bool> {
    // Check sound layer structure here and linked sounds:
    //
    // [wpn_abakan_snd_shoot]
    // snd_1_layer = weapons\abakan\abakan_shoot
    // snd_1_layer1 = weapons\abakan\abakan_shoot1

    let mut is_valid: bool = true;

    for (field_name, field_value) in section {
      if !self
        .verify_weapon_sound_layer_field_name(options, section_name, field_name, field_value)
        .is_ok_and(|it| it)
      {
        is_valid = false
      }

      if !self
        .verify_weapon_sound_asset(options, section_name, field_name, field_value)
        .is_ok_and(|it| it)
      {
        is_valid = false
      }
    }

    if is_valid && options.is_verbose_logging_enabled() {
      eprintln!("Sound layers section verified: [{section_name}]");
    }

    Ok(is_valid)
  }

  fn verify_weapon_sound_layer_field_name(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    section_name: &str,
    field_name: &str,
    field_value: &str,
  ) -> GamedataResult<bool> {
    let mut is_valid: bool = true;

    if !Regex::new(r"^snd_([1-9]([0-9]+)?)_layer([1-9]([0-9]+)?)?$")
      .unwrap()
      .is_match(field_name)
    {
      is_valid = false;

      if options.is_logging_enabled() {
        eprintln!(
            "Sound layer field name is invalid, should match pattern: [{section_name}] {field_name} : {field_value}"
          );
      }
    }

    Ok(is_valid)
  }

  fn verify_weapon_sound_asset(
    &mut self,
    options: &GamedataProjectVerifyOptions,
    section_name: &str,
    field_name: &str,
    field_value: &str,
  ) -> GamedataResult<bool> {
    let mut is_valid: bool = true;

    // Sounds field is 1-3 comma separated values:
    let mut sound_object_value: String = String::from(
      *field_value
        .split(",")
        .collect::<Vec<&str>>()
        .first()
        .unwrap_or(&field_value),
    );

    // Support variant with and without extension in ltx files.
    if !sound_object_value.ends_with(".ogg") {
      sound_object_value.push_str(".ogg");
    }

    // todo: Check OGG file, check existing.
    if let Some(sound_path) = self.get_prefixed_relative_asset_path("sounds", &sound_object_value) {
      if sound_path.is_file() && sound_path.exists() {
        if options.is_verbose_logging_enabled() {
          eprintln!(
            "Sound verified in section: [{section_name}] : {field_name} -> {sound_object_value}"
          );
        }
      } else {
        is_valid = false
      }
    } else {
      if options.is_logging_enabled() {
        eprintln!(
          "Sound not found in section: [{section_name}] : {field_name} -> {sound_object_value}"
        );
      }

      is_valid = false;
    }

    Ok(is_valid)
  }
}

impl GamedataProject {
  pub fn get_section_ogf_visual(&mut self, section: &Section, field_name: &str) -> Option<PathBuf> {
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

  pub fn get_omf_visual(&mut self, visual_path: &str) -> Option<PathBuf> {
    let mut visual_path: String = String::from(visual_path);

    if !visual_path.ends_with(".omf") {
      visual_path.push_str(".omf");
    }

    self.get_prefixed_relative_asset_path("meshes", &visual_path)
  }
}
