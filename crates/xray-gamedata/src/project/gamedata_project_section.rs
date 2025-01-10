use crate::GamedataProject;
use xray_ltx::Section;

impl GamedataProject {
  pub fn is_weapon_section(section: &Section) -> bool {
    section.contains_key("class")
      && section.contains_key("weapon_class")
      && (section
        .get("$scheme")
        .is_some_and(|it| it == "$item_weapon")
        || section.contains_key("flame_particles")
        || section.contains_key("flame"))
  }
}
