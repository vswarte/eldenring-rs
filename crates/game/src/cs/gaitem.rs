use std::fmt::Display;

use crate::pointer::OwnedPtr;

use super::ItemId;
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
pub struct CSGaitemIns {
    vftable: usize,
    pub gaitem_handle: GaitemHandle,
    pub item_id: ItemId,
}

impl CSGaitemIns {
    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_wep(&self) -> Option<&CSWepGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Weapon) => unsafe { std::mem::transmute(self) },
            _ => return None,
        })
    }

    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_wep_mut(&mut self) -> Option<&mut CSWepGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Weapon) => unsafe { std::mem::transmute(self) },
            _ => return None,
        })
    }

    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_gem(&self) -> Option<&CSGemGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Gem) => unsafe { std::mem::transmute(self) },
            _ => return None,
        })
    }

    /// Downcast the CSGaitemIns to the derivant class. Will return None if the requested type
    /// does not match the gaitem ins's type.
    pub fn as_gem_mut(&mut self) -> Option<&mut CSGemGaitemIns> {
        Some(match self.gaitem_handle.category() {
            // Safety: consumers are not allowed to make their own CSGaitemIns and other instances
            // come from the game. The category can reliably be used to do this downcast.
            Ok(GaitemCategory::Gem) => unsafe { std::mem::transmute(self) },
            _ => return None,
        })
    }
}

#[repr(C)]
pub struct CSGaitemImpEntry {
    unindexed_gaitem_handle: u32,
    ref_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaitemHandle(i32);

impl GaitemHandle {
    pub const fn from_parts(selector: i32, category: GaitemCategory) -> Result<Self, ()> {
        Ok(GaitemHandle(
            selector & 0x00FFFFFF | ((category as i32) | -8) << 28,
        ))
    }

    pub const fn is_indexed(self) -> bool {
        self.0 >> 23 & 1 == 1
    }

    pub const fn selector(self) -> i32 {
        self.0 & 0x00ffffff
    }

    pub const fn index(self) -> i32 {
        self.0 & 0xffff
    }

    pub const fn category(self) -> Result<GaitemCategory, ()> {
        GaitemCategory::from_u8(&((self.0 >> 28 & 7) as u8))
    }
}

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
pub struct CSWepGaitemIns {
    pub gaitem_ins: CSGaitemIns,
    /// Item durability mechanic. Unused in ER.
    pub durability: u32,
    _unk14: u32,
    /// Gem slots, used for ashes of war in ER.
    pub gem_slot_table: CSGemSlotTable,
}

#[repr(C)]
pub struct CSGemSlotTable {
    vtable: usize,
    pub gem_slots: [CSGemSlot; 1],
}

#[repr(C)]
pub struct CSGemSlot {
    vtable: usize,
    /// Refers to the actual gem entry in the CSGaitemImp.
    pub gaitem_handle: GaitemHandle,
    unkc: u32,
}

#[repr(C)]
pub struct CSGemGaitemIns {
    pub gaitem_ins: CSGaitemIns,
    gaitem_handle: GaitemHandle,
    /// Item ID of the active gem.
    pub item_id: u32,
}

#[cfg(test)]
mod test {
    use crate::cs::{
        CSGaitemImp, CSGaitemIns, CSGemGaitemIns, CSGemSlot, CSGemSlotTable, CSWepGaitemIns,
    };

    #[test]
    fn proper_sizes() {
        assert_eq!(0x19038, size_of::<CSGaitemImp>());
        assert_eq!(0x10, size_of::<CSGaitemIns>());
        assert_eq!(0x30, size_of::<CSWepGaitemIns>());
        assert_eq!(0x18, size_of::<CSGemSlotTable>());
        assert_eq!(0x10, size_of::<CSGemSlot>());
        assert_eq!(0x18, size_of::<CSGemGaitemIns>());
    }
}
