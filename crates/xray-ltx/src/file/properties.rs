use crate::file::iterator::{PropertyIter, PropertyIterMut};
use ordered_multimap::ListOrderedMultimap;
use std::ops::Index;

/// Properties type (key-value pairs).
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Properties {
  pub inherited: Vec<String>,
  pub data: ListOrderedMultimap<String, String>,
}

impl Properties {
  /// Create an instance.
  pub fn new() -> Properties {
    Default::default()
  }

  /// Get the number of the properties.
  pub fn len(&self) -> usize {
    self.data.keys_len()
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
  pub fn inherits_section<S>(&self, section: S) -> bool
  where
    S: Into<String>,
  {
    self.inherited.contains(&section.into())
  }

  /// Insert (key, value) pair by replace.
  pub fn inherit<S>(&mut self, section: S)
  where
    S: Into<String>,
  {
    self.inherited.push(section.into());
  }

  /// Append key with (key, value) pair.
  pub fn append<K, V>(&mut self, k: K, v: V)
  where
    K: Into<String>,
    V: Into<String>,
  {
    self.data.append(k.into(), v.into());
  }

  /// Get the first value associate with the key.
  pub fn get<S: Into<String>>(&self, s: S) -> Option<&str> {
    self.data.get(&s.into()).map(|v| v.as_str())
  }

  /// Get all values associate with the key.
  pub fn get_all<S: AsRef<str>>(&self, s: S) -> impl DoubleEndedIterator<Item = &str> {
    self.data.get_all(s.as_ref()).map(|v| v.as_str())
  }

  /// Remove the property with the first value of the key.
  pub fn remove<S: AsRef<str>>(&mut self, s: S) -> Option<String> {
    self.data.remove(s.as_ref())
  }

  /// Remove the property with all values with the same key.
  pub fn remove_all<S: AsRef<str>>(
    &mut self,
    s: S,
  ) -> impl DoubleEndedIterator<Item = String> + '_ {
    self.data.remove_all(s.as_ref())
  }

  pub fn get_mut<S: AsRef<str>>(&mut self, s: S) -> Option<&mut str> {
    self.data.get_mut(s.as_ref()).map(|v| v.as_mut_str())
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
    let mut props = Properties::new();
    props.insert("k1", "v1");

    assert_eq!(Some("v1"), props.get("k1"));
    let res = props.get_all("k1").collect::<Vec<&str>>();
    assert_eq!(res, vec!["v1"]);

    props.insert("k1", "v2");
    assert_eq!(Some("v2"), props.get("k1"));

    let res = props.get_all("k1").collect::<Vec<&str>>();
    assert_eq!(res, vec!["v2"]);
  }

  #[test]
  fn property_get_vec() {
    let mut props = Properties::new();
    props.append("k1", "v1");

    assert_eq!(Some("v1"), props.get("k1"));

    props.append("k1", "v2");

    assert_eq!(Some("v1"), props.get("k1"));

    let res = props.get_all("k1").collect::<Vec<&str>>();
    assert_eq!(res, vec!["v1", "v2"]);

    let res = props.get_all("k2").collect::<Vec<&str>>();
    assert!(res.is_empty());
  }

  #[test]
  fn property_remove() {
    let mut props = Properties::new();
    props.append("k1", "v1");
    props.append("k1", "v2");

    let res = props.remove_all("k1").collect::<Vec<String>>();
    assert_eq!(res, vec!["v1", "v2"]);
    assert!(!props.contains_key("k1"));
  }
}
