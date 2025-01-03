use crate::pointer::OwnedPtr;

use super::{ItemCategory, ItemId};

#[repr(C)]
pub struct CSGaitemIns {
    vftable: usize,
    pub gaitem_handle: u32,
    pub item_id: i32,
}

#[repr(C)]
pub struct CSGaitemImpEntry {
    pub unindexed_gaitem_handle: u32,
    pub ref_count: u32,
}

#[repr(C)]
pub struct CSGaitemImp {
    vftable: usize,
    pub gaitem_instances: [OwnedPtr<CSGaitemImp>; 5120],
    pub gaitem_entries: [CSGaitemImpEntry; 5120],
    pub indexes: [u32; 5120],
    pub write_idx: u32,
    pub read_idx: u32,
    rand_xorshift: [u8; 0x18],
    unk23028: [u8; 8],
    pub is_being_serialized: bool,
    unk23038: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaitemHandle(pub u32);

impl GaitemHandle {
    /// converts gaitem handle to selector
    /// should be the same as param id of the item this handle points to
    pub const fn to_selector(self) -> u32 {
        self.0 & 0x00ffffff
    }

    pub fn from_parts(selector: u32, category: ItemCategory) -> Self {
        GaitemHandle(selector & 0x00FFFFFF | (category as u32 | 0xfffffff8) << 28)
    }

    /// returns true if the gaitem handle has index
    /// and therefore is refcounted in CSGaitemImp
    pub const fn is_indexed(self) -> bool {
        self.0 >> 23 & 1 == 1
    }

    /// returns the index of the gaitem handle in CSGaitemImp
    pub const fn index(self) -> u32 {
        self.0 & 0xffff
    }

    pub fn category(self) -> ItemCategory {
        ItemCategory::from((self.0 >> 28 & 7) as u8)
    }
}
