use crate::error::texture_processing_error::TextureProcessingError;
use crate::{TextureResult, INVENTORY_ICON_GRID_SQUARE_BASE};
use image::{ImageBuffer, Rgba, RgbaImage};
use serde::Serialize;
use std::cmp::max;
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

  pub fn new_optional_from_section<T>(section_name: T, section: &Section) -> Option<Self>
  where
    T: Into<String>,
  {
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
  ) -> TextureResult<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let (max_width, max_height) = Self::get_equipment_sprite_boundaries_from_ltx(ltx);

    if max_width > 32 * 1024 || max_height > 32 * 1024 {
      Err(TextureProcessingError::new_texture_error(format!(
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
