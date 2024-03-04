pub(crate) mod error;
pub(crate) mod escape_policy;
pub(crate) mod ini;
pub(crate) mod iterator;
pub(crate) mod line_separator;
pub(crate) mod parse_option;
pub(crate) mod parser;
pub(crate) mod properties;
mod property;
pub(crate) mod section_entry;
pub(crate) mod section_setter;
pub(crate) mod write_option;

pub use crate::escape_policy::EscapePolicy;
pub use crate::ini::Ini;
pub use crate::parse_option::ParseOption;
pub use crate::properties::Properties;
pub use crate::write_option::WriteOption;

#[cfg(test)]
mod test {
  use crate::escape_policy::{escape_str, EscapePolicy};
  use crate::ini::Ini;
  use crate::line_separator::{LineSeparator, DEFAULT_LINE_SEPARATOR};
  use crate::parse_option::ParseOption;
  use crate::properties::Properties;
  use crate::write_option::WriteOption;
  use std::env::temp_dir;
  use std::fs::File;
  use std::io::Write;

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

  #[test]
  fn load_from_str_with_empty_general_section() {
    let input = "[sec1]\nkey1=val1\n";
    let opt = Ini::load_from_str(input);
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
    let input = "";
    let opt = Ini::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert!(output.general_section().is_empty());
    assert!(output.general_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  fn load_from_str_with_empty_lines() {
    let input = "\n\n\n";
    let opt = Ini::load_from_str(input);
    assert!(opt.is_ok());

    let mut output = opt.unwrap();
    assert!(output.general_section().is_empty());
    assert!(output.general_section_mut().is_empty());
    assert_eq!(output.len(), 1);
  }

  #[test]
  #[cfg(not(feature = "brackets-in-section-names"))]
  fn load_from_str_with_valid_input() {
    let input = "[sec1]\nkey1=val1\nkey2=377\n[sec2]foo=bar\n";
    let opt = Ini::load_from_str(input);
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
  #[cfg(feature = "brackets-in-section-names")]
  fn load_from_str_with_valid_input() {
    let input = "[sec1]\nkey1=val1\nkey2=377\n[sec2]\nfoo=bar\n";
    let opt = Ini::load_from_str(input);
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
  #[cfg(not(feature = "brackets-in-section-names"))]
  fn load_from_str_without_ending_newline() {
    let input = "[sec1]\nkey1=val1\nkey2=377\n[sec2]foo=bar";
    let opt = Ini::load_from_str(input);
    assert!(opt.is_ok());
  }

  #[test]
  #[cfg(feature = "brackets-in-section-names")]
  fn load_from_str_without_ending_newline() {
    let input = "[sec1]\nkey1=val1\nkey2=377\n[sec2]\nfoo=bar";
    let opt = Ini::load_from_str(input);
    assert!(opt.is_ok());
  }

  #[test]
  fn parse_error_numbers() {
    let invalid_input = "\n\\x";
    let ini = Ini::load_from_str_opt(
      invalid_input,
      ParseOption {
        enabled_escape: true,
        ..Default::default()
      },
    );
    assert!(ini.is_err());

    let err = ini.unwrap_err();
    assert_eq!(err.line, 2);
    assert_eq!(err.col, 3);
  }

  #[test]
  fn parse_comment() {
    let input = "; abcdefghijklmn\n";
    let opt = Ini::load_from_str(input);
    assert!(opt.is_ok());
  }

  #[test]
  fn sharp_comment() {
    let input = "
[section name]
name = hello
# abcdefg
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(ini.get_from(Some("section name"), "name").unwrap(), "hello");
  }

  #[test]
  fn iter() {
    let input = "
[section name]
name = hello # abcdefg
gender = mail ; abdddd
";
    let mut ini = Ini::load_from_str(input).unwrap();

    for _ in &mut ini {}
    for _ in &ini {}
    // for _ in ini {}
  }

  #[test]
  fn colon() {
    let input = "
[section name]
name: hello
gender : mail
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(ini.get_from(Some("section name"), "name").unwrap(), "hello");
    assert_eq!(
      ini.get_from(Some("section name"), "gender").unwrap(),
      "mail"
    );
  }

  #[test]
  fn string() {
    let input = "
[section name]
# This is a comment
Key = \"Value\"
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(ini.get_from(Some("section name"), "Key").unwrap(), "Value");
  }

  #[test]
  fn string_multiline() {
    let input = "
[section name]
# This is a comment
Key = \"Value
Otherline\"
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(
      ini.get_from(Some("section name"), "Key").unwrap(),
      "Value\nOtherline"
    );
  }

  #[test]
  fn string_comment() {
    let input = "
[section name]
# This is a comment
Key = \"Value   # This is not a comment ; at all\"
Stuff = Other
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(
      ini.get_from(Some("section name"), "Key").unwrap(),
      "Value   # This is not a comment ; at all"
    );
  }

  #[test]
  fn string_single() {
    let input = "
[section name]
# This is a comment
Key = 'Value'
Stuff = Other
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(ini.get_from(Some("section name"), "Key").unwrap(), "Value");
  }

  #[test]
  fn string_includes_quote() {
    let input = "
[Test]
Comment[tr]=İnternet'e erişin
Comment[uk]=Доступ до Інтернету
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(
      ini.get_from(Some("Test"), "Comment[tr]").unwrap(),
      "İnternet'e erişin"
    );
  }

  #[test]
  fn string_single_multiline() {
    let input = "
[section name]
# This is a comment
Key = 'Value
Otherline'
Stuff = Other
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(
      ini.get_from(Some("section name"), "Key").unwrap(),
      "Value\nOtherline"
    );
  }

  #[test]
  fn string_single_comment() {
    let input = "
[section name]
# This is a comment
Key = 'Value   # This is not a comment ; at all'
";
    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(
      ini.get_from(Some("section name"), "Key").unwrap(),
      "Value   # This is not a comment ; at all"
    );
  }

  #[test]
  fn load_from_str_with_valid_empty_input() {
    let input = "key1=\nkey2=val2\n";
    let opt = Ini::load_from_str(input);
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
    let input = "key1=val1\r\nkey2=val2\r\n";
    let opt = Ini::load_from_str(input);
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
    let input = "key1=val1\rkey2=val2\r";
    let opt = Ini::load_from_str(input);
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
  #[cfg(not(feature = "brackets-in-section-names"))]
  fn load_from_file_with_bom() {
    let file_name = temp_dir().join("rust_ini_load_from_file_with_bom");

    let file_content = b"\xEF\xBB\xBF[Test]Key=Value\n";

    {
      let mut file = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ini = Ini::load_from_file(&file_name).unwrap();
    assert_eq!(ini.get_from(Some("Test"), "Key"), Some("Value"));
  }

  #[test]
  #[cfg(feature = "brackets-in-section-names")]
  fn load_from_file_with_bom() {
    let file_name = temp_dir().join("rust_ini_load_from_file_with_bom");

    let file_content = b"\xEF\xBB\xBF[Test]\nKey=Value\n";

    {
      let mut file = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ini = Ini::load_from_file(&file_name).unwrap();
    assert_eq!(ini.get_from(Some("Test"), "Key"), Some("Value"));
  }

  #[test]
  #[cfg(not(feature = "brackets-in-section-names"))]
  fn load_from_file_without_bom() {
    let file_name = temp_dir().join("rust_ini_load_from_file_without_bom");

    let file_content = b"[Test]Key=Value\n";

    {
      let mut file = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ini = Ini::load_from_file(&file_name).unwrap();
    assert_eq!(ini.get_from(Some("Test"), "Key"), Some("Value"));
  }

  #[test]
  #[cfg(feature = "brackets-in-section-names")]
  fn load_from_file_without_bom() {
    let file_name = temp_dir().join("rust_ini_load_from_file_without_bom");

    let file_content = b"[Test]\nKey=Value\n";

    {
      let mut file = File::create(&file_name).expect("create");
      file.write_all(file_content).expect("write");
    }

    let ini = Ini::load_from_file(&file_name).unwrap();
    assert_eq!(ini.get_from(Some("Test"), "Key"), Some("Value"));
  }

  #[test]
  fn get_with_non_static_key() {
    let input = "key1=val1\nkey2=val2\n";
    let opt = Ini::load_from_str(input).unwrap();

    let sec1 = opt.section(None::<String>).unwrap();

    let key = "key1".to_owned();
    sec1.get(&key).unwrap();
  }

  #[test]
  fn load_from_str_noescape() {
    let input = "path=C:\\Windows\\Some\\Folder\\";
    let opt = Ini::load_from_str_noescape(input);
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
    let input = "
[Section]
A=\"quote\" arg0
B=b";

    let opt = Ini::load_from_str(input).unwrap();
    let sec = opt.section(Some("Section")).unwrap();
    assert_eq!(&sec["A"], "quote arg0");
    assert_eq!(&sec["B"], "b");
  }

  #[test]
  fn partial_quoting_single() {
    let input = "
[Section]
A='quote' arg0
B=b";

    let opt = Ini::load_from_str(input).unwrap();
    let sec = opt.section(Some("Section")).unwrap();
    assert_eq!(&sec["A"], "quote arg0");
    assert_eq!(&sec["B"], "b");
  }

  #[test]
  fn parse_without_quote() {
    let input = "
[Desktop Entry]
Exec = \"/path/to/exe with space\" arg
";

    let opt = Ini::load_from_str_opt(
      input,
      ParseOption {
        enabled_quote: false,
        ..ParseOption::default()
      },
    )
    .unwrap();
    let sec = opt.section(Some("Desktop Entry")).unwrap();
    assert_eq!(&sec["Exec"], "\"/path/to/exe with space\" arg");
  }

  #[test]
  fn case_insensitive() {
    let input = "
[SecTION]
KeY=value
";

    let ini = Ini::load_from_str(input).unwrap();
    let section = ini.section(Some("section")).unwrap();
    let val = section.get("key").unwrap();
    assert_eq!("value", val);
  }

  #[test]
  fn preserve_order_section() {
    let input = r"
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

    let data = Ini::load_from_str(input).unwrap();
    let keys: Vec<Option<&str>> = data.iter().map(|(k, _)| k).collect();

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
    let data = Ini::load_from_str(input).unwrap();
    let section = data.general_section();
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
    let data = Ini::load_from_str(input).unwrap();
    let section = data.section(Some("s")).unwrap();
    let keys: Vec<&str> = section.iter().map(|(k, _)| k).collect();
    assert_eq!(keys, vec!["x2", "xb", "a3"])
  }

  #[test]
  fn preserve_order_write() {
    let input = r"
x2 = n2
x1 = n2
x3 = n2
[s]
x2 = n2
xb = n2
a3 = n3
";
    let data = Ini::load_from_str(input).unwrap();
    let mut buf = vec![];
    data.write_to(&mut buf).unwrap();
    let new_data = Ini::load_from_str(&String::from_utf8(buf).unwrap()).unwrap();

    let sec0 = new_data.general_section();
    let keys0: Vec<&str> = sec0.iter().map(|(k, _)| k).collect();
    assert_eq!(keys0, vec!["x2", "x1", "x3"]);

    let sec1 = new_data.section(Some("s")).unwrap();
    let keys1: Vec<&str> = sec1.iter().map(|(k, _)| k).collect();
    assert_eq!(keys1, vec!["x2", "xb", "a3"]);
  }

  #[test]
  fn write_new() {
    use std::str;

    let ini = Ini::new();

    let opt = WriteOption {
      line_separator: LineSeparator::CR,
      ..Default::default()
    };
    let mut buf = Vec::new();
    ini.write_to_opt(&mut buf, opt).unwrap();

    assert_eq!("", str::from_utf8(&buf).unwrap());
  }

  #[test]
  fn write_line_separator() {
    use std::str;

    let mut ini = Ini::new();
    ini
      .with_section(Some("Section1"))
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section(Some("Section2"))
      .set("Key1", "Value")
      .set("Key2", "Value");

    {
      let mut buf = Vec::new();
      ini
        .write_to_opt(
          &mut buf,
          WriteOption {
            line_separator: LineSeparator::CR,
            ..Default::default()
          },
        )
        .unwrap();

      assert_eq!(
        "[Section1]\nKey1=Value\nKey2=Value\n\n[Section2]\nKey1=Value\nKey2=Value\n",
        str::from_utf8(&buf).unwrap()
      );
    }

    {
      let mut buf = Vec::new();
      ini
        .write_to_opt(
          &mut buf,
          WriteOption {
            line_separator: LineSeparator::CRLF,
            ..Default::default()
          },
        )
        .unwrap();

      assert_eq!(
        "[Section1]\r\nKey1=Value\r\nKey2=Value\r\n\r\n[Section2]\r\nKey1=Value\r\nKey2=Value\r\n",
        str::from_utf8(&buf).unwrap()
      );
    }

    {
      let mut buf = Vec::new();
      ini
        .write_to_opt(
          &mut buf,
          WriteOption {
            line_separator: LineSeparator::SystemDefault,
            ..Default::default()
          },
        )
        .unwrap();

      if cfg!(windows) {
        assert_eq!(
          "[Section1]\r\nKey1=Value\r\nKey2=Value\r\n\r\n[Section2]\r\nKey1=Value\r\nKey2=Value\r\n",
          str::from_utf8(&buf).unwrap()
        );
      } else {
        assert_eq!(
          "[Section1]\nKey1=Value\nKey2=Value\n\n[Section2]\nKey1=Value\nKey2=Value\n",
          str::from_utf8(&buf).unwrap()
        );
      }
    }
  }

  #[test]
  fn write_kv_separator() {
    use std::str;

    let mut ini = Ini::new();
    ini
      .with_section(None::<String>)
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section(Some("Section1"))
      .set("Key1", "Value")
      .set("Key2", "Value");
    ini
      .with_section(Some("Section2"))
      .set("Key1", "Value")
      .set("Key2", "Value");

    let mut buf = Vec::new();
    ini
      .write_to_opt(
        &mut buf,
        WriteOption {
          kv_separator: " = ",
          ..Default::default()
        },
      )
      .unwrap();

    // Test different line endings in Windows and Unix
    if cfg!(windows) {
      assert_eq!(
        "Key1 = Value\r\nKey2 = Value\r\n\r\n[Section1]\r\nKey1 = Value\r\nKey2 = Value\r\n\r\n[Section2]\r\nKey1 = Value\r\nKey2 = Value\r\n",
        str::from_utf8(&buf).unwrap()
      );
    } else {
      assert_eq!(
        "Key1 = Value\nKey2 = Value\n\n[Section1]\nKey1 = Value\nKey2 = Value\n\n[Section2]\nKey1 = Value\nKey2 = Value\n",
        str::from_utf8(&buf).unwrap()
      );
    }
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

    let ini = Ini::load_from_str(input).unwrap();
    assert_eq!(3, ini.section_all(Some("Peer")).count());

    let mut iter = ini.iter();
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
    let mut ini = Ini::new();

    assert!(ini.general_section().is_empty());
    assert!(ini.general_section_mut().is_empty());
    assert_eq!(ini.len(), 1);
  }

  #[test]
  fn fix_issue63() {
    let section = "PHP";
    let key = "engine";
    let value = "On";
    let new_value = "Off";

    // create a new configuration
    let mut conf = Ini::new();
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
  fn fix_issue64() {
    let input = format!("some-key=åäö{}", DEFAULT_LINE_SEPARATOR);

    let conf = Ini::load_from_str(&input).unwrap();

    let mut output = Vec::new();
    conf
      .write_to_policy(&mut output, EscapePolicy::Basics)
      .unwrap();

    assert_eq!(input, String::from_utf8(output).unwrap());
  }

  #[test]
  fn invalid_codepoint() {
    use std::io::Cursor;

    let d = vec![
      10, 8, 68, 8, 61, 10, 126, 126, 61, 49, 10, 62, 8, 8, 61, 10, 91, 93, 93, 36, 91, 61, 10, 75,
      91, 10, 10, 10, 61, 92, 120, 68, 70, 70, 70, 70, 70, 126, 61, 10, 0, 0, 61, 10, 38, 46, 49,
      61, 0, 39, 0, 0, 46, 92, 120, 46, 36, 91, 91, 1, 0, 0, 16, 0, 0, 0, 0, 0, 0,
    ];
    let mut file = Cursor::new(d);
    assert!(Ini::read_from(&mut file).is_err());
  }

  #[test]
  #[cfg(feature = "brackets-in-section-names")]
  fn fix_issue84() {
    let input = "
[[*]]
a = b
c = d
";
    let ini = Ini::load_from_str(input).unwrap();
    let sect = ini.section(Some("[*]"));
    assert!(sect.is_some());
    assert!(sect.unwrap().contains_key("a"));
    assert!(sect.unwrap().contains_key("c"));
  }

  #[test]
  #[cfg(feature = "brackets-in-section-names")]
  fn fix_issue84_brackets_inside() {
    let input = "
[a[b]c]
a = b
c = d
";
    let ini = Ini::load_from_str(input).unwrap();
    let sect = ini.section(Some("a[b]c"));
    assert!(sect.is_some());
    assert!(sect.unwrap().contains_key("a"));
    assert!(sect.unwrap().contains_key("c"));
  }

  #[test]
  #[cfg(feature = "brackets-in-section-names")]
  fn fix_issue84_whitespaces_after_bracket() {
    let input = "
[[*]]\t\t
a = b
c = d
";
    let ini = Ini::load_from_str(input).unwrap();
    let sect = ini.section(Some("[*]"));
    assert!(sect.is_some());
    assert!(sect.unwrap().contains_key("a"));
    assert!(sect.unwrap().contains_key("c"));
  }

  #[test]
  #[cfg(feature = "brackets-in-section-names")]
  fn fix_issue84_not_whitespaces_after_bracket() {
    let input = "
[[*]]xx
a = b
c = d
";
    let ini = Ini::load_from_str(input);
    assert!(ini.is_err());
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
    let test_unicode = r"é£∳字✨";
    let test_emoji = r"🐱😉";
    let test_cjk = r"𠈌𠕇";
    let test_high_points = "\u{10ABCD}\u{10FFFF}";

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
    let input = r"
x2 = nc
x1 = na
x3 = nb
";
    let mut data = Ini::load_from_str(input).unwrap();
    let section = data.general_section_mut();
    section.iter_mut().enumerate().for_each(|(i, (_, v))| {
      v.push_str(&i.to_string());
    });
    let props: Vec<_> = section.iter().collect();
    assert_eq!(props, vec![("x2", "nc0"), ("x1", "na1"), ("x3", "nb2")]);
  }

  #[test]
  fn preserve_order_properties_into_iter() {
    let input = r"
x2 = nc
x1 = na
x3 = nb
";
    let data = Ini::load_from_str(input).unwrap();
    let (_, section) = data.into_iter().next().unwrap();
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
