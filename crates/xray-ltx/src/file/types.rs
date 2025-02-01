use crate::file::file_section::section::Section;
use crate::scheme::field_scheme::LtxFieldScheme;
use crate::scheme::section_scheme::LtxSectionScheme;
use fxhash::FxBuildHasher;
use indexmap::IndexMap;

pub type SectionData = IndexMap<String, String, FxBuildHasher>;

pub type LtxIncluded = Vec<String>;

pub type LtxSections = IndexMap<String, Section, FxBuildHasher>;

pub type LtxSectionSchemes = IndexMap<String, LtxSectionScheme, FxBuildHasher>;

pub type LtxSectionFieldSchemes = IndexMap<String, LtxFieldScheme, FxBuildHasher>;
