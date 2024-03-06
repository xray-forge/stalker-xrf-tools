use crate::file::ltx_include::LtxIncludeConvertor;
use crate::file::ltx_inherit::LtxInheritConvertor;
use crate::file::section_entry::SectionEntry;
use crate::file::section_setter::SectionSetter;
use crate::file::types::LtxSections;
use crate::{LtxError, ParseOptions, Properties, ROOT_SECTION};
use std::ops::{Index, IndexMut};
use std::path::PathBuf;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Ltx {
  pub(crate) path: Option<PathBuf>,
  pub(crate) directory: Option<PathBuf>,
  pub(crate) includes: Vec<String>,
  pub(crate) sections: LtxSections,
}

impl Ltx {
  /// Create an instance
  pub fn new() -> Ltx {
    Default::default()
  }

  /// Convert current instance of ltx file into full parsed one.
  pub fn into_included(self) -> Result<Ltx, LtxError> {
    LtxIncludeConvertor::convert(self, ParseOptions::default())
  }

  /// Convert current instance of ltx file into full parsed one.
  pub fn into_included_opt(self, options: ParseOptions) -> Result<Ltx, LtxError> {
    LtxIncludeConvertor::convert(self, options)
  }

  /// Convert current instance of ltx file into full parsed one.
  pub fn into_inherited(self) -> Result<Ltx, LtxError> {
    LtxInheritConvertor::convert(self)
  }

  /// Get parent directory of LTX file.
  pub fn get_directory(&self) -> Option<&PathBuf> {
    self.directory.as_ref()
  }

  /// Set with a specified section, `None` is for the general section
  pub fn with_section<S>(&mut self, section: S) -> SectionSetter
  where
    S: Into<String>,
  {
    SectionSetter::new(self, section.into())
  }

  /// Set with general section, a simple wrapper of `with_section(ROOT_SECTION)`
  pub fn with_root_section(&mut self) -> SectionSetter {
    self.with_section(ROOT_SECTION)
  }

  /// Get the immutable general section
  pub fn root_section(&mut self) -> &Properties {
    self
      .entry(ROOT_SECTION.into())
      .or_insert_with(Default::default)
  }

  /// Get the mutable general section
  pub fn root_section_mut(&mut self) -> &mut Properties {
    self
      .section_mut(ROOT_SECTION)
      .expect("There is no root section in this Ltx")
  }

  /// Get a immutable section
  pub fn section<S>(&self, name: S) -> Option<&Properties>
  where
    S: Into<String>,
  {
    self.sections.get(&name.into())
  }

  /// Check whether ltx has section with name.
  pub fn has_section<S>(&self, name: S) -> bool
  where
    S: Into<String>,
  {
    self.sections.contains_key(&name.into())
  }

  /// Get a mutable section
  pub fn section_mut<S>(&mut self, name: S) -> Option<&mut Properties>
  where
    S: Into<String>,
  {
    self.sections.get_mut(&name.into())
  }

  pub fn entry(&mut self, name: String) -> SectionEntry<'_> {
    SectionEntry::from(self.sections.entry(name))
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
  pub fn sections(&self) -> impl DoubleEndedIterator<Item = &str> {
    self.sections.keys().map(|s| s.as_str())
  }

  /// Set key-value to a section
  pub fn set_to<S>(&mut self, section: S, key: String, value: String)
  where
    S: Into<String>,
  {
    self.with_section(section).set(key, value);
  }

  /// Get the first value from the sections with key
  pub fn get_from<S>(&self, section: S, key: &str) -> Option<&str>
  where
    S: Into<String>,
  {
    self
      .sections
      .get(&section.into())
      .and_then(|props| props.get(key))
  }

  /// Get the first value from the sections with key, return the default value if it does not exist
  pub fn get_from_or<'a, S>(&'a self, section: S, key: &str, default: &'a str) -> &'a str
  where
    S: Into<String>,
  {
    self.get_from(section, key).unwrap_or(default)
  }

  /// Get the first mutable value from the sections with key
  pub fn get_from_mut<S>(&mut self, section: S, key: &str) -> Option<&mut str>
  where
    S: Into<String>,
  {
    self
      .sections
      .get_mut(&section.into())
      .and_then(|prop| prop.get_mut(key).map(|it| it.as_mut_str()))
  }

  /// Delete the first section with key, return the properties if it exists
  pub fn delete<S>(&mut self, section: S) -> Option<Properties>
  where
    S: Into<String>,
  {
    self.sections.shift_remove(&section.into())
  }

  /// Delete the key from the section, return the value if key exists or None
  pub fn delete_from<S>(&mut self, section: S, key: &str) -> Option<String>
  where
    S: Into<String>,
  {
    self.section_mut(section).and_then(|prop| prop.remove(key))
  }

  /// Total sections count
  pub fn len(&self) -> usize {
    self.sections.len()
  }

  /// Check if object contains no section
  pub fn is_empty(&self) -> bool {
    self.sections.is_empty()
  }
}

impl<'q> Index<&'q str> for Ltx {
  type Output = Properties;

  fn index<'a>(&'a self, index: &'q str) -> &'a Properties {
    match self.section(index) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

impl<'q> IndexMut<&'q str> for Ltx {
  fn index_mut<'a>(&'a mut self, index: &'q str) -> &'a mut Properties {
    match self.section_mut(index) {
      Some(p) => p,
      None => panic!("Section `{}` does not exist", index),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::file::configuration::escape_policy::escape_str;
  use crate::file::configuration::parse_options::ParseOptions;
  use crate::file::error::LtxParseError;
  use crate::file::ltx::Ltx;
  use crate::file::properties::Properties;
  use crate::{EscapePolicy, ROOT_SECTION};

  #[test]
  fn load_from_str_with_empty_general_section() {
    let input = "[sec1]\nkey1=val1\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert_eq!(output.len(), 1);

    assert!(output.root_section().is_empty());
    assert!(output.root_section_mut().is_empty());

    let props1 = output.section(ROOT_SECTION).unwrap();
    assert!(props1.is_empty());
    let props2 = output.section("sec1").unwrap();
    assert_eq!(props2.len(), 1);
    assert_eq!(props2.get("key1"), Some("val1"));

    // Root section added.
    assert_eq!(output.len(), 2);
  }

  #[test]
  fn load_from_str_with_empty_input() {
    let input: &str = "";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert!(output.root_section().is_empty());
    assert!(output.root_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  fn load_from_str_with_empty_lines() {
    let input: &str = "\n\n\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert!(output.root_section().is_empty());
    assert!(output.root_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  fn load_from_str_with_valid_input() {
    let input: &str = "[sec1]\nkey1=val1\nkey2=377\n[sec2]foo=bar\n";
    let opt = Ltx::load_from_str(input);
    assert!(opt.is_ok());

    let output = opt.unwrap();
    // there is always a general section
    assert_eq!(output.len(), 2);
    assert!(output.section("sec1").is_some());

    let sec1 = output.section("sec1").unwrap();
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
      ParseOptions {
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
name = hello
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

    assert_eq!(ltx.get_from("section_name", "name").unwrap(), "hello");
    assert_eq!(ltx.get_from("section_name", "key").unwrap(), "value");

    let properties = ltx.section("section_name").expect("Existing section");

    assert_eq!(properties.inherited.len(), 3);
    assert!(!properties.inherits_section("base0"));
    assert!(properties.inherits_section("base1"));
    assert!(properties.inherits_section("base2"));
    assert!(properties.inherits_section("base3"));
    assert!(!properties.inherits_section("base4"));
  }

  #[test]
  fn inherited_empty() {
    let input = "
[section_name]: ,,
name = hello
";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let properties: &Properties = ltx.section("section_name").expect("Existing section");

    assert_eq!(properties.inherited.len(), 0);
  }

  #[test]
  fn includes() {
    let input = "
; comment line 1 before
; comment line 2 before
#include \"file1.ltx\"
#include \"file2.ltx\"
; comment line between
#include \"file3.ltx\"

[section_name]: base1, base2
name = hello
key = value ; comment
";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();

    assert_eq!(ltx.get_from("section_name", "name").unwrap(), "hello");
    assert_eq!(ltx.get_from("section_name", "key").unwrap(), "value");

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
    assert_eq!(ltx.get_from("section name", "Key").unwrap(), "Value");
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
      ltx.get_from("section name", "Key").unwrap(),
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
      ltx.get_from("section name", "Key").unwrap(),
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
    assert_eq!(ltx.get_from("section name", "Key").unwrap(), "Value");
  }

  #[test]
  fn string_includes_quote() {
    let input: &str = "
[test]
Comment[tr]=İnternet'e erişin
Comment[uk]=Доступ до Інтернету
";
    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    assert_eq!(
      ltx.get_from("test", "Comment[tr]").unwrap(),
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
      ltx.get_from("section name", "Key").unwrap(),
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
      ltx.get_from("section name", "Key").unwrap(),
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
    assert!(output.section(ROOT_SECTION).is_some());

    let sec1 = output.section(ROOT_SECTION).unwrap();
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
    let ltx: Result<Ltx, LtxParseError> = Ltx::load_from_str(input);

    assert!(ltx.is_ok());

    let ltx: Ltx = ltx.unwrap();
    assert_eq!(ltx.len(), 1);
    assert!(ltx.section(ROOT_SECTION).is_some());

    let sec1: &Properties = ltx.section(ROOT_SECTION).unwrap();
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
    assert!(output.section(ROOT_SECTION).is_some());
    let sec1 = output.section(ROOT_SECTION).unwrap();
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

    let sec1 = opt.section(ROOT_SECTION).unwrap();

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
    let sec = output.section(ROOT_SECTION).unwrap();
    assert_eq!(sec.len(), 1);
    assert!(sec.contains_key("path"));
    assert_eq!(&sec["path"], "C:\\Windows\\Some\\Folder\\");
  }

  #[test]
  fn partial_quoting_double() {
    let input: &str = "
[section]
A=\"quote\" arg0
B=b";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let sec: &Properties = ltx.section("section").unwrap();
    assert_eq!(&sec["A"], "quote arg0");
    assert_eq!(&sec["B"], "b");
  }

  #[test]
  fn partial_quoting_single() {
    let input = "
[section]
A='quote' arg0
B=b";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let section: &Properties = ltx.section("section").unwrap();
    assert_eq!(&section["A"], "quote arg0");
    assert_eq!(&section["B"], "b");
  }

  #[test]
  fn parse_without_quote() {
    let input = "
[desktop_entry]
Exec = \"/path/to/exe with space\" arg
";

    let ltx: Ltx = Ltx::load_from_str_opt(
      input,
      ParseOptions {
        enabled_quote: false,
        ..ParseOptions::default()
      },
    )
    .unwrap();
    let sec = ltx.section("desktop_entry").unwrap();
    assert_eq!(&sec["Exec"], "\"/path/to/exe with space\" arg");
  }

  #[test]
  fn preserve_order_section() {
    let input: &str = r"
none2 = n2
[sb]
p2 = 2
[sa]
x2 = 2
[sc]
cd1 = x
[xc]
xd = x
        ";

    let ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let keys: Vec<&str> = ltx.iter().map(|(k, _)| k).collect();

    assert_eq!(keys.len(), 5);
    assert_eq!(keys[0], ROOT_SECTION);
    assert_eq!(keys[1], "sb");
    assert_eq!(keys[2], "sa");
    assert_eq!(keys[3], "sc");
    assert_eq!(keys[4], "xc");
  }

  #[test]
  fn preserve_order_property() {
    let input = r"
x2 = n2
x1 = n2
x3 = n2
";
    let mut ltx: Ltx = Ltx::load_from_str(input).unwrap();
    let section: &Properties = ltx.root_section();
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
    let section: &Properties = ltx.section("s").unwrap();
    let keys: Vec<&str> = section.iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec!["x2", "xb", "a3"])
  }

  #[test]
  fn duplicate_sections() {
    // https://github.com/zonyitoo/rust-ini/issues/49

    let input = r"
[peer]
foo = a

[peer]
foo = c
";

    let ltx: Result<Ltx, LtxParseError> = Ltx::load_from_str(input);

    assert!(ltx.is_err());
    assert_eq!(
      ltx.unwrap_err().message,
      "Duplicate sections are not allowed"
    );
  }

  #[test]
  fn new_has_empty_general_section() {
    let mut ltx: Ltx = Ltx::new();

    assert!(ltx.root_section().is_empty());
    assert!(ltx.root_section_mut().is_empty());
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
    conf.with_section(section).set(key, value);

    // assert the value is the one expected
    let v = conf.get_from(section, key).unwrap();
    assert_eq!(v, value);

    // update the section/key with a new value
    conf.set_to(section, key.to_string(), new_value.to_string());

    // assert the new value was set
    let v = conf.get_from(section, key).unwrap();
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
    let section: &mut Properties = str.root_section_mut();
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
