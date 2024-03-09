use crate::file::section::section::Section;
use crate::file::types::LtxSections;
use crate::{Ltx, LtxConvertError, LtxError};

/// Converter object to process and inject all inherit section statements.
#[derive(Default)]
pub struct LtxInheritConvertor {}

impl LtxInheritConvertor {
  fn new() -> LtxInheritConvertor {
    LtxInheritConvertor {}
  }

  /// Cast LTX file to fully parsed with include sections.
  pub fn convert(ltx: Ltx) -> Result<Ltx, LtxError> {
    LtxInheritConvertor::new().convert_ltx(ltx)
  }
}

impl LtxInheritConvertor {
  /// Convert ltx file with inclusion of inherited sections.
  fn convert_ltx(&self, mut ltx: Ltx) -> Result<Ltx, LtxError> {
    if !ltx.includes.is_empty() {
      return Err(LtxConvertError::new_ltx_error(
        "Failed to convert ltx file, not processed include statements detected on inheritance conversion",
      ));
    }

    // Nothing to parse - no child sections.
    if ltx.sections.is_empty() {
      return Ok(ltx);
    }

    let mut new_sections: LtxSections = Default::default();

    self.inherit_sections(&ltx.sections, &mut new_sections)?;

    ltx.sections = new_sections;

    Ok(ltx)
  }

  fn inherit_sections(
    &self,
    base: &LtxSections,
    destination: &mut LtxSections,
  ) -> Result<(), LtxError> {
    for (key, _) in base {
      Self::inherit_section(base, destination, key)?;
    }

    Ok(())
  }

  fn inherit_section(
    base: &LtxSections,
    destination: &mut LtxSections,
    key: &str,
  ) -> Result<(), LtxError> {
    let section: &Section = match base.get(key) {
      None => {
        return Err(LtxConvertError::new_ltx_error(format!(
          "Failed to inherit unknown section {key} in ltx"
        )));
      }
      Some(it) => it,
    };

    if section.inherited.is_empty() {
      destination.insert(key.into(), section.clone());
    } else {
      for inherited in &section.inherited {
        Self::inherit_section(base, destination, inherited)?;
      }

      let mut new_props: Section = Default::default();

      for inherited in &section.inherited {
        for (key, value) in base.get(inherited).unwrap() {
          new_props.insert(key, value)
        }
      }

      for (key, value) in section {
        new_props.insert(key, value)
      }

      new_props.inherited = Default::default();

      destination.insert(key.into(), new_props);
    }

    Ok(())
  }
}
