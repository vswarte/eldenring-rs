use pelite::Pod;

#[repr(C)]
#[derive(Debug, Pod)]
pub struct CharacterTypePropertiesEntry {
    unk0: u8,
    unk1: u8,
    unk2: u8,
    unk3: u8,
    unk4: u8,
    unk5: u8,
    unk6: u8,
    pub can_use_rune_arcs: u8,
    pub can_receive_message_rate_buff: u8,
    pub is_white_phantom: u8,
    pub is_black_phantom: u8,
    unkb: u8,
    unkc: i32,
    unk10: i32,
}

#[repr(C)]
#[derive(Debug, Pod)]
pub struct CharacterTypePropertiesTable {
    pub entries: [CharacterTypePropertiesEntry; 22],
    pub default: CharacterTypePropertiesEntry,
}
