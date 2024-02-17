#![allow(dead_code)]

pub const CFS_COMPRESS_MARK: u32 = 1u32 << 31u32;

pub const FLAG_SPAWN_ENABLED: u16 = 1;
pub const FLAG_SPAWN_ON_SURGE_ONLY: u16 = 2;
pub const FLAG_SPAWN_SINGLE_ITEM_ONLY: u16 = 4;
pub const FLAG_SPAWN_IF_DESTROYED_ONLY: u16 = 8;
pub const FLAG_SPAWN_INFINITE_COUNT: u16 = 16;
pub const FLAG_SPAWN_DESTROY_ON_SPAWN: u16 = 32;
