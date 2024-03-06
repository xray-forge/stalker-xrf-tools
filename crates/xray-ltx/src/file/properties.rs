use crate::file::iterator::{PropertyIter, PropertyIterMut};
use crate::file::types::PropertiesData;
use std::ops::Index;

/// Properties type (key-value pairs).
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Properties {
  pub inherited: Vec<String>,
  pub data: PropertiesData,
}

impl Properties {
  /// Create an instance.
  pub fn new() -> Properties {
    Default::default()
  }

  /// Get the number of the properties.
  pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Check if properties has 0 elements.
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }

  /// Get an iterator of the properties.
  pub fn iter(&self) -> PropertyIter {
    PropertyIter {
      inner: self.data.iter(),
    }
  }

  /// Get a mutable iterator of the properties.
  pub fn iter_mut(&mut self) -> PropertyIterMut {
    PropertyIterMut {
      inner: self.data.iter_mut(),
    }
  }

  /// Return true if property exist.
  pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
    self.data.contains_key(key.as_ref())
  }

  /// Insert (key, value) pair by replace.
  pub fn insert<K, V>(&mut self, key: K, value: V)
  where
    K: Into<String>,
    V: Into<String>,
  {
    self.data.insert(key.into(), value.into());
  }

  /// Return true if section inherits another section.
  pub fn inherits_section<S>(&self, parent_section: S) -> bool
  where
    S: Into<String>,
  {
    self.inherited.contains(&parent_section.into())
  }

  /// Insert (key, value) pair by replace.
  pub fn inherit<S>(&mut self, parent_section: S)
  where
    S: Into<String>,
  {
    self.inherited.push(parent_section.into());
  }

  /// Append key with (key, value) pair.
  pub fn append<K, V>(&mut self, key: K, value: V)
  where
    K: Into<String>,
    V: Into<String>,
  {
    self.data.insert(key.into(), value.into());
  }

  /// Get the first value associate with the key.
  pub fn get<S: Into<String>>(&self, key: S) -> Option<&str> {
    self.data.get(&key.into()).map(|value| value.as_str())
  }

  /// Get the first value associate with the key.
  pub fn get_mut<S: Into<String>>(&mut self, key: S) -> Option<&mut String> {
    self.data.get_mut(&key.into())
  }

  /// Remove the property with the first value of the key.
  pub fn remove<S: AsRef<str>>(&mut self, key: S) -> Option<String> {
    self.data.shift_remove(key.as_ref())
  }
}

impl<S: AsRef<str>> Index<S> for Properties {
  type Output = str;

  fn index(&self, index: S) -> &str {
    let section: &str = index.as_ref();

    match self.get(section) {
      Some(property) => property,
      None => panic!("Key `{}` does not exist", section),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::Properties;

  #[test]
  fn property_replace() {
    let mut props: Properties = Properties::new();

    assert_eq!(props.len(), 0);

    props.insert("k1", "v1");

    assert_eq!(props.len(), 1);
    assert_eq!(props.get("k1"), Some("v1"));

    props.insert("k1", "v2");

    assert_eq!(props.len(), 1);
    assert_eq!(props.get("k1"), Some("v2"));
  }

  #[test]
  fn property_remove() {
    let mut props = Properties::new();

    props.append("k1", "v1");
    props.append("k1", "v2");

    assert_eq!(props.remove("k1"), Some("v2".into()));
    assert!(!props.contains_key("k1"));
  }
}
