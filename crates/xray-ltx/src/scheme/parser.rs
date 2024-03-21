use crate::error::ltx_convert_error::LtxConvertError;
use crate::error::ltx_error::LtxError;
use crate::error::ltx_read_error::LtxReadError;
use crate::file::configuration::constants::{
  LTX_SCHEME_FIELD, LTX_SCHEME_STRICT_FIELD, LTX_SYMBOL_SCHEME,
};
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

      for (name, section) in &ltx {
        if !name.starts_with(LTX_SYMBOL_SCHEME) {
          return Err(LtxConvertError::new_ltx_error(format!(
            "Failed to parse ltx schemes - scheme section declaration should be prefixed with $, \
             got [{name}]"
          )));
        }

        match schemes.entry(name.into()) {
          Entry::Occupied(_) => {
            return Err(LtxConvertError::new_ltx_error(format!(
              "Failed to parse ltx schemes - duplicate declaration of [{name}] section when reading '{}'",
              &ltx
                .path
                .as_ref()
                .map_or("virtial", |path| path.to_str().unwrap())
            )));
          }
          Entry::Vacant(entry) => {
            entry.insert(Self::parse_section_scheme(name, section)?);
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

    // Insert default definition of $scheme field.
    scheme.fields.insert(
      LTX_SCHEME_FIELD.into(),
      LtxFieldScheme {
        data_type: LtxFieldDataType::TypeString,
        is_array: false,
        is_optional: false,
        name: LTX_SCHEME_FIELD.into(),
        section: section_name.into(),
      },
    );

    for (field_name, value) in section {
      match field_name {
        LTX_SCHEME_STRICT_FIELD => {
          scheme.is_strict =
            Self::parse_strict_mode(field_name, section_name, value).map_err(LtxError::from)?;
        }
        _ => {
          scheme.fields.insert(
            field_name.into(),
            Self::parse_field_scheme(field_name, section_name, value)?,
          );
        }
      }
    }

    Ok(scheme)
  }

  /// Parse LTX field definition from section by field name.
  fn parse_field_scheme(
    field_name: &str,
    section_name: &str,
    field_data: &str,
  ) -> Result<LtxFieldScheme, LtxError> {
    let data_type: LtxFieldDataType =
      LtxFieldDataType::from_field_data(field_name, section_name, field_data)?;

    // Do not allow unknown typing.
    if data_type == LtxFieldDataType::TypeUnknown {
      return Err(LtxReadError::new_ltx_error(format!(
        "Invalid ltx [{section_name}] {field_name} configuration, unknown type '{field_data}' supplied",
      )));
    }

    Ok(LtxFieldScheme {
      data_type,
      is_array: LtxFieldDataType::is_field_data_array(field_data),
      is_optional: LtxFieldDataType::is_field_data_optional(field_data),
      name: field_name.into(),
      section: section_name.into(),
    })
  }

  /// Parse whether strict mode is activated for ltx scheme.
  fn parse_strict_mode(
    field_name: &str,
    section_name: &str,
    field_data: &str,
  ) -> Result<bool, LtxReadError> {
    match field_data.parse::<bool>() {
      Ok(value) => Ok(value),
      Err(_) => Err(LtxReadError::new(format!(
        "Invalid scheme declaration, unexpected value for [{section_name}] {field_name} - '{field_data}', boolean expected"
      ))),
    }
  }
}
