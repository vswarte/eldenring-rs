use std::ptr::NonNull;

use crate::pointer::OwnedPtr;

use super::ChrAsm;

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerGameData {
    vftable: usize,
    pub character_type: u32,
    unkc: u32,
    pub current_hp: u32,
    pub current_max_hp: u32,
    pub base_max_hp: u32,
    pub current_fp: u32,
    pub current_max_fp: u32,
    pub base_max_fp: u32,
    unk28: f32,
    pub current_stamina: u32,
    pub current_max_stamina: u32,
    pub base_max_stamina: u32,
    unk38: f32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    unk5c: f32,
    unk60: f32,
    unk64: f32,
    pub level: u32,
    pub rune_count: u32,
    pub rune_memory: u32,
    unk74: u32,
    pub poison_resist: u32,
    pub rot_resist: u32,
    pub bleed_resist: u32,
    pub death_resist: u32,
    pub frost_resist: u32,
    pub sleep_resist: u32,
    pub madness_resist: u32,
    unk94: u32,
    unk98: u32,
    character_name: [u16; 16],
    unkbc: u8,
    unkbd: u8,
    pub gender: u8,
    pub archetype: u8,
    pub vow_type: u8,
    unkc1: u8,
    pub voice_type: u8,
    pub starting_gift: u8,
    unkc4: u8,
    pub unlocked_magic_slots: u8,
    unkc6: [u8; 0x19],
    pub furlcalling_finger_remedy_active: u8,
    unke0: u8,
    unke1: u8,
    pub matching_weapon_level: u8,
    pub white_ring_active: u8,
    pub blue_ring_active: u8,
    unke5: [u8; 0x7],
    unkec: u32,
    unkf0: [u8; 0x4],
    pub solo_breakin_point: u32,
    unkf8: [u8; 0x7],
    pub rune_arc_active: u8,
    unk100: u8,
    pub max_hp_flask: u8,
    pub max_fp_flask: u8,
    unk103: [u8; 0x6],
    pub reached_max_rune_memory: u8,
    unk10a: [u8; 0xE],
    pub password: [u16; 0x8],
    unk128: u16,
    group_password_1: [u16; 0x8],
    unk13a: u16,
    group_password_2: [u16; 0x8],
    unk14c: u16,
    group_password_3: [u16; 0x8],
    unk15e: u16,
    group_password_4: [u16; 0x8],
    unk170: u16,
    group_password_5: [u16; 0x8],
    unk182: u16,
    unk184: [u8; 0x34],
    pub sp_effects: [PlayerGameDataSpEffect; 0xD],
    /// Level after any buffs and corrections
    pub effective_vigor: u32,
    /// Level after any buffs and corrections
    pub effective_mind: u32,
    /// Level after any buffs and corrections
    pub effective_endurance: u32,
    /// Level after any buffs and corrections
    pub effective_vitality: u32,
    /// Level after any buffs and corrections
    pub effective_strength: u32,
    /// Level after any buffs and corrections
    pub effective_dexterity: u32,
    /// Level after any buffs and corrections
    pub effective_intelligence: u32,
    /// Level after any buffs and corrections
    pub effective_faith: u32,
    /// Level after any buffs and corrections
    pub effective_arcane: u32,
    unk2ac: u32,
    pub equipment: EquipGameData,
    face_data: [u8; 0x170],
    /// Describes the storage box contents.
    pub storage: OwnedPtr<EquipInventoryData>,
    gesture_game_data: usize,
    ride_game_data: usize,
    unk8e8: usize,
    unk8f0: [u8; 0x10],
    unk900: u32,
    pub hp_estus_rate: f32,
    pub hp_estus_additional: u8,
    _pad909: [u8; 3],
    pub fp_estus_rate: f32,
    pub fp_estus_additional: u8,
    _pad911: [u8; 3],
    unk914: [u8; 0x164],
    menu_ref_special_effect_1: usize,
    menu_ref_special_effect_2: usize,
    menu_ref_special_effect_3: usize,
    unka90: [u8; 0x58],
}

#[repr(C)]
pub struct PlayerGameDataSpEffect {
    pub sp_effect_id: u32,
    pub duration: f32,
    unk8: u32,
    unkc: u32,
}

#[repr(C)]
pub struct EquipGameData {
    vftable: usize,
    unk8: [u32; 22],
    unk60: usize,
    unk68: u32,
    pub chr_asm: ChrAsm,
    _pad154: u32,
    pub equip_inventory_data: EquipInventoryData,
    pub equip_magic_data: OwnedPtr<EquipMagicData>,
    pub equip_item_data: EquipItemData,
    unk330: u32,
    unk334: u32,
    unk338: u32,
    unk33c: u32,
    unk340: u32,
    unk344: u32,
    pub weapon_primary_left: i32,
    pub weapon_primary_right: i32,
    pub weapon_secondary_left: i32,
    pub weapon_secondary_right: i32,
    pub weapon_tertiary_left: i32,
    pub weapon_tertiary_right: i32,
    pub arrow_primary: i32,
    pub bolt_primary: i32,
    pub arrow_secondary: i32,
    pub bolt_secondary: i32,
    pub arrow_tertiary: i32,
    pub bolt_tertiary: i32,
    pub protector_head: i32,
    pub protector_chest: i32,
    pub protector_hands: i32,
    pub protector_legs: i32,
    unk388: i32,
    pub accessories: [i32; 4],
    pub covenant: i32,
    pub quick_tems: [i32; 10],
    pub pouch: [i32; 6],
    unk3e0: usize,
    unk3e8: usize,
    pub player_game_data: NonNull<PlayerGameData>,
    unk3f8: [u8; 0xb8], 
}

#[repr(C)]
pub struct EquipInventoryData {
    vftable: usize,
    /// How many items can one hold in total?
    pub global_capacity: u32,

    pub normal_item_capacity: u32,
    normal_item_head: OwnedPtr<EquipInventoryDataListEntry>,
    pub normal_item_count: u32,

    pub key_item_capacity: u32,
    key_item_head: OwnedPtr<EquipInventoryDataListEntry>,
    pub key_item_count: u32,

    pub secondary_key_item_capacity: u32,
    secondary_key_item_head: OwnedPtr<EquipInventoryDataListEntry>,
    pub secondary_key_item_count: u32,

    _pad3c: u32,

    normal_item_head_ptr: *mut EquipInventoryDataListEntry,
    normal_item_count_ptr: *const u32,
    key_item_head_ptr: *mut EquipInventoryDataListEntry,
    key_item_count_ptr: *const u32,

    unk_list_1_head: *mut u16,
    unk_list_1_count: u64,
    unk_list_2_head: *mut u16,
    unk_list_2_count: u64,

    unk80: [u8; 0xa8],
}

impl EquipInventoryData {
    pub fn normal_items(&self) -> &mut [EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.normal_item_head.as_ptr(),
                self.normal_item_count as usize,
            )
        }
    }

    pub fn key_items(&self) -> &mut [EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.key_item_head.as_ptr(),
                self.key_item_count as usize,
            )
        }
    }

    pub fn secondary_key_items(&self) -> &mut [EquipInventoryDataListEntry] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.secondary_key_item_head.as_ptr(),
                self.secondary_key_item_count as usize,
            )
        }
    }
}

#[repr(C)]
pub struct EquipInventoryDataListEntry {
    /// Handle to the gaitem instance which describes additional properties to the inventory item,
    /// like durability and gems in the case of weapons.
    pub gaitem_handle: u32,
    /// Item ID without the category.
    pub item_id: u16,
    unk6: u8,
    /// Item category, goods, weapon, protector, accessory, etc.
    pub category: u8,
    /// Quantity of the item we have.
    pub quantity: u32,
    pub display_id: u32,
    unk10: u8,
    _pad11: [u8; 3],
    unk14: i32,
}

#[repr(C)]
pub struct EquipMagicData {
    vftable: usize,
    pub equip_game_data: NonNull<EquipGameData>,
    pub entries: [EquipDataItem; 14],
    pub selected_slot: u32,
    unk84: u32,
}


#[repr(C)]
pub struct EquipItemData {
    vftable: usize,
    pub quick_slots: [EquipDataItem; 10],
    pub pouch_slots: [EquipDataItem; 6],
    pub great_rune: EquipDataItem,
    unk90: usize,
    pub inventory: OwnedPtr<EquipInventoryData>,
    unka0: i32,
    unka4: u32,
}

#[repr(C)]
pub struct EquipDataItem {
    pub id: i32,
    unk4: i32,
}
