use std::fmt::Display;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ItemId(pub u32);

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Weapon = 0,
    Protector = 1,
    Accessory = 2,
    Goods = 4,
    Gem = 8,
    // u4 max
    None = 15,
}

impl From<u8> for ItemCategory {
    fn from(value: u8) -> Self {
        match value {
            0 => ItemCategory::Weapon,
            1 => ItemCategory::Protector,
            2 => ItemCategory::Accessory,
            4 => ItemCategory::Goods,
            8 => ItemCategory::Gem,
            15 => ItemCategory::None,
            _ => panic!("Invalid item category"),
        }
    }
}

impl ItemId {
    pub const fn from_parts(item_id: u32, category: ItemCategory) -> Self {
        Self((item_id & 0x0FFFFFFF) | ((category as u32) << 28))
    }

    pub const fn item_id(&self) -> u32 {
        self.0 & 0x0FFFFFFF
    }

    pub fn category(&self) -> ItemCategory {
        ItemCategory::from((self.0 >> 28) as u8)
    }
}

impl From<u32> for ItemId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Display for ItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Item ID: {:0>8}, Category: {:?}",
            self.item_id(),
            self.category()
        )
    }
}
