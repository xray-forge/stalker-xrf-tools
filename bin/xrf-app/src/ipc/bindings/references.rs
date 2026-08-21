//! Reading generated TypeScript back, to learn which declarations a block of it depends on.
//!
//! Specta renders each module in isolation, so the imports tying the modules together have to be recovered
//! from the rendered text rather than from the type graph.

use std::collections::{BTreeMap, BTreeSet};

use crate::ipc::bindings::constants::{BINDINGS_ROOT, TYPES_DIRECTORY};

/// Generated source with doc comments and string literals blanked out.
///
/// A type reference cannot be told from surrounding text by shape alone: class ids and pack modes render as
/// PascalCase string unions, and doc comment prose is capitalised. Removing both leaves only real code.
fn without_comments_and_strings(source: &str) -> String {
  let mut stripped: String = String::with_capacity(source.len());
  let mut characters = source.chars().peekable();

  while let Some(character) = characters.next() {
    match character {
      '/' if characters.peek() == Some(&'/') => {
        for skipped in characters.by_ref() {
          if skipped == '\n' {
            stripped.push('\n');
            break;
          }
        }
      }
      '/' if characters.peek() == Some(&'*') => {
        characters.next();

        let mut previous: char = '\0';

        for skipped in characters.by_ref() {
          if previous == '*' && skipped == '/' {
            break;
          }

          previous = skipped;
        }

        stripped.push(' ');
      }
      '"' | '\'' | '`' => {
        for skipped in characters.by_ref() {
          if skipped == character {
            break;
          }
        }

        stripped.push(' ');
      }
      _ => stripped.push(character),
    }
  }

  stripped
}

/// Names of generated types that `source` references and `owner` does not itself declare.
pub(super) fn referenced_types<'a>(
  source: &str,
  owners: &'a BTreeMap<String, String>,
  owner: &str,
) -> BTreeSet<&'a str> {
  let source: String = without_comments_and_strings(source);
  let bytes: &[u8] = source.as_bytes();
  let mut referenced: BTreeSet<&str> = BTreeSet::new();
  let mut index: usize = 0;

  while index < bytes.len() {
    if !bytes[index].is_ascii_alphabetic() && bytes[index] != b'_' {
      index += 1;
      continue;
    }

    let start: usize = index;

    while index < bytes.len() && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') {
      index += 1;
    }

    // A property name is never a type reference; `Shape` renders its variants as `{ Sphere: .. }`.
    let trailing: &str = source[index..].trim_start();

    if trailing.starts_with(':') || trailing.starts_with("?:") {
      continue;
    }

    if let Some((name, module)) = owners.get_key_value(&source[start..index])
      && module != owner
    {
      referenced.insert(name.as_str());
    }
  }

  referenced
}

/// Import statements pulling every referenced type from the module that declares it.
pub(super) fn render_imports(referenced: &BTreeSet<&str>, owners: &BTreeMap<String, String>) -> String {
  let mut grouped: BTreeMap<&str, Vec<&str>> = BTreeMap::new();

  for name in referenced {
    grouped
      .entry(
        owners
          .get(*name)
          .unwrap_or_else(|| panic!("`{name}` is referenced but has no owning module"))
          .as_str(),
      )
      .or_default()
      .push(name);
  }

  grouped
    .into_iter()
    .map(|(module, names)| {
      format!(
        "import {{ {} }} from \"{BINDINGS_ROOT}/{TYPES_DIRECTORY}/{module}\";\n",
        names.join(", ")
      )
    })
    .collect()
}
