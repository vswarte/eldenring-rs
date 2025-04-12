use std::fmt::Display;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ItemId(i32);

#[repr(i8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    Goods = 4,
    Gem = 8,
    None = -1,
}

impl ItemCategory {
    pub const fn from_i8(value: i8) -> Result<Self, ()> {
        Ok(match value {
            0 => ItemCategory::Weapon,
            1 => ItemCategory::Protector,
            2 => ItemCategory::Accessory,
            4 => ItemCategory::Goods,
            8 => ItemCategory::Gem,
            15 | -1 => ItemCategory::None,
            _ => return Err(()),
        })
    }
}

impl ItemId {
    pub const fn from_parts(item_id: i32, category: ItemCategory) -> Self {
        Self((item_id & 0x0FFFFFFF) | ((category as i32) << 28))
    }

    pub const fn item_id(&self) -> i32 {
        if self.0 < 0 {
            return -1;
        }
        self.0 & 0x0FFFFFFF
    }

    pub const fn category(&self) -> Result<ItemCategory, ()> {
        ItemCategory::from_i8((self.0 >> 28) as i8)
    }
}

impl From<i32> for ItemId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
