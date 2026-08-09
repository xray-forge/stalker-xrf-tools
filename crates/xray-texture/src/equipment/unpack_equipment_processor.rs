use image::GenericImageView;
use image_dds::Mipmaps;
use xray_error::XRayResult;

use crate::constants::DDS_EXTENSION;
use crate::data::inventory_sprite_descriptor::InventorySpriteDescriptor;
use crate::{UnpackEquipmentOptions, save_image_as_ui_dds};

pub struct UnpackEquipmentProcessor {}

impl UnpackEquipmentProcessor {
  pub fn unpack_sprites(options: UnpackEquipmentOptions) -> XRayResult {
    let mut count: u32 = 0;

    for (section_name, section) in &options.ltx.sections {
      if let Some(sprite) =
        InventorySpriteDescriptor::new_optional_from_section(section_name, section)
        && Self::unpack_sprite(&options, &sprite)?
      {
        count += 1;
      }
    }

    xray_output::info!(options.output, "Unpacked {count} icons");

    Ok(())
  }

  pub fn unpack_sprite(
    options: &UnpackEquipmentOptions,
    sprite: &InventorySpriteDescriptor,
  ) -> XRayResult<bool> {
    let (x, y, w, h) = sprite.get_boundaries();

    xray_output::verbose!(
      options.output,
      "Unpacking icon: '{}' x:{}({x}), y:{}({y}), w:{}({w}), h:{}({h})",
      sprite.section,
      sprite.x,
      sprite.y,
      sprite.w,
      sprite.h,
    );

    // todo: Respect custom icon path from LTX file here (sprite.custom_icon).

    if x + w > options.source.width() || y + h > options.source.height() {
      xray_output::warning!(
        options.output,
        "Skip for possible section: '{}' - icon is out of source file bonds",
        sprite.section
      );

      Ok(false)
    } else {
      // Unpacked icons are packing input read at their base level, so a mip chain would only cost
      // space.
      save_image_as_ui_dds(
        &options
          .output_path
          .join(format!("{}.{}", sprite.section, DDS_EXTENSION)),
        &options.source.view(x, y, w, h).to_image(),
        options.dds_compression_format,
        Mipmaps::Disabled,
      )?;

      Ok(true)
    }
  }
}
