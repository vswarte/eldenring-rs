use std::fmt::Display;

use crate::pointer::OwnedPtr;

use super::ItemId;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GaitemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    Goods = 3,
    Gem = 4,
}

impl GaitemCategory {
    pub const fn from_u8(val: &u8) -> Result<Self, ()> {
        Ok(match val {
            0 => GaitemCategory::Weapon,
            1 => GaitemCategory::Protector,
            2 => GaitemCategory::Accessory,
            3 => GaitemCategory::Goods,
            4 => GaitemCategory::Gem,
            _ => return Err(()),
        })
    }
}

#[repr(C)]
pub struct CSGaitemIns {
    vftable: usize,
    pub gaitem_handle: GaitemHandle,
    pub item_id: ItemId,
}

#[repr(C)]
pub struct CSGaitemImpEntry {
    pub unindexed_gaitem_handle: u32,
    pub ref_count: u32,
}

#[repr(C)]
pub struct CSGaitemImp {
    vftable: usize,
    pub gaitem_instances: [OwnedPtr<CSGaitemIns>; 5120],
    pub gaitem_entries: [CSGaitemImpEntry; 5120],
    pub indexes: [u32; 5120],
    pub write_index: u32,
    pub read_index: u32,
    rand_xorshift: [u8; 0x18],
    unk23028: [u8; 8],
    pub is_being_serialized: bool,
    unk23038: [u8; 7],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaitemHandle(u32);

impl GaitemHandle {
    pub const fn from_parts(selector: u32, category: GaitemCategory) -> Result<Self, ()> {
        Ok(GaitemHandle(
            selector & 0x00FFFFFF | ((category as u32) | 0xfffffff8) << 28,
        ))
    }

    pub const fn is_indexed(self) -> bool {
        self.0 >> 23 & 1 == 1
    }

    pub const fn selector(self) -> u32 {
        self.0 & 0x00ffffff
    }

    pub const fn index(self) -> u32 {
        self.0 & 0xffff
    }

    pub const fn category(self) -> Result<GaitemCategory, ()> {
        GaitemCategory::from_u8(&((self.0 >> 28 & 7) as u8))
    }
}

impl Display for GaitemHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Gaitem Handle: {:0>8}, Category: {:?}, Selector: {:0>6}, Indexed: {:?}",
            self.0,
            self.category(),
            self.selector(),
            self.is_indexed()
        )
    }
}
