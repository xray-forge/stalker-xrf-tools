use crate::INVENTORY_ICON_GRID_SQUARE_BASE;
use image::{ImageBuffer, Rgba, RgbaImage};
use serde::Serialize;
use std::cmp::max;
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventorySpriteDescriptor {
  pub section: String,
  pub custom_icon: Option<String>,
  // X/Y/W/H are not absolute pixel units, just inventory boxes.
  pub x: u32,
  pub y: u32,
  pub w: u32,
  pub h: u32,
}

impl InventorySpriteDescriptor {
  pub fn new_list_from_ltx(ltx: &Ltx) -> Vec<Self> {
    let mut inventory_sections: Vec<Self> = Vec::new();

    for (section_name, section) in &ltx.sections {
      if let Some(inventory_section) = Self::new_optional_from_section(section_name, section) {
        inventory_sections.push(inventory_section);
      }
    }

    inventory_sections
  }

  /// Describe the inventory icon of a section, if it declares one.
  ///
  /// A section opts in with `$inventory_icon = true`. The grid fields alone are not enough: they are
  /// also declared by abstract base sections purely so children can inherit them, and section lookups
  /// see inherited fields, so keying off them would pack sections that have no icon of their own.
  pub fn new_optional_from_section<T>(section_name: T, section: &Section) -> Option<Self>
  where
    T: Into<String>,
  {
    if !section
      .get("$inventory_icon")
      .and_then(|value| value.trim().parse::<bool>().ok())
      .unwrap_or(false)
    {
      return None;
    }

    let x: u32 = section
      .get("inv_grid_x")?
      .parse::<u32>()
      .unwrap_or(u32::MAX);
    let y: u32 = section
      .get("inv_grid_y")?
      .parse::<u32>()
      .unwrap_or(u32::MAX);
    let w: u32 = section
      .get("inv_grid_width")?
      .parse::<u32>()
      .unwrap_or(u32::MAX);
    let h: u32 = section
      .get("inv_grid_height")?
      .parse::<u32>()
      .unwrap_or(u32::MAX);

    if x == u32::MAX || y == u32::MAX || w == u32::MAX || w == 0 || h == u32::MAX || h == 0 {
      None
    } else {
      Some(Self {
        section: section_name.into(),
        custom_icon: section
          .get("$inventory_icon_path")
          .map(|value| value.into()),
        x,
        y,
        w,
        h,
      })
    }
  }
}

impl InventorySpriteDescriptor {
  pub fn get_boundaries(&self) -> (u32, u32, u32, u32) {
    (
      self.x * INVENTORY_ICON_GRID_SQUARE_BASE,
      self.y * INVENTORY_ICON_GRID_SQUARE_BASE,
      self.w * INVENTORY_ICON_GRID_SQUARE_BASE,
      self.h * INVENTORY_ICON_GRID_SQUARE_BASE,
    )
  }
}

impl InventorySpriteDescriptor {
  /// Prepare combined equipment image base with suitable base size.
  pub fn create_equipment_sprite_base_for_ltx(
    ltx: &Ltx,
  ) -> XRayResult<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let (max_width, max_height) = Self::get_equipment_sprite_boundaries_from_ltx(ltx);

    if max_width > 32 * 1024 || max_height > 32 * 1024 {
      Err(XRayError::new_texture_processing_error(format!(
        "Trying to create too large resulting dds file over 32k*32k ({max_width}x{max_height}), it is not supported",
      )))
    } else {
      Ok(RgbaImage::new(max_width, max_height))
    }
  }

  pub fn get_equipment_sprite_boundaries_from_ltx(ltx: &Ltx) -> (u32, u32) {
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;

    for (section_name, section) in &ltx.sections {
      if let Some(sprite) = Self::new_optional_from_section(section_name, section) {
        max_width = max(
          (sprite.x + sprite.w) * INVENTORY_ICON_GRID_SQUARE_BASE,
          max_width,
        );
        max_height = max(
          (sprite.y + sprite.h) * INVENTORY_ICON_GRID_SQUARE_BASE,
          max_height,
        );
      }
    }

    // Make sure resulting sprites are multiples of 4 for width and height
    max_width = max_width + (4 - max_width % 4);
    max_height = max_height + (4 - max_height % 4);

    (max_width, max_height)
  }
}

#[cfg(test)]
mod tests {
  use super::InventorySpriteDescriptor;
  use xray_ltx::Ltx;

  fn descriptor_for(ltx: &str, section: &str) -> Option<InventorySpriteDescriptor> {
    let ltx: Ltx = Ltx::read_from_str(ltx).expect("test LTX is valid");

    InventorySpriteDescriptor::new_optional_from_section(section, &ltx[section])
  }

  #[test]
  fn describes_sections_that_opt_in() {
    let descriptor: InventorySpriteDescriptor = descriptor_for(
      "[wpn_ak74]\n\
       $inventory_icon = true\n\
       inv_grid_x = 25\n\
       inv_grid_y = 4\n\
       inv_grid_width = 5\n\
       inv_grid_height = 2\n",
      "wpn_ak74",
    )
    .expect("expect an opted in section to describe an icon");

    assert_eq!(descriptor.x, 25);
    assert_eq!(descriptor.y, 4);
    assert_eq!(descriptor.w, 5);
    assert_eq!(descriptor.h, 2);
  }

  #[test]
  fn ignores_grid_fields_without_opt_in() {
    assert!(
      descriptor_for(
        "[some_section]\n\
         inv_grid_x = 25\n\
         inv_grid_y = 4\n\
         inv_grid_width = 5\n\
         inv_grid_height = 2\n",
        "some_section",
      )
      .is_none(),
      "Expect grid fields alone not to declare an icon, so adding them cannot pack an asset"
    );
  }

  #[test]
  fn ignores_sections_that_opt_out() {
    assert!(
      descriptor_for(
        "[af_base]\n\
         $inventory_icon = false\n\
         inv_grid_x = 0\n\
         inv_grid_y = 0\n\
         inv_grid_width = 1\n\
         inv_grid_height = 1\n",
        "af_base",
      )
      .is_none(),
      "Expect an explicit opt out to be honoured even when the section is grid complete"
    );
  }

  #[test]
  fn requires_grid_fields_even_when_opted_in() {
    assert!(
      descriptor_for(
        "[af_base]\n\
         $inventory_icon = true\n\
         inv_grid_width = 1\n\
         inv_grid_height = 1\n",
        "af_base",
      )
      .is_none(),
      "Expect a section without grid position not to describe an icon"
    );
  }
}
