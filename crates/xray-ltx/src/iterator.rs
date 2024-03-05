use crate::ltx::Ltx;
use crate::properties::Properties;
use ordered_multimap::list_ordered_multimap::{IntoIter, Iter, IterMut};

pub struct PropertyIter<'a> {
  pub(crate) inner: Iter<'a, String, String>,
}

impl<'a> Iterator for PropertyIter<'a> {
  type Item = (&'a str, &'a str);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(|(k, v)| (k.as_ref(), v.as_ref()))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl DoubleEndedIterator for PropertyIter<'_> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self
      .inner
      .next_back()
      .map(|(k, v)| (k.as_ref(), v.as_ref()))
  }
}

/// Iterator for traversing sections
pub struct PropertyIterMut<'a> {
  pub(crate) inner: IterMut<'a, String, String>,
}

impl<'a> Iterator for PropertyIterMut<'a> {
  type Item = (&'a str, &'a mut String);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(|(k, v)| (k.as_ref(), v))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl DoubleEndedIterator for PropertyIterMut<'_> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.inner.next_back().map(|(k, v)| (k.as_ref(), v))
  }
}

pub struct PropertiesIntoIter {
  inner: IntoIter<String, String>,
}

impl Iterator for PropertiesIntoIter {
  type Item = (String, String);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl DoubleEndedIterator for PropertiesIntoIter {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.inner.next_back()
  }
}

impl<'a> IntoIterator for &'a Properties {
  type Item = (&'a str, &'a str);
  type IntoIter = PropertyIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Properties {
  type Item = (&'a str, &'a mut String);
  type IntoIter = PropertyIterMut<'a>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl IntoIterator for Properties {
  type Item = (String, String);
  type IntoIter = PropertiesIntoIter;

  fn into_iter(self) -> Self::IntoIter {
    PropertiesIntoIter {
      inner: self.data.into_iter(),
    }
  }
}

/// Iterator for traversing sections
pub struct SectionIter<'a> {
  inner: Iter<'a, String, Properties>,
}

impl<'a> Iterator for SectionIter<'a> {
  type Item = (&'a str, &'a Properties);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(|(k, v)| (k.as_str(), v))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl DoubleEndedIterator for SectionIter<'_> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.inner.next_back().map(|(k, v)| (k.as_str(), v))
  }
}

/// Iterator for traversing sections
pub struct SectionIterMut<'a> {
  inner: IterMut<'a, String, Properties>,
}

impl<'a> Iterator for SectionIterMut<'a> {
  type Item = (&'a str, &'a mut Properties);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next().map(|(k, v)| (k.as_str(), v))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl DoubleEndedIterator for SectionIterMut<'_> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.inner.next_back().map(|(k, v)| (k.as_str(), v))
  }
}

/// Iterator for traversing sections
pub struct SectionIntoIter {
  inner: IntoIter<String, Properties>,
}

impl Iterator for SectionIntoIter {
  type Item = (String, Properties);

  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl DoubleEndedIterator for SectionIntoIter {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.inner.next_back()
  }
}

impl<'a> Ltx {
  /// Immutable iterate though sections
  pub fn iter(&'a self) -> SectionIter<'a> {
    SectionIter {
      inner: self.sections.iter(),
    }
  }

  /// Mutable iterate though sections
  #[deprecated(note = "Use `iter_mut` instead!")]
  pub fn mut_iter(&'a mut self) -> SectionIterMut<'a> {
    self.iter_mut()
  }

  /// Mutable iterate though sections
  pub fn iter_mut(&'a mut self) -> SectionIterMut<'a> {
    SectionIterMut {
      inner: self.sections.iter_mut(),
    }
  }
}

impl<'a> IntoIterator for &'a Ltx {
  type Item = (&'a str, &'a Properties);
  type IntoIter = SectionIter<'a>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a> IntoIterator for &'a mut Ltx {
  type Item = (&'a str, &'a mut Properties);
  type IntoIter = SectionIterMut<'a>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl IntoIterator for Ltx {
  type Item = (String, Properties);
  type IntoIter = SectionIntoIter;

  fn into_iter(self) -> Self::IntoIter {
    SectionIntoIter {
      inner: self.sections.into_iter(),
    }
  }
}
