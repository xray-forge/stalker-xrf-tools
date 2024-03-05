use crate::Properties;
use indexmap::map::{Entry, OccupiedEntry, VacantEntry};

/// A view into a vacant entry in a `Ltx`.
pub struct SectionVacantEntry<'a> {
  inner: VacantEntry<'a, String, Properties>,
}

impl<'a> SectionVacantEntry<'a> {
  /// Insert one new section.
  pub fn insert(self, value: Properties) -> &'a mut Properties {
    self.inner.insert(value)
  }
}

/// A view into an occupied entry in a `Ltx`.
pub struct SectionOccupiedEntry<'a> {
  inner: OccupiedEntry<'a, String, Properties>,
}

impl<'a> SectionOccupiedEntry<'a> {
  /// Into the first internal mutable properties
  pub fn into_mut(self) -> &'a mut Properties {
    self.inner.into_mut()
  }

  /// Append a new section
  pub fn append(&mut self, props: Properties) {
    self.inner.insert(props);
  }
}

/// A view into an `Ltx`, which may either be vacant or occupied.
pub enum SectionEntry<'a> {
  Vacant(SectionVacantEntry<'a>),
  Occupied(SectionOccupiedEntry<'a>),
}

impl<'a> SectionEntry<'a> {
  /// Ensures a value is in the entry by inserting the default if empty,
  /// and returns a mutable reference to the value in the entry.
  pub fn or_insert(self, properties: Properties) -> &'a mut Properties {
    match self {
      SectionEntry::Occupied(entry) => entry.into_mut(),
      SectionEntry::Vacant(entry) => entry.insert(properties),
    }
  }

  /// Ensures a value is in the entry by inserting the result of the default function if empty,
  /// and returns a mutable reference to the value in the entry.
  pub fn or_insert_with<F: FnOnce() -> Properties>(self, default: F) -> &'a mut Properties {
    match self {
      SectionEntry::Occupied(entry) => entry.into_mut(),
      SectionEntry::Vacant(entry) => entry.insert(default()),
    }
  }
}

impl<'a> From<Entry<'a, String, Properties>> for SectionEntry<'a> {
  fn from(entry: Entry<'a, String, Properties>) -> SectionEntry<'a> {
    match entry {
      Entry::Occupied(inner) => SectionEntry::Occupied(SectionOccupiedEntry { inner }),
      Entry::Vacant(inner) => SectionEntry::Vacant(SectionVacantEntry { inner }),
    }
  }
}
