use crate::properties::Properties;
use crate::property::{section_key, SectionKey};
use crate::section_entry::SectionEntry;
use crate::section_setter::SectionSetter;
use ordered_multimap::ListOrderedMultimap;
use std::ops::{Index, IndexMut};
use unicase::UniCase;

#[derive(Debug, Clone)]
pub struct Ltx {
  pub(crate) includes: Vec<String>,
  pub(crate) sections: ListOrderedMultimap<SectionKey, Properties>,
}

impl Ltx {
  /// Create an instance
  pub fn new() -> Ltx {
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
      .expect("There is no general section in this Ltx")
  }

  /// Get the mutable general section
  pub fn general_section_mut(&mut self) -> &mut Properties {
    self
      .section_mut(None::<String>)
      .expect("There is no general section in this Ltx")
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

  pub fn include(&mut self, file: String) {
    self.includes.push(file);
  }

  pub fn includes(&self, file: &String) -> bool {
    self.includes.contains(file)
  }

  pub fn get_included(&self) -> &Vec<String> {
    &self.includes
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

impl Default for Ltx {
  /// Creates a ltx instance with an empty general section. This allows [Ltx::general_section]
  /// and [Ltx::with_general_section] to be called without panicking.
  fn default() -> Self {
    let mut result: Ltx = Ltx {
      includes: Default::default(),
      sections: Default::default(),
    };

    result.sections.insert(None, Default::default());

    result
  }
}

impl<S: Into<String>> Index<Option<S>> for Ltx {
  type Output = Properties;

  fn index(&self, index: Option<S>) -> &Properties {
    match self.section(index) {
      Some(p) => p,
      None => panic!("Section does not exist"),
    }
  }
}

impl<S: Into<String>> IndexMut<Option<S>> for Ltx {
  fn index_mut(&mut self, index: Option<S>) -> &mut Properties {
    match self.section_mut(index) {
      Some(p) => p,
      None => panic!("Section does not exist"),
    }
  }
}

impl<'q> Index<&'q str> for Ltx {
  type Output = Properties;

  fn index<'a>(&'a self, index: &'q str) -> &'a Properties {
    match self.section(Some(index)) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

impl<'q> IndexMut<&'q str> for Ltx {
  fn index_mut<'a>(&'a mut self, index: &'q str) -> &'a mut Properties {
    match self.section_mut(Some(index)) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::error::LtxParseError;
  use crate::escape_policy::{escape_str, EscapePolicy};
  use crate::ltx::Ltx;
  use crate::parse_option::ParseOption;
  use crate::properties::Properties;

  #[test]
  fn load_from_str_with_empty_general_section() {
    let input = "[sec1]\nkey1=val1\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert_eq!(output.len(), 2);

    assert!(output.general_section().is_empty());
    assert!(output.general_section_mut().is_empty());

    let props1 = output.section(None::<String>).unwrap();
    assert!(props1.is_empty());
    let props2 = output.section(Some("sec1")).unwrap();
    assert_eq!(props2.len(), 1);
    assert_eq!(props2.get("key1"), Some("val1"));
  }

  #[test]
  fn load_from_str_with_empty_input() {
    let input: &str = "";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert!(output.general_section().is_empty());
    assert!(output.general_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  fn load_from_str_with_empty_lines() {
    let input: &str = "\n\n\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert!(output.general_section().is_empty());
    assert!(output.general_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  fn load_from_str_with_valid_input() {
    let input: &str = "[sec1]\nkey1=val1\nkey2=377\n[sec2]foo=bar\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    // there is always a general section
    assert_eq!(output.len(), 3);
    assert!(output.section(Some("sec1")).is_some());

    let sec1 = output.section(Some("sec1")).unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "val1".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "377".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn load_from_str_without_ending_newline() {
    let input: &str = "[sec1]\nkey1=val1\nkey2=377\n[sec2]foo=bar";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());
  }

  #[test]
  fn parse_error_numbers() {
    let invalid_input: &str = "\n\\x";
    let ltx = Ltx::load_from_str_opt(
      invalid_input,
      ParseOption {
        enabled_escape: true,
        ..Default::default()
      },
    );
    assert!(ltx.is_err());

    let error: LtxParseError = ltx.unwrap_err();

    assert_eq!(error.line, 2);
    assert_eq!(error.col, 3);
  }

  #[test]
  fn parse_comment() {
    let input: &str = "; abcdefghijklmn\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());
  }

  #[test]
  fn iter() {
    let input = "
[section name]
name = hello # abcdefg
gender = mail ; abdddd
";

    let mut ltx: Ltx = Ltx::load_from_str(input).unwrap();

    for _ in &mut ltx {}
    for _ in &ltx {}
    // for _ in ini {}
  }

  #[test]
  fn inherited() {
    let input = "
[section_name]: base1, base2, base3
name = hello
key = value ; comment
";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();

    assert_eq!(ltx.get_from(Some("section_name"), "name").unwrap(), "hello");
    assert_eq!(ltx.get_from(Some("section_name"), "key").unwrap(), "value");

    let properties = ltx.section(Some("section_name")).expect("Existing section");

    assert_eq!(properties.inherited.len(), 3);
    assert!(!properties.inherits_section(Some("base0")));
    assert!(properties.inherits_section(Some("base1")));
    assert!(properties.inherits_section(Some("base2")));
    assert!(properties.inherits_section(Some("base3")));
    assert!(!properties.inherits_section(Some("base4")));
  }

  #[test]
  fn includes() {
    let input = "
#include \"file1.ltx\"
#include \"file2.ltx\"
#include \"file3.ltx\"

[section_name]: base1, base2
name = hello
key = value ; comment
";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();

    assert_eq!(ltx.get_from(Some("section_name"), "name").unwrap(), "hello");
    assert_eq!(ltx.get_from(Some("section_name"), "key").unwrap(), "value");

    assert_eq!(ltx.get_included().len(), 3);
    assert!(ltx.includes(&String::from("file1.ltx")));
    assert!(ltx.includes(&String::from("file2.ltx")));
    assert!(ltx.includes(&String::from("file3.ltx")));
  }

  #[test]
  fn includes_no_duplicates() {
    let input = "
#include \"file1.ltx\"
#include \"file1.ltx\"

[section_name]: base1, base2
name = hello
";

    let ltx = Ltx::load_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      ltx.unwrap_err().message,
      "Failed to parse include statement in ltx file, including 'file1.ltx' more than once"
    );
  }

  #[test]
  fn includes_valid() {
    let input = "
#include

[section_name]: base1, base2
name = hello
";

    let ltx = Ltx::load_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      ltx.unwrap_err().message,
      "Expected correct '#include \"config.ltx\"' statement, got '#include'"
    );
  }

  #[test]
  fn includes_only_ltx() {
    let input = "
#include \"file1.ini\"

[section_name]: base1, base2
name = hello
";

    let ltx = Ltx::load_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      ltx.unwrap_err().message,
      "Included file should have .ltx extension, got 'file1.ini'"
    );
  }

  #[test]
  fn includes_empty() {
    let input = "
#include \"\"

[section_name]: base1, base2
name = hello
";

    let ltx = Ltx::load_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      ltx.unwrap_err().message,
      "Expected valid file name in include statement, got empty file name"
    );
  }

  #[test]
  fn includes_not_in_header() {
    let input = "
#include \"file1.ltx\"

[section_name]: base1, base2
name = hello

#include \"file2.ltx\"
";

    let ltx = Ltx::load_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      ltx.unwrap_err().message,
      "Unexpected '#include' statement, all include statements should be part of config heading"
    );
  }

  #[test]
  fn string() {
    let input: &str = "
[section name]
; This is a comment
Key = \"Value\"
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(ltx.get_from(Some("section name"), "Key").unwrap(), "Value");
  }

  #[test]
  fn string_multiline() {
    let input: &str = "
[section name]
; This is a comment
Key = \"Value
Otherline\"
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from(Some("section name"), "Key").unwrap(),
      "Value\nOtherline"
    );
  }

  #[test]
  fn string_comment() {
    let input: &str = "
[section name]
; This is a comment
Key = \"Value   # This is not a comment ; at all\"
Stuff = Other
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from(Some("section name"), "Key").unwrap(),
      "Value   # This is not a comment ; at all"
    );
  }

  #[test]
  fn string_single() {
    let input: &str = "
[section name]
; This is a comment
Key = 'Value'
Stuff = Other
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(ltx.get_from(Some("section name"), "Key").unwrap(), "Value");
  }

  #[test]
  fn string_includes_quote() {
    let input: &str = "
[Test]
Comment[tr]=İnternet'e erişin
Comment[uk]=Доступ до Інтернету
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from(Some("Test"), "Comment[tr]").unwrap(),
      "İnternet'e erişin"
    );
  }

  #[test]
  fn string_single_multiline() {
    let input = "
[section name]
; This is a comment
Key = 'Value
Otherline'
Stuff = Other
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from(Some("section name"), "Key").unwrap(),
      "Value\nOtherline"
    );
  }

  #[test]
  fn string_single_comment() {
    let input: &str = "
[section name]
; This is a comment
Key = 'Value   # This is not a comment ; at all'
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from(Some("section name"), "Key").unwrap(),
      "Value   # This is not a comment ; at all"
    );
  }

  #[test]
  fn load_from_str_with_valid_empty_input() {
    let input: &str = "key1=\nkey2=val2\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    assert_eq!(output.len(), 1);
    assert!(output.section(None::<String>).is_some());

    let sec1 = output.section(None::<String>).unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "val2".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn load_from_str_with_crlf() {
    let input: &str = "key1=val1\r\nkey2=val2\r\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    assert_eq!(output.len(), 1);
    assert!(output.section(None::<String>).is_some());
    let sec1 = output.section(None::<String>).unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "val1".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "val2".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn load_from_str_with_cr() {
    let input: &str = "key1=val1\rkey2=val2\r";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    assert_eq!(output.len(), 1);
    assert!(output.section(None::<String>).is_some());
    let sec1 = output.section(None::<String>).unwrap();
    assert_eq!(sec1.len(), 2);
    let key1: String = "key1".into();
    assert!(sec1.contains_key(&key1));
    let key2: String = "key2".into();
    assert!(sec1.contains_key(&key2));
    let val1: String = "val1".into();
    assert_eq!(sec1[&key1], val1);
    let val2: String = "val2".into();
    assert_eq!(sec1[&key2], val2);
  }

  #[test]
  fn get_with_non_static_key() {
    let input: &str = "key1=val1\nkey2=val2\n";
    let opt = Ltx::load_from_str(input).unwrap();

    let sec1 = opt.section(None::<String>).unwrap();

    let key = "key1".to_owned();
    sec1.get(&key).unwrap();
  }

  #[test]
  fn load_from_str_noescape() {
    let input: &str = "path=C:\\Windows\\Some\\Folder\\";
    let opt = Ltx::load_from_str_noescape(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    assert_eq!(output.len(), 1);
    let sec = output.section(None::<String>).unwrap();
    assert_eq!(sec.len(), 1);
    assert!(sec.contains_key("path"));
    assert_eq!(&sec["path"], "C:\\Windows\\Some\\Folder\\");
  }

  #[test]
  fn partial_quoting_double() {
    let input: &str = "
[Section]
A=\"quote\" arg0
B=b";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let sec: &Properties = ltx.section(Some("Section")).unwrap();
    assert_eq!(&sec["A"], "quote arg0");
    assert_eq!(&sec["B"], "b");
  }

  #[test]
  fn partial_quoting_single() {
    let input = "
[Section]
A='quote' arg0
B=b";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let section: &Properties = ltx.section(Some("Section")).unwrap();
    assert_eq!(&section["A"], "quote arg0");
    assert_eq!(&section["B"], "b");
  }

  #[test]
  fn parse_without_quote() {
    let input = "
[Desktop Entry]
Exec = \"/path/to/exe with space\" arg
";

    let ltx: Ltx = Ltx::load_from_str_opt(
      input,
      ParseOption {
        enabled_quote: false,
        ..ParseOption::default()
      },
    )
    .unwrap();
    let sec = ltx.section(Some("Desktop Entry")).unwrap();
    assert_eq!(&sec["Exec"], "\"/path/to/exe with space\" arg");
  }

  #[test]
  fn case_insensitive() {
    let input = "
[SecTION]
KeY=value
";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let section = ltx.section(Some("section")).unwrap();
    let val = section.get("key").unwrap();
    assert_eq!("value", val);
  }

  #[test]
  fn preserve_order_section() {
    let input: &str = r"
none2 = n2
[SB]
p2 = 2
[SA]
x2 = 2
[SC]
cd1 = x
[xC]
xd = x
        ";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let keys: Vec<Option<&str>> = ltx.iter().map(|(k, _)| k).collect();

    assert_eq!(keys.len(), 5);
    assert_eq!(keys[0], None);
    assert_eq!(keys[1], Some("SB"));
    assert_eq!(keys[2], Some("SA"));
    assert_eq!(keys[3], Some("SC"));
    assert_eq!(keys[4], Some("xC"));
  }

  #[test]
  fn preserve_order_property() {
    let input = r"
x2 = n2
x1 = n2
x3 = n2
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let section: &Properties = ltx.general_section();
    let keys: Vec<&str> = section.iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec!["x2", "x1", "x3"]);
  }

  #[test]
  fn preserve_order_property_in_section() {
    let input = r"
[s]
x2 = n2
xb = n2
a3 = n3
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let section: &Properties = ltx.section(Some("s")).unwrap();
    let keys: Vec<&str> = section.iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec!["x2", "xb", "a3"])
  }

  #[test]
  fn duplicate_sections() {
    // https://github.com/zonyitoo/rust-ini/issues/49

    let input = r"
[Peer]
foo = a
bar = b

[Peer]
foo = c
bar = d

[Peer]
foo = e
bar = f
";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(3, ltx.section_all(Some("Peer")).count());

    let mut iter = ltx.iter();
    // there is always an empty general section
    let (k0, p0) = iter.next().unwrap();
    assert_eq!(None, k0);
    assert!(p0.is_empty());
    let (k1, p1) = iter.next().unwrap();
    assert_eq!(Some("Peer"), k1);
    assert_eq!(Some("a"), p1.get("foo"));
    assert_eq!(Some("b"), p1.get("bar"));
    let (k2, p2) = iter.next().unwrap();
    assert_eq!(Some("Peer"), k2);
    assert_eq!(Some("c"), p2.get("foo"));
    assert_eq!(Some("d"), p2.get("bar"));
    let (k3, p3) = iter.next().unwrap();
    assert_eq!(Some("Peer"), k3);
    assert_eq!(Some("e"), p3.get("foo"));
    assert_eq!(Some("f"), p3.get("bar"));

    assert_eq!(None, iter.next());
  }

  #[test]
  fn new_has_empty_general_section() {
    let mut ltx: Ltx = Ltx::new();

    assert!(ltx.general_section().is_empty());
    assert!(ltx.general_section_mut().is_empty());
    assert_eq!(ltx.len(), 1);
  }

  #[test]
  fn fix_issue63() {
    let section = "PHP";
    let key = "engine";
    let value = "On";
    let new_value = "Off";

    // create a new configuration
    let mut conf = Ltx::new();
    conf.with_section(Some(section)).set(key, value);

    // assert the value is the one expected
    let v = conf.get_from(Some(section), key).unwrap();
    assert_eq!(v, value);

    // update the section/key with a new value
    conf.set_to(Some(section), key.to_string(), new_value.to_string());

    // assert the new value was set
    let v = conf.get_from(Some(section), key).unwrap();
    assert_eq!(v, new_value);
  }

  #[test]
  fn escape_str_nothing_policy() {
    let test_str = "\0\x07\n字'\"✨🍉杓";
    // This policy should never escape anything.
    let policy = EscapePolicy::Nothing;
    assert_eq!(escape_str(test_str, policy), test_str);
  }

  #[test]
  fn escape_str_basics() {
    let test_backslash = r"\backslashes\";
    let test_nul = "string with \x00nulls\x00 in it";
    let test_controls = "|\x07| bell, |\x08| backspace, |\x7f| delete, |\x1b| escape";
    let test_whitespace = "\t \r\n";

    assert_eq!(
      escape_str(test_backslash, EscapePolicy::Nothing),
      test_backslash
    );
    assert_eq!(escape_str(test_nul, EscapePolicy::Nothing), test_nul);
    assert_eq!(
      escape_str(test_controls, EscapePolicy::Nothing),
      test_controls
    );
    assert_eq!(
      escape_str(test_whitespace, EscapePolicy::Nothing),
      test_whitespace
    );

    for policy in [
      EscapePolicy::Basics,
      EscapePolicy::BasicsUnicode,
      EscapePolicy::BasicsUnicodeExtended,
      EscapePolicy::Reserved,
      EscapePolicy::ReservedUnicode,
      EscapePolicy::ReservedUnicodeExtended,
      EscapePolicy::Everything,
    ] {
      assert_eq!(escape_str(test_backslash, policy), r"\\backslashes\\");
      assert_eq!(escape_str(test_nul, policy), r"string with \0nulls\0 in it");
      assert_eq!(
        escape_str(test_controls, policy),
        r"|\a| bell, |\b| backspace, |\x007f| delete, |\x001b| escape"
      );
      assert_eq!(escape_str(test_whitespace, policy), r"\t \r\n");
    }
  }

  #[test]
  fn escape_str_reserved() {
    // Test reserved characters.
    let test_reserved = ":=;#";
    // And characters which are *not* reserved, but look like they might be.
    let test_punctuation = "!@$%^&*()-_+/?.>,<[]{}``";

    // These policies should *not* escape reserved characters.
    for policy in [
      EscapePolicy::Nothing,
      EscapePolicy::Basics,
      EscapePolicy::BasicsUnicode,
      EscapePolicy::BasicsUnicodeExtended,
    ] {
      assert_eq!(escape_str(test_reserved, policy), ":=;#");
      assert_eq!(escape_str(test_punctuation, policy), test_punctuation);
    }

    // These should.
    for policy in [
      EscapePolicy::Reserved,
      EscapePolicy::ReservedUnicodeExtended,
      EscapePolicy::ReservedUnicode,
      EscapePolicy::Everything,
    ] {
      assert_eq!(escape_str(test_reserved, policy), r"\:\=\;\#");
      assert_eq!(
        escape_str(test_punctuation, policy),
        "!@$%^&*()-_+/?.>,<[]{}``"
      );
    }
  }

  #[test]
  fn escape_str_unicode() {
    // Test unicode escapes.
    // The first are Basic Multilingual Plane (BMP) characters - i.e. <= U+FFFF
    // Emoji are above U+FFFF (e.g. in the 1F???? range), and the CJK characters are in the U+20???? range.
    // The last one is for codepoints at the edge of Rust's char type.
    let test_unicode: &str = r"é£∳字✨";
    let test_emoji: &str = r"🐱😉";
    let test_cjk: &str = r"𠈌𠕇";
    let test_high_points: &str = "\u{10ABCD}\u{10FFFF}";

    let policy = EscapePolicy::Nothing;
    assert_eq!(escape_str(test_unicode, policy), test_unicode);
    assert_eq!(escape_str(test_emoji, policy), test_emoji);
    assert_eq!(escape_str(test_high_points, policy), test_high_points);

    // The "Unicode" policies should escape standard BMP unicode, but should *not* escape emoji or supplementary CJK codepoints.
    // The Basics/Reserved policies should behave identically in this regard.
    for policy in [EscapePolicy::BasicsUnicode, EscapePolicy::ReservedUnicode] {
      assert_eq!(
        escape_str(test_unicode, policy),
        r"\x00e9\x00a3\x2233\x5b57\x2728"
      );
      assert_eq!(escape_str(test_emoji, policy), test_emoji);
      assert_eq!(escape_str(test_cjk, policy), test_cjk);
      assert_eq!(escape_str(test_high_points, policy), test_high_points);
    }

    // UnicodeExtended policies should escape both BMP and supplementary plane characters.
    for policy in [
      EscapePolicy::BasicsUnicodeExtended,
      EscapePolicy::ReservedUnicodeExtended,
    ] {
      assert_eq!(
        escape_str(test_unicode, policy),
        r"\x00e9\x00a3\x2233\x5b57\x2728"
      );
      assert_eq!(escape_str(test_emoji, policy), r"\x1f431\x1f609");
      assert_eq!(escape_str(test_cjk, policy), r"\x2020c\x20547");
      assert_eq!(escape_str(test_high_points, policy), r"\x10abcd\x10ffff");
    }
  }

  #[test]
  fn iter_mut_preserve_order_in_section() {
    let input: &str = r"
x2 = nc
x1 = na
x3 = nb
";

    let mut str: Ltx = Ltx::load_from_str(input).unwrap();
    let section: &mut Properties = str.general_section_mut();
    section.iter_mut().enumerate().for_each(|(i, (_, v))| {
      v.push_str(&i.to_string());
    });
    let props: Vec<_> = section.iter().collect();
    assert_eq!(props, vec![("x2", "nc0"), ("x1", "na1"), ("x3", "nb2")]);
  }

  #[test]
  fn preserve_order_properties_into_iter() {
    let input: &str = r"
x2 = nc
x1 = na
x3 = nb
";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let (_, section) = ltx.into_iter().next().unwrap();
    let props: Vec<_> = section.into_iter().collect();
    assert_eq!(
      props,
      vec![
        ("x2".to_owned(), "nc".to_owned()),
        ("x1".to_owned(), "na".to_owned()),
        ("x3".to_owned(), "nb".to_owned())
      ]
    );
  }
}
