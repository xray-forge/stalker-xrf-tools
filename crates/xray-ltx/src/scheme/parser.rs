use crate::error::ltx_convert_error::LtxConvertError;
use crate::error::ltx_error::LtxError;
use crate::error::ltx_read_error::LtxReadError;
use crate::file::configuration::constants::LTX_SCHEME_STRICT_FIELD;
use crate::file::ltx::Ltx;
use crate::file::section::section::Section;
use crate::file::types::LtxSectionSchemes;
use crate::scheme::field_data_type::LtxFieldDataType;
use crate::scheme::field_scheme::LtxFieldScheme;
use crate::scheme::section_scheme::LtxSectionScheme;
use indexmap::map::Entry;
use walkdir::DirEntry;

/// Parser of LTX scheme definitions.
#[derive(Clone, Debug)]
pub struct LtxSchemeParser {}

impl LtxSchemeParser {
  /// Parse LTX sections scheme definitions from list of files.
  pub fn parse_from_files(files: &Vec<DirEntry>) -> Result<LtxSectionSchemes, LtxError> {
    let mut schemes: LtxSectionSchemes = Default::default();

    for file in files {
      let ltx: Ltx = Ltx::read_from_file(file.path())?
        .into_included()?
        .into_inherited()?;

      for (name, section) in ltx {
        match schemes.entry(name.clone()) {
          Entry::Occupied(_) => {
            return Err(LtxConvertError::new_ltx_error(format!(
              "Failed to parse ltx schemes - duplicate declaration of '{name}' section"
            )));
          }
          Entry::Vacant(entry) => {
            entry.insert(Self::parse_section_scheme(&name, &section)?);
          }
        }
      }
    }

    Ok(schemes)
  }

  /// Parse scheme from section.
  fn parse_section_scheme(
    section_name: &str,
    section: &Section,
  ) -> Result<LtxSectionScheme, LtxError> {
    let mut scheme: LtxSectionScheme = LtxSectionScheme::new(section_name);

    for (field, value) in section {
      if field == LTX_SCHEME_STRICT_FIELD {
        scheme.is_strict = LtxSectionScheme::parse_strict_mode(value).map_err(LtxError::from)?;

        continue;
      }

      match field.split_once('.') {
        None => {
          return Err(LtxReadError::new_ltx_error(format!(
          "Failed to read scheme field '{field}', expected dot separated schema declaration fields"
        )))
        }
        Some((field_name, _)) => {
          if !scheme.fields.contains_key(field_name) {
            scheme.fields.insert(
              field_name.into(),
              Self::parse_field_scheme(field_name, section_name, section)?,
            );
          }
        }
      }
    }

    Ok(scheme)
  }

  /// Parse LTX field definition from section.
  fn parse_field_scheme(
    name: &str,
    section_name: &str,
    section: &Section,
  ) -> Result<LtxFieldScheme, LtxError> {
    let field_type: Option<&str> = section.get(format!("{name}.type"));
    let field_optional: Option<&str> = section.get(format!("{name}.optional"));

    if field_type.is_none() && field_optional.is_none() {
      return Err(LtxReadError::new_ltx_error(format!(
        "Invalid ltx field '{name}' configuration, no valid definitions supplied"
      )));
    }

    let data_type: LtxFieldDataType = LtxFieldDataType::from_field_data_optional(field_type);

    if data_type == LtxFieldDataType::TypeUnknown {
      return Err(LtxReadError::new_ltx_error(format!(
        "Invalid ltx field '{name}' configuration, unknown type '{}' supplied",
        field_type.unwrap()
      )));
    }

    Ok(LtxFieldScheme {
      section: section_name.into(),
      name: name.into(),
      is_optional: LtxFieldDataType::is_field_optional(field_optional),
      data_type,
    })
  }
}
