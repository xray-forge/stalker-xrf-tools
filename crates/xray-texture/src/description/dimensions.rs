use crate::description::file_description::FileDescription;
use crate::description::texture_description::TextureDescription;
use std::cmp::max;

pub fn get_xml_description_sprite_max_dimension(file_description: &FileDescription) -> (u32, u32) {
  let mut max_width: u32 = 0;
  let mut max_height: u32 = 0;

  for texture in &file_description.textures {
    let (width, height) = get_texture_description_sprite_max_dimension(texture);

    max_width = max(width, max_width);
    max_height = max(height, max_height);
  }

  // Make sure resulting sprites are multiples of 4 for width and height
  max_width = max_width + (4 - max_width % 4);
  max_height = max_height + (4 - max_height % 4);

  assert_eq!(
    max_width % 4,
    0,
    "DirectX compression requires texture width to be multiple of 4"
  );
  assert_eq!(
    max_height % 4,
    0,
    "DirectX compression requires texture height to be multiple of 4"
  );

  (max_width, max_height)
}

pub fn get_texture_description_sprite_max_dimension(
  texture_description: &TextureDescription,
) -> (u32, u32) {
  (
    texture_description.x + texture_description.w,
    texture_description.y + texture_description.h,
  )
}
