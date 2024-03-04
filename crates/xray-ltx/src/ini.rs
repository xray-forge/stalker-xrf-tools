use crate::error::{LtxError, ParseError};
use crate::escape_policy::{escape_str, EscapePolicy};
use crate::parse_option::ParseOption;
use crate::parser::Parser;
use crate::properties::Properties;
use crate::property::{section_key, SectionKey};
use crate::section_entry::SectionEntry;
use crate::section_setter::SectionSetter;
use crate::write_option::WriteOption;
use ordered_multimap::ListOrderedMultimap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::{Index, IndexMut};
use std::path::Path;
use unicase::UniCase;

#[derive(Debug, Clone)]
pub struct Ini {
  pub(crate) sections: ListOrderedMultimap<SectionKey, Properties>,
}

impl Ini {
  /// Create an instance
  pub fn new() -> Ini {
    Default::default()
  }

  /// Set with a specified section, `None` is for the general section
  pub fn with_section<S>(&mut self, section: Option<S>) -> SectionSetter
  where
    S: Into<String>,
  {
    SectionSetter::new(self, section.map(Into::into))
  }

  /// Set with general section, a simple wrapper of `with_section(None::<String>)`
  pub fn with_general_section(&mut self) -> SectionSetter {
    self.with_section(None::<String>)
  }

  /// Get the immutable general section
  pub fn general_section(&self) -> &Properties {
    self
      .section(None::<String>)
      .expect("There is no general section in this Ini")
  }

  /// Get the mutable general section
  pub fn general_section_mut(&mut self) -> &mut Properties {
    self
      .section_mut(None::<String>)
      .expect("There is no general section in this Ini")
  }

  /// Get a immutable section
  pub fn section<S>(&self, name: Option<S>) -> Option<&Properties>
  where
    S: Into<String>,
  {
    self.sections.get(&section_key!(name))
  }

  /// Get a mutable section
  pub fn section_mut<S>(&mut self, name: Option<S>) -> Option<&mut Properties>
  where
    S: Into<String>,
  {
    self.sections.get_mut(&section_key!(name))
  }

  /// Get all sections immutable with the same key
  pub fn section_all<S>(&self, name: Option<S>) -> impl DoubleEndedIterator<Item = &Properties>
  where
    S: Into<String>,
  {
    self.sections.get_all(&section_key!(name))
  }

  /// Get all sections mutable with the same key
  pub fn section_all_mut<S>(
    &mut self,
    name: Option<S>,
  ) -> impl DoubleEndedIterator<Item = &mut Properties>
  where
    S: Into<String>,
  {
    self.sections.get_all_mut(&section_key!(name))
  }

  pub fn entry(&mut self, name: Option<String>) -> SectionEntry<'_> {
    SectionEntry::from(self.sections.entry(name.map(UniCase::from)))
  }

  /// Clear all entries
  pub fn clear(&mut self) {
    self.sections.clear()
  }

  /// Iterate with sections
  pub fn sections(&self) -> impl DoubleEndedIterator<Item = Option<&str>> {
    self.sections.keys().map(|s| s.as_ref().map(AsRef::as_ref))
  }

  /// Set key-value to a section
  pub fn set_to<S>(&mut self, section: Option<S>, key: String, value: String)
  where
    S: Into<String>,
  {
    self.with_section(section).set(key, value);
  }

  /// Get the first value from the sections with key
  pub fn get_from<'a, S>(&'a self, section: Option<S>, key: &str) -> Option<&'a str>
  where
    S: Into<String>,
  {
    self
      .sections
      .get(&section_key!(section))
      .and_then(|prop| prop.get(key))
  }

  /// Get the first value from the sections with key, return the default value if it does not exist
  pub fn get_from_or<'a, S>(&'a self, section: Option<S>, key: &str, default: &'a str) -> &'a str
  where
    S: Into<String>,
  {
    self.get_from(section, key).unwrap_or(default)
  }

  /// Get the first mutable value from the sections with key
  pub fn get_from_mut<'a, S>(&'a mut self, section: Option<S>, key: &str) -> Option<&'a mut str>
  where
    S: Into<String>,
  {
    self
      .sections
      .get_mut(&section_key!(section))
      .and_then(|prop| prop.get_mut(key))
  }

  /// Delete the first section with key, return the properties if it exists
  pub fn delete<S>(&mut self, section: Option<S>) -> Option<Properties>
  where
    S: Into<String>,
  {
    let key = section_key!(section);
    self.sections.remove(&key)
  }

  /// Delete the key from the section, return the value if key exists or None
  pub fn delete_from<S>(&mut self, section: Option<S>, key: &str) -> Option<String>
  where
    S: Into<String>,
  {
    self.section_mut(section).and_then(|prop| prop.remove(key))
  }

  /// Total sections count
  pub fn len(&self) -> usize {
    self.sections.keys_len()
  }

  /// Check if object contains no section
  pub fn is_empty(&self) -> bool {
    self.sections.is_empty()
  }
}

impl Default for Ini {
  /// Creates an ini instance with an empty general section. This allows [Ini::general_section]
  /// and [Ini::with_general_section] to be called without panicking.
  fn default() -> Self {
    let mut result = Ini {
      sections: Default::default(),
    };

    result.sections.insert(None, Default::default());

    result
  }
}

impl<S: Into<String>> Index<Option<S>> for Ini {
  type Output = Properties;

  fn index(&self, index: Option<S>) -> &Properties {
    match self.section(index) {
      Some(p) => p,
      None => panic!("Section does not exist"),
    }
  }
}

impl<S: Into<String>> IndexMut<Option<S>> for Ini {
  fn index_mut(&mut self, index: Option<S>) -> &mut Properties {
    match self.section_mut(index) {
      Some(p) => p,
      None => panic!("Section does not exist"),
    }
  }
}

impl<'q> Index<&'q str> for Ini {
  type Output = Properties;

  fn index<'a>(&'a self, index: &'q str) -> &'a Properties {
    match self.section(Some(index)) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

impl<'q> IndexMut<&'q str> for Ini {
  fn index_mut<'a>(&'a mut self, index: &'q str) -> &'a mut Properties {
    match self.section_mut(Some(index)) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

impl Ini {
  /// Write to a file
  pub fn write_to_file<P: AsRef<Path>>(&self, filename: P) -> io::Result<()> {
    self.write_to_file_policy(filename, EscapePolicy::Basics)
  }

  /// Write to a file
  pub fn write_to_file_policy<P: AsRef<Path>>(
    &self,
    filename: P,
    policy: EscapePolicy,
  ) -> io::Result<()> {
    let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(filename.as_ref())?;
    self.write_to_policy(&mut file, policy)
  }

  /// Write to a file with options
  pub fn write_to_file_opt<P: AsRef<Path>>(&self, filename: P, opt: WriteOption) -> io::Result<()> {
    let mut file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .create(true)
      .open(filename.as_ref())?;
    self.write_to_opt(&mut file, opt)
  }

  /// Write to a writer
  pub fn write_to<W: Write>(&self, writer: &mut W) -> io::Result<()> {
    self.write_to_opt(writer, Default::default())
  }

  /// Write to a writer
  pub fn write_to_policy<W: Write>(&self, writer: &mut W, policy: EscapePolicy) -> io::Result<()> {
    self.write_to_opt(
      writer,
      WriteOption {
        escape_policy: policy,
        ..Default::default()
      },
    )
  }

  /// Write to a writer with options
  pub fn write_to_opt<W: Write>(&self, writer: &mut W, opt: WriteOption) -> io::Result<()> {
    let mut firstline = true;

    for (section, props) in &self.sections {
      if !props.data.is_empty() {
        if firstline {
          firstline = false;
        } else {
          // Write an empty line between sections
          writer.write_all(opt.line_separator.as_str().as_bytes())?;
        }
      }

      if let Some(ref section) = *section {
        write!(
          writer,
          "[{}]{}",
          escape_str(&section[..], opt.escape_policy),
          opt.line_separator
        )?;
      }
      for (k, v) in props.iter() {
        let k_str = escape_str(k, opt.escape_policy);
        let v_str = escape_str(v, opt.escape_policy);
        write!(
          writer,
          "{}{}{}{}",
          k_str, opt.kv_separator, v_str, opt.line_separator
        )?;
      }
    }
    Ok(())
  }
}

impl Ini {
  /// Load from a string
  pub fn load_from_str(buf: &str) -> Result<Ini, ParseError> {
    Ini::load_from_str_opt(buf, ParseOption::default())
  }

  /// Load from a string, but do not interpret '\' as an escape character
  pub fn load_from_str_noescape(buf: &str) -> Result<Ini, ParseError> {
    Ini::load_from_str_opt(
      buf,
      ParseOption {
        enabled_escape: false,
        ..ParseOption::default()
      },
    )
  }

  /// Load from a string with options
  pub fn load_from_str_opt(buf: &str, opt: ParseOption) -> Result<Ini, ParseError> {
    let mut parser = Parser::new(buf.chars(), opt);
    parser.parse()
  }

  /// Load from a reader
  pub fn read_from<R: Read>(reader: &mut R) -> Result<Ini, LtxError> {
    Ini::read_from_opt(reader, ParseOption::default())
  }

  /// Load from a reader, but do not interpret '\' as an escape character
  pub fn read_from_noescape<R: Read>(reader: &mut R) -> Result<Ini, LtxError> {
    Ini::read_from_opt(
      reader,
      ParseOption {
        enabled_escape: false,
        ..ParseOption::default()
      },
    )
  }

  /// Load from a reader with options
  pub fn read_from_opt<R: Read>(reader: &mut R, opt: ParseOption) -> Result<Ini, LtxError> {
    let mut s = String::new();
    reader.read_to_string(&mut s).map_err(LtxError::Io)?;
    let mut parser = Parser::new(s.chars(), opt);
    match parser.parse() {
      Err(e) => Err(LtxError::Parse(e)),
      Ok(success) => Ok(success),
    }
  }

  /// Load from a file
  pub fn load_from_file<P: AsRef<Path>>(filename: P) -> Result<Ini, LtxError> {
    Ini::load_from_file_opt(filename, ParseOption::default())
  }

  /// Load from a file, but do not interpret '\' as an escape character
  pub fn load_from_file_noescape<P: AsRef<Path>>(filename: P) -> Result<Ini, LtxError> {
    Ini::load_from_file_opt(
      filename,
      ParseOption {
        enabled_escape: false,
        ..ParseOption::default()
      },
    )
  }

  /// Load from a file with options
  pub fn load_from_file_opt<P: AsRef<Path>>(
    filename: P,
    opt: ParseOption,
  ) -> Result<Ini, LtxError> {
    let mut reader = match File::open(filename.as_ref()) {
      Err(e) => {
        return Err(LtxError::Io(e));
      }
      Ok(r) => r,
    };

    let mut with_bom = false;

    // Check if file starts with a BOM marker
    // UTF-8: EF BB BF
    let mut bom = [0u8; 3];
    if reader.read_exact(&mut bom).is_ok() && &bom == b"\xEF\xBB\xBF" {
      with_bom = true;
    }

    if !with_bom {
      // Reset file pointer
      reader.seek(SeekFrom::Start(0))?;
    }

    Ini::read_from_opt(&mut reader, opt)
  }
}
