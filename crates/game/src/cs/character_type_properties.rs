use pelite::Pod;

#[repr(C)]
#[derive(Debug, Pod)]
pub struct CharacterTypePropertiesEntry {
    pub unk0: u8,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
    pub can_use_rune_arcs: u8,
    pub can_receive_message_rate_buff: u8,
    pub check_item_use_for_white_phantoms: u8,
    pub check_item_use_for_black_phantoms: u8,
    pub unkb: u8,
    pub unkc: i32,
    pub unk10: i32,
}

#[repr(C)]
#[derive(Debug, Pod)]
pub struct CharacterTypePropertiesTable {
    pub entries: [CharacterTypePropertiesEntry; 22],
    pub default: CharacterTypePropertiesEntry,
}
