pub(crate) mod data;
pub(crate) mod description;
pub(crate) mod equipment;
pub(crate) mod error;
pub(crate) mod types;
pub(crate) mod utils;

pub use crate::description::pack_description_options::PackDescriptionOptions;
pub use crate::description::pack_description_processor::PackDescriptionProcessor;
pub use crate::description::unpack_description_processor::UnpackDescriptionProcessor;

pub use crate::error::texture_error::TextureError;

pub use crate::equipment::config::get_ltx_inventory_descriptors;
pub use crate::equipment::config_inventory_section_descriptor::ConfigInventorySectionDescriptor;
pub use crate::equipment::convert_constants::INVENTORY_ICON_GRID_SQUARE_BASE;
pub use crate::equipment::convert_constants::SECTION_TYPE_INVENTORY_ICON;
pub use crate::equipment::pack::pack_equipment_icon;
pub use crate::equipment::pack::pack_equipment_icons_by_ltx;
pub use crate::equipment::pack_options::PackEquipmentOptions;
pub use crate::equipment::pack_result::PackEquipmentResult;
pub use crate::equipment::unpack::unpack_equipment_icon;
pub use crate::equipment::unpack::unpack_equipment_icons_by_ltx;

pub use crate::equipment::unpack_options::UnpackEquipmentOptions;
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
