use crate::ltx::Ltx;

/// A setter which could be used to set key-value pair in a specified section.
pub struct SectionSetter<'a> {
  ltx: &'a mut Ltx,
  section_name: Option<String>,
}

impl<'a> SectionSetter<'a> {
  pub fn new(ltx: &'a mut Ltx, section_name: Option<String>) -> SectionSetter<'a> {
    SectionSetter { ltx, section_name }
  }

  /// Set (replace) key-value pair in this section (all with the same name).
  pub fn set<K, V>(&'a mut self, key: K, value: V) -> &'a mut SectionSetter<'a>
  where
    K: Into<String>,
    V: Into<String>,
  {
    self
      .ltx
      .entry(self.section_name.clone())
      .or_insert_with(Default::default)
      .insert(key, value);

    self
  }

  /// Delete the first entry in this section with `key`.
  pub fn delete<K: AsRef<str>>(&'a mut self, key: &K) -> &'a mut SectionSetter<'a> {
    for prop in self.ltx.section_all_mut(self.section_name.as_ref()) {
      prop.remove(key);
    }

    self
  }

  /// Get the entry in this section with `key`.
  pub fn get<K: AsRef<str>>(&'a mut self, key: K) -> Option<&'a str> {
    self
      .ltx
      .section(self.section_name.as_ref())
      .and_then(|prop| prop.get(key))
      .map(AsRef::as_ref)
  }
}
