use super::weather_definitions::WeatherDefinitions;
use super::weather_field_rules::{
  WEATHER_REQUIRED_FIELDS, is_valid_weather_field_value, parse_weather_time,
};
use crate::{GamedataProject, GamedataProjectVerifyOptions};
use std::collections::{BTreeSet, HashSet};
use std::path::Path;
use xray_error::XRayResult;
use xray_ltx::{Ltx, Section};

/// Validates one weather-cycle LTX file against assembled project assets and definitions.
///
/// The check expands includes and inheritance, validates the cycle timeline and canonical fields,
/// and resolves ambient, sun, thunderbolt, sky, and cloud references. When logging is enabled, it
/// reports every discovered problem before returning `Ok(false)`.
pub fn verify_weather_with_definitions(
  project: &GamedataProject,
  options: &GamedataProjectVerifyOptions,
  config_path: &Path,
  definitions: &WeatherDefinitions,
  definition_load_errors: &mut BTreeSet<String>,
) -> XRayResult<bool> {
  let ltx: Ltx = match Ltx::read_from_file_full(config_path) {
    Ok(ltx) => ltx,
    Err(error) => {
      if options.is_logging_enabled() {
        eprintln!("Could not open weather LTX: {}", error);
      }

      return Ok(false);
    }
  };
  let weather_sections: Vec<&str> = ltx
    .iter()
    .map(|(section_name, _)| section_name)
    .filter(|section_name| !section_name.is_empty())
    .collect();

  let mut is_valid: bool = true;

  if weather_sections.len() < 2 {
    if options.is_logging_enabled() {
      eprintln!(
        "Weather file requires at least two time sections: {}",
        config_path.display()
      );
    }

    is_valid = false;
  }

  let mut execution_times: HashSet<u32> = HashSet::new();

  for section_name in weather_sections {
    let section: &Section = ltx
      .section(section_name)
      .expect("Expected discovered weather section to exist");

    for field_name in WEATHER_REQUIRED_FIELDS {
      if !section.contains_key(field_name) {
        if options.is_logging_enabled() {
          eprintln!(
            "Weather [{}] is missing required field [{}]: {}",
            section_name,
            field_name,
            config_path.display()
          );
        }

        is_valid = false;
      }
    }

    if let Some(scheme) = section.get("$scheme")
      && scheme != "$weather"
    {
      if options.is_logging_enabled() {
        eprintln!(
          "Weather [{}] uses unexpected schema [{}]: {}",
          section_name,
          scheme,
          config_path.display()
        );
      }

      is_valid = false;
    }

    for (field_name, value) in section {
      if !is_valid_weather_field_value(field_name, value) {
        if options.is_logging_enabled() {
          eprintln!(
            "Weather [{}] has invalid [{}] value [{}]: {}",
            section_name,
            field_name,
            value,
            config_path.display()
          );
        }

        is_valid = false;
      }
    }

    if let Some(execution_time) = parse_weather_time(section_name) {
      if !execution_times.insert(execution_time) {
        if options.is_logging_enabled() {
          eprintln!(
            "Weather file has duplicate execution time [{}]: {}",
            section_name,
            config_path.display()
          );
        }

        is_valid = false;
      }
    } else {
      if options.is_logging_enabled() {
        eprintln!(
          "Weather file has invalid time section [{}]: {}",
          section_name,
          config_path.display()
        );
      }

      is_valid = false;
    }

    if let Some(ambient) = section.get("ambient") {
      match &definitions.ambient_sections {
        Ok(ambient_sections) if ambient_sections.contains(ambient) => {}
        Ok(_) => {
          if options.is_logging_enabled() {
            eprintln!(
              "Weather [{}] references missing ambient [{}]: {}",
              section_name,
              ambient,
              config_path.display()
            );
          }

          is_valid = false;
        }
        Err(error) => {
          definition_load_errors.insert(error.clone());
          is_valid = false;
        }
      }
    }

    if let Some(sun) = section.get("sun").filter(|sun| !sun.is_empty()) {
      match definitions.has_sun(sun) {
        Ok(true) => {}
        Ok(false) => {
          if options.is_logging_enabled() {
            eprintln!(
              "Weather [{}] references missing sun [{}]: {}",
              section_name,
              sun,
              config_path.display()
            );
          }

          is_valid = false;
        }
        Err(error) => {
          definition_load_errors.insert(error);
          is_valid = false;
        }
      }
    }

    if let Some(collection) = section
      .get("thunderbolt_collection")
      .filter(|collection| !collection.is_empty())
    {
      match definitions.missing_thunderbolt_definitions(collection) {
        Ok(Some(missing_definitions)) if missing_definitions.is_empty() => {}
        Ok(Some(missing_definitions)) => {
          if options.is_logging_enabled() {
            eprintln!(
              "Weather [{}] thunderbolt collection [{}] references missing definitions [{}]: {}",
              section_name,
              collection,
              missing_definitions.join(", "),
              config_path.display()
            );
          }

          is_valid = false;
        }
        Ok(None) => {
          if options.is_logging_enabled() {
            eprintln!(
              "Weather [{}] references missing thunderbolt collection [{}]: {}",
              section_name,
              collection,
              config_path.display()
            );
          }

          is_valid = false;
        }
        Err(error) => {
          definition_load_errors.insert(error);
          is_valid = false;
        }
      }
    }

    if let Some(sky_texture) = section.get("sky_texture") {
      for texture_reference in [sky_texture.to_string(), format!("{sky_texture}#small")] {
        if project
          .resolve_dds_texture_path(&texture_reference)
          .is_none()
        {
          if options.is_logging_enabled() {
            eprintln!(
              "Weather [{}] references missing sky texture [{}]: {}",
              section_name,
              texture_reference,
              config_path.display()
            );
          }

          is_valid = false;
        }
      }
    }

    if let Some(clouds_texture) = section.get("clouds_texture")
      && project.resolve_dds_texture_path(clouds_texture).is_none()
    {
      if options.is_logging_enabled() {
        eprintln!(
          "Weather [{}] references missing clouds texture [{}]: {}",
          section_name,
          clouds_texture,
          config_path.display()
        );
      }

      is_valid = false;
    }
  }

  Ok(is_valid)
}
