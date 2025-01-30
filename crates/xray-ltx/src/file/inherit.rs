use crate::file::section::section::Section;
use crate::file::types::LtxSections;
use crate::Ltx;
use xray_error::{XRayError, XRayResult};

/// Converter object to process and inject all inherit section statements.
#[derive(Default)]
pub struct LtxInheritConvertor {}

impl LtxInheritConvertor {
  fn new() -> Self {
    Self {}
  }

  /// Cast LTX file to fully parsed with include sections.
  pub fn convert(ltx: Ltx) -> XRayResult<Ltx> {
    Self::new().convert_ltx(ltx)
  }
}

impl LtxInheritConvertor {
  /// Convert ltx file with inclusion of inherited sections.
  fn convert_ltx(&self, mut ltx: Ltx) -> XRayResult<Ltx> {
    if !ltx.includes.is_empty() {
      return Err(XRayError::new_convert_error(
        "Failed to equipment ltx file, not processed include statements detected on inheritance conversion",
      ));
    }

    // Nothing to parse - no child sections.
    if ltx.sections.is_empty() {
      return Ok(ltx);
    }

    let mut new_sections: LtxSections = Default::default();

    self.inherit_sections(&ltx, &mut new_sections)?;

    ltx.sections = new_sections;

    Ok(ltx)
  }

  fn inherit_sections(&self, ltx: &Ltx, destination: &mut LtxSections) -> XRayResult {
    for (section_name, _) in &ltx.sections {
      Self::inherit_section(ltx, destination, section_name)?;
    }

    Ok(())
  }

  fn inherit_section(ltx: &Ltx, destination: &mut LtxSections, section_name: &str) -> XRayResult {
    let section: &Section = match ltx.sections.get(section_name) {
      None => {
        return Err(XRayError::new_convert_error(format!(
          "Failed to inherit unknown section [{section_name}] when reading ltx file ({})",
          ltx
            .path
            .as_ref()
            .map_or("virtual", |path| path.to_str().unwrap())
        )));
      }
      Some(it) => it,
    };

    // No need in recursive check multiple times with re-declaration.
    if destination.contains_key(section_name) {
      return Ok(());
    }

    if section.inherited.is_empty() {
      destination.insert(section_name.into(), section.clone());
    } else {
      for inherited in &section.inherited {
        if section_name == inherited {
          return Err(XRayError::new_convert_error(format!(
            "Failed to inherit section '{inherited}' in '{section_name}', cannot inherit self"
          )));
        }

        Self::inherit_section(ltx, destination, inherited)?;
      }

      let mut new_props: Section = Default::default();

      for inherited in &section.inherited {
        for (key, value) in destination.get(inherited).unwrap() {
          new_props.insert(key, value)
        }
      }

      for (key, value) in section {
        new_props.insert(key, value)
      }

      new_props.inherited = Default::default();

      destination.insert(section_name.into(), new_props);
    }

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::file::ltx::Ltx;
  use crate::Section;
  use xray_error::XRayResult;

  #[test]
  fn test_inheritance_chain() {
    let input = "
[base_1]
a = 1
b = 2

[base_2]:base_1
b = 3
c = 4

[base_3]:base_2
c = 10
d = 20

[target]:base_3
e = 100
";

    let ltx: XRayResult<Ltx> = Ltx::read_from_str(input);

    assert!(ltx.is_ok());

    let ltx: XRayResult<Ltx> = ltx.unwrap().into_inherited();

    assert!(ltx.is_ok());

    let output: Ltx = ltx.unwrap();
    assert_eq!(output.len(), 4);

    let target: &Section = output.section("target").unwrap();

    assert_eq!(target.len(), 5);
    assert_eq!(target.get("a"), Some("1"));
    assert_eq!(target.get("b"), Some("3"));
    assert_eq!(target.get("c"), Some("10"));
    assert_eq!(target.get("d"), Some("20"));
    assert_eq!(target.get("e"), Some("100"));

    let base_3: &Section = output.section("base_3").unwrap();

    assert_eq!(base_3.len(), 4);
    assert_eq!(base_3.get("a"), Some("1"));
    assert_eq!(base_3.get("b"), Some("3"));
    assert_eq!(base_3.get("c"), Some("10"));
    assert_eq!(base_3.get("d"), Some("20"));

    let base_2: &Section = output.section("base_2").unwrap();

    assert_eq!(base_2.len(), 3);
    assert_eq!(base_2.get("a"), Some("1"));
    assert_eq!(base_2.get("b"), Some("3"));
    assert_eq!(base_2.get("c"), Some("4"));

    let base_2: &Section = output.section("base_1").unwrap();

    assert_eq!(base_2.len(), 2);
    assert_eq!(base_2.get("a"), Some("1"));
    assert_eq!(base_2.get("b"), Some("2"));
  }
}
