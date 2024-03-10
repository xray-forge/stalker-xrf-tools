use crate::error::ltx_scheme_error::LtxSchemeError;
use crate::file::configuration::constants::LTX_SCHEME_FIELD;
use crate::project::verify_options::LtxVerifyOptions;
use crate::{Ltx, LtxError, LtxProject};
use std::path::Path;

impl LtxProject {
  /// Verify all the entries in current ltx project.
  /// Make sure that:
  /// - All included files exist or `.ts` counterpart is declared
  /// - All the inherited sections are valid and declared before inherit attempt
  pub fn verify_entries_opt(&self, options: LtxVerifyOptions) -> Result<bool, LtxError> {
    let mut scheme_errors: Vec<LtxSchemeError> = Vec::new();
    let mut checked_sections: usize = 0;
    let mut checked_fields: usize = 0;

    if !options.is_silent {
      println!("Verify path: {:?}", self.root);
    }

    // For each file entry in the project:
    for entry in &self.ltx_file_entries {
      let entry_path: &Path = entry.path();
      let ltx: Ltx = Ltx::load_from_file_full(entry_path)?;

      // For each section in file:
      for (section_name, section) in &ltx {
        // Check only if schema is defined:
        if let Some(scheme_name) = section.get(LTX_SCHEME_FIELD) {
          checked_sections += 1;

          // Check if definition or required schema exists:
          if let Some(scheme_definition) = self.ltx_scheme_declarations.get(scheme_name) {
            for (field_name, _) in section {
              if let Some(field_definition) = scheme_definition.fields.get(field_name) {
                checked_fields += 1;

                let validation_error: Option<LtxSchemeError> = field_definition.validate(section);

                if options.is_verbose && !options.is_silent {
                  println!("Checking {:?} - [{section_name}] {field_name}", entry_path);
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

            if scheme_definition.is_strict
              && section.len() < scheme_definition.get_required_fields_count()
            {
              scheme_errors.push(LtxSchemeError::new_at(
                section_name,
                "*",
                format!(
                  "Not matching count of received fields and scheme definitions, {} got, {} expected",
                  section.len(),
                  scheme_definition.get_required_fields_count(),
                ),
                entry_path.to_str().unwrap()
              ));
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

      println!("Checked {} files", self.ltx_files.len());
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
    let ltx: Ltx = Ltx::read_from_file(path)?;

    ltx.into_included()?.into_inherited()?;

    Ok(())
  }
}
