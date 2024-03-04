/// Internal storage of section's key
pub type SectionKey = Option<unicase::UniCase<String>>;
/// Internal storage of property's key
pub type PropertyKey = unicase::UniCase<String>;

macro_rules! property_get_key {
  ($s:expr) => {
    &unicase::UniCase::from($s)
  };
}

macro_rules! property_insert_key {
  ($s:expr) => {
    unicase::UniCase::from($s)
  };
}

macro_rules! section_key {
  ($s:expr) => {
    $s.map(|s| unicase::UniCase::from(s.into()))
  };
}

pub(crate) use property_get_key;
pub(crate) use property_insert_key;
pub(crate) use section_key;
