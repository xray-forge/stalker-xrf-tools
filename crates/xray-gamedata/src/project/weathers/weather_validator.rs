use super::weather_definitions::WeatherDefinitions;
use super::weather_field_rules::{
  WEATHER_REQUIRED_FIELDS, is_valid_weather_field_value, parse_weather_time,
};
use crate::GamedataFindingFactory;
use crate::{Finding, GamedataProject, GamedataProjectVerifyOptions, GamedataVerificationRule};
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
  Ok(
    verify_weather_findings_with_definitions(
      project,
      options,
      config_path,
      definitions,
      definition_load_errors,
    )?
    .is_empty(),
  )
}

/// Validates one weather-cycle LTX file and returns every failure with its source file.
pub fn verify_weather_findings_with_definitions(
  project: &GamedataProject,
  options: &GamedataProjectVerifyOptions,
  config_path: &Path,
  definitions: &WeatherDefinitions,
  definition_load_errors: &mut BTreeSet<String>,
) -> XRayResult<Vec<Finding>> {
  let ltx: Ltx = match Ltx::read_from_file_full(config_path) {
    Ok(ltx) => ltx,
    Err(error) => {
      xray_output::error!(options.output, "Could not open weather LTX: {}", error);

      return Ok(vec![GamedataFindingFactory::for_asset(
        GamedataVerificationRule::WeathersValidation,
        config_path,
        format!("Could not open weather LTX: {error}"),
      )]);
    }
  };
  let weather_sections: Vec<&str> = ltx
    .iter()
    .map(|(section_name, _)| section_name)
    .filter(|section_name| !section_name.is_empty())
    .collect();

  let mut findings: Vec<Finding> = Vec::new();

  if weather_sections.len() < 2 {
    findings.push(report_weather_finding(
      options,
      config_path,
      "Weather file requires at least two time sections",
    ));
  }

  let mut execution_times: HashSet<u32> = HashSet::new();

  for section_name in weather_sections {
    let section: &Section = ltx
      .section(section_name)
      .expect("Expected discovered weather section to exist");

    for field_name in WEATHER_REQUIRED_FIELDS {
      if !section.contains_key(field_name) {
        findings.push(report_weather_finding(
          options,
          config_path,
          format!("Weather [{section_name}] is missing required field [{field_name}]"),
        ));
      }
    }

    if let Some(scheme) = section.get("$scheme")
      && scheme != "$weather"
    {
      findings.push(report_weather_finding(
        options,
        config_path,
        format!("Weather [{section_name}] uses unexpected schema [{scheme}]"),
      ));
    }

    for (field_name, value) in section {
      if !is_valid_weather_field_value(field_name, value) {
        findings.push(report_weather_finding(
          options,
          config_path,
          format!("Weather [{section_name}] has invalid [{field_name}] value [{value}]"),
        ));
      }
    }

    if let Some(execution_time) = parse_weather_time(section_name) {
      if !execution_times.insert(execution_time) {
        findings.push(report_weather_finding(
          options,
          config_path,
          format!("Weather file has duplicate execution time [{section_name}]"),
        ));
      }
    } else {
      findings.push(report_weather_finding(
        options,
        config_path,
        format!("Weather file has invalid time section [{section_name}]"),
      ));
    }

    if let Some(ambient) = section.get("ambient") {
      match &definitions.ambient_sections {
        Ok(ambient_sections) if ambient_sections.contains(ambient) => {}
        Ok(_) => {
          findings.push(report_weather_finding(
            options,
            config_path,
            format!("Weather [{section_name}] references missing ambient [{ambient}]"),
          ));
        }
        Err(error) => {
          definition_load_errors.insert(error.clone());
          findings.push(GamedataFindingFactory::for_asset(
            GamedataVerificationRule::WeathersDefinitions,
            config_path,
            format!("Could not load weather definitions: {error}"),
          ));
        }
      }
    }

    if let Some(sun) = section.get("sun").filter(|sun| !sun.is_empty()) {
      match definitions.has_sun(sun) {
        Ok(true) => {}
        Ok(false) => {
          findings.push(report_weather_finding(
            options,
            config_path,
            format!("Weather [{section_name}] references missing sun [{sun}]"),
          ));
        }
        Err(error) => {
          definition_load_errors.insert(error.clone());
          findings.push(GamedataFindingFactory::for_asset(
            GamedataVerificationRule::WeathersDefinitions,
            config_path,
            format!("Could not load weather definitions: {error}"),
          ));
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
          findings.push(report_weather_finding(
            options,
            config_path,
            format!(
              "Weather [{section_name}] thunderbolt collection [{collection}] references missing definitions [{}]",
              missing_definitions.join(", ")
            ),
          ));
        }
        Ok(None) => {
          findings.push(report_weather_finding(
            options,
            config_path,
            format!(
              "Weather [{section_name}] references missing thunderbolt collection [{collection}]"
            ),
          ));
        }
        Err(error) => {
          definition_load_errors.insert(error.clone());
          findings.push(GamedataFindingFactory::for_asset(
            GamedataVerificationRule::WeathersDefinitions,
            config_path,
            format!("Could not load weather definitions: {error}"),
          ));
        }
      }
    }

    if let Some(sky_texture) = section.get("sky_texture") {
      for texture_reference in [sky_texture.to_string(), format!("{sky_texture}#small")] {
        if project
          .resolve_dds_texture_path(&texture_reference)
          .is_none()
        {
          findings.push(report_weather_finding(
            options,
            config_path,
            format!(
              "Weather [{section_name}] references missing sky texture [{texture_reference}]"
            ),
          ));
        }
      }
    }

    if let Some(clouds_texture) = section.get("clouds_texture")
      && project.resolve_dds_texture_path(clouds_texture).is_none()
    {
      findings.push(report_weather_finding(
        options,
        config_path,
        format!("Weather [{section_name}] references missing clouds texture [{clouds_texture}]"),
      ));
    }
  }

  Ok(findings)
}

fn report_weather_finding(
  options: &GamedataProjectVerifyOptions,
  config_path: &Path,
  message: impl Into<String>,
) -> Finding {
  let message: String = message.into();

  xray_output::error!(options.output, "{}: {}", message, config_path.display());

  GamedataFindingFactory::for_asset(
    GamedataVerificationRule::WeathersValidation,
    config_path,
    message,
  )
}
