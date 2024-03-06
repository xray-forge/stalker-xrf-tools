use crate::Properties;
use fxhash::FxBuildHasher;
use indexmap::IndexMap;

pub type PropertiesData = IndexMap<String, String, FxBuildHasher>;

pub type LtxSections = IndexMap<String, Properties, FxBuildHasher>;

pub type LtxIncludes = Vec<String>;
