pub(crate) mod data;
pub(crate) mod description;
pub(crate) mod equipment;
pub(crate) mod error;
pub(crate) mod types;
pub(crate) mod utils;

pub use crate::data::inventory_sprite_descriptor::InventorySpriteDescriptor;
pub use crate::data::texture_file_descriptor::TextureFileDescriptor;
pub use crate::data::texture_sprite_descriptor::TextureSpriteDescriptor;

pub use crate::description::pack_description_options::PackDescriptionOptions;
pub use crate::description::pack_description_processor::PackDescriptionProcessor;
pub use crate::description::unpack_description_processor::UnpackDescriptionProcessor;

pub use crate::error::texture_error::TextureError;

pub use crate::equipment::equipment_constants::INVENTORY_ICON_GRID_SQUARE_BASE;
pub use crate::equipment::pack_equipment_options::PackEquipmentOptions;
pub use crate::equipment::pack_equipment_processor::PackEquipmentProcessor;
pub use crate::equipment::pack_equipment_result::PackEquipmentResult;
pub use crate::equipment::unpack_equipment_processor::UnpackEquipmentProcessor;

pub use crate::equipment::unpack_equipment_options::UnpackEquipmentOptions;
pub use utils::images::dds_to_image;
pub use utils::images::open_dds_as_png;
pub use utils::images::read_dds_by_path;
pub use utils::images::rescale_image_to_bounds;
pub use utils::images::save_image_as_ui_dds;
pub use utils::images::save_image_as_ui_png;

pub use image::RgbaImage;
pub use image_dds::image::GenericImage;
pub use image_dds::image::GenericImageView;
pub use image_dds::ImageFormat;

pub use types::TextureResult;
