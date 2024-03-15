use crate::error::ltx_scheme_error::LtxSchemeError;
use crate::file::configuration::constants::{LTX_SCHEME_FIELD, LTX_SYMBOL_ANY};
use crate::project::verify_options::LtxVerifyOptions;
use crate::scheme::field_data_type::LtxFieldDataType;
use crate::{Ltx, LtxError, LtxProject};
use fxhash::FxBuildHasher;
use indexmap::IndexSet;
use std::path::Path;

impl LtxProject {
  /// Verify all the entries in current ltx project.
  /// Make sure that:
  /// - All included files exist or `.ts` counterpart is declared
  /// - All the inherited sections are valid and declared before inherit attempt
  pub fn verify_entries_opt(&self, options: LtxVerifyOptions) -> Result<bool, LtxError> {
    let mut scheme_errors: Vec<LtxSchemeError> = Vec::new();
    let mut total_sections: usize = 0;
    let mut checked_sections: usize = 0;
    let mut checked_fields: usize = 0;

    if !options.is_silent {
      println!("Verify path: {:?}", self.root);
    }

    // For each file entry in the project:
    for entry in &self.ltx_file_entries {
      let entry_path: &Path = entry.path();

      // Do not check scheme definitions for scheme files - makes no sense.
      if Self::is_ltx_scheme_path(entry_path) {
        continue;
      }

      let ltx: Ltx = Ltx::load_from_file_full(entry_path)?;

      // For each section in file:
      for (section_name, section) in &ltx {
        total_sections += 1;

        // Check only if schema is defined:
        if let Some(scheme_name) = section.get(LTX_SCHEME_FIELD) {
          checked_sections += 1;

          // Check if definition or required schema exists:
          if let Some(scheme_definition) = self.ltx_scheme_declarations.get(scheme_name) {
            let mut validated: IndexSet<String, FxBuildHasher> = Default::default();

            // Check all fields in section data.
            for (field_name, value) in section {
              validated.insert(field_name.into());

              if let Some(field_definition) = scheme_definition
                .fields
                .get(field_name)
                .or_else(|| scheme_definition.fields.get(LTX_SYMBOL_ANY))
              {
                checked_fields += 1;

                let validation_error: Option<LtxSchemeError> = match field_definition.data_type {
                  LtxFieldDataType::TypeSection => {
                    if ltx.has_section(value) {
                      None
                    } else {
                      Some(LtxSchemeError::new_at(
                        section_name,
                        field_name,
                        format!("Required section '{value}' is not in file scope"),
                        entry_path.to_str().unwrap(),
                      ))
                    }
                  }
                  _ => field_definition.validate_value(value),
                };

                if options.is_verbose && !options.is_silent {
                  println!("Checking {:?} [{section_name}] {field_name}", entry_path);
                }

                if let Some(mut error) = validation_error {
                  error.at = Some(entry_path.to_str().unwrap().into());

                  scheme_errors.push(error);
                }
              } else if scheme_definition.is_strict {
                scheme_errors.push(LtxSchemeError::new_at(
                  section_name,
                  field_name,
                  "Unexpected field, definition is required in strict mode",
                  entry_path.to_str().unwrap(),
                ));
              }
            }

            if scheme_definition.is_strict {
              for (field_name, definition) in &scheme_definition.fields {
                if !definition.is_optional
                  && field_name != LTX_SYMBOL_ANY
                  && !validated.contains(field_name)
                {
                  scheme_errors.push(LtxSchemeError::new_at(
                    section_name,
                    field_name,
                    "Required field was not provided",
                    entry_path.to_str().unwrap(),
                  ));
                }
              }
            }
          } else {
            scheme_errors.push(LtxSchemeError::new_at(
              section_name,
              "*",
              format!("Required schema '{scheme_name}' definition is not found"),
              entry_path.to_str().unwrap(),
            ));
          }
        } else if options.is_strict {
          scheme_errors.push(LtxSchemeError::new_at(
            section_name,
            "*",
            "Expected '$schema' field to be defined in strict mode check",
            entry_path.to_str().unwrap(),
          ));
        }
      }
    }

    if !options.is_silent {
      for error in &scheme_errors {
        println!("{}", error);
      }

      println!(
        "Checked {} files, {total_sections} sections",
        self.ltx_files.len()
      );
      println!(
        "Verified {} files, {checked_sections} sections, {checked_fields} fields",
        self.ltx_files.len()
      );
      println!("Found {} error(s)", scheme_errors.len());
    }

    Ok(scheme_errors.is_empty())
  }

  /// Verify all the section/field entries in current ltx project.
  pub fn verify_entries(&self) -> Result<bool, LtxError> {
    self.verify_entries_opt(Default::default())
  }

  /// Format single LTX file by provided path
  pub fn verify_file(path: &Path) -> Result<(), LtxError> {
    Ltx::load_from_file_full(path)?;

    Ok(())
  }
}
