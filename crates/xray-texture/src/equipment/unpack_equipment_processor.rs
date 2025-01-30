use crate::data::inventory_sprite_descriptor::InventorySpriteDescriptor;
use crate::{save_image_as_ui_dds, UnpackEquipmentOptions};
use image::GenericImageView;
use xray_error::XRayResult;

pub struct UnpackEquipmentProcessor {}

impl UnpackEquipmentProcessor {
  pub fn unpack_sprites(options: UnpackEquipmentOptions) -> XRayResult {
    let mut count: u32 = 0;

    for (section_name, section) in &options.ltx.sections {
      if let Some(sprite) =
        InventorySpriteDescriptor::new_optional_from_section(section_name, section)
      {
        if Self::unpack_sprite(&options, &sprite)? {
          count += 1;
        }
      }
    }

    println!("Unpacked {count} icons");

    Ok(())
  }

  pub fn unpack_sprite(
    options: &UnpackEquipmentOptions,
    sprite: &InventorySpriteDescriptor,
  ) -> XRayResult<bool> {
    let (x, y, w, h) = sprite.get_boundaries();

    if options.is_verbose {
      println!(
        "Unpacking icon: '{}' x:{}({x}), y:{}({y}), w:{}({w}), h:{}({h})",
        sprite.section, sprite.x, sprite.y, sprite.w, sprite.h,
      );
    }

    // todo: Respect custom icon path from LTX file here (sprite.custom_icon).

    if x + w > options.source.width() || y + h > options.source.height() {
      println!(
        "Skip for possible section: '{}' - icon is out of source file bonds",
        sprite.section
      );

      Ok(false)
    } else {
      save_image_as_ui_dds(
        &options.output.join(format!("{}.dds", sprite.section)),
        &options.source.view(x, y, w, h).to_image(),
        options.dds_compression_format,
      )?;

      Ok(true)
    }
  }
}
