use std::ptr::NonNull;

use crate::{dltx::DLString, matrix::FSVector4, pointer::OwnedPtr, CSFixedList};

use super::{CSMenuManImp, FieldInsHandle};

#[repr(C)]
#[dlrf::singleton("CSFeMan")]
pub struct CSFeManImp {
    vftable: usize,
    unk8: usize,
    game_rend: usize,
    pub menu_man: NonNull<CSMenuManImp>,
    /// Object, containing all scaleform data for FrontEnd (FE) scene
    pub front_end_view: OwnedPtr<FrontEndView>,
    fade_screen: usize,
    fe_system_announce_view: usize,
    fe_summon_message_view: usize,
    clock_view: usize,
    unk40: [u8; 40],
    unk70: usize,
    /// Toggle, to enable/disable the HUD
    pub enable_hud: bool,
    unk79: [u8; 7],
    /// Structure, holding intermidiate data FrontEndView
    /// read from menu window jobs
    pub frontend_values: FrontEndViewValues,
    unk4e5c: [u8; 4],
    auto_hide_ctrl_ptr: usize,
    unk4e68: [u8; 8],
    auto_hide_ctrl: [u8; 0xb04],
    unk5974: [u8; 4],
    menu_resist_gauge: [u8; 0x28],
    unk59a0: [u8; 4],
    pub displayed_proc_status_messages: [i32; 6],
    unk59bc: [u8; 4],
    pub next_free_proc_message_slot: u32,
    unk59c4: [u8; 12],
    unk59d0: FSVector4,
    unk59e0: u32,
    unk59e4: [u8; 12],
    unk59f0: [u8; 0x40],
    unk5a30: [u8; 0x1c0],
    /// Data used for the boss health display
    /// Will be copied to the boss health tags in FrontEndView
    pub boss_health_displays: [BossHealthDisplayEntry; 3],
    unk5c50: [u8; 0x10],
    /// Data used for the player tag display
    /// Will be copied to the player tags in FrontEndView
    pub player_tag_displays: [ChrTagEntry; 7],
    unk6130: f32,
    unk6134: [u8; 20],
    fe_menu_chr_state_data: [u8; 0x168],
    pub summon_msg_queue: SummonMsgQueue,
    unk6530: u32,
    pub subarea_name_popup_message_id: i32,
    unk6538: [u8; 24],
    unk6550: u32,
    /// Toggle, requesting the area welcome message
    /// to be displayed
    pub area_welcome_message_request: bool,
    unk6555: [u8; 11],
    pub get_item_log_view_model: [u8; 0x1d48],
    unk82a8: [u8; 8],
    pub clock_view_model: usize,
    unk82b0: [u8; 16],
    /// Tag of the debug player
    pub debug_tag: TagHudData,
    unk83f0: [u8; 48],
}

#[repr(C)]
pub struct BossHealthDisplayEntry {
    /// Id of the fmg text entry for the boss name
    pub fmg_id: i32,
    unk4: [u8; 4],
    pub field_ins_handle: FieldInsHandle,
    unk10: u32,
    unk14: u32,
    unk18: [u8; 0x8],
}

#[repr(C)]
pub struct SummonMsgQueue {
    vftable: usize,
    pub current: SummonMsgData,
    pub list: CSFixedList<SummonMsgData, 4>,
    unk278: u32,
    unk27c: u32,
}

#[repr(C)]
pub struct SummonMsgData {
    vftable: usize,
    pub priority: i16,
    pub force_play: bool,
    unkb: [u8; 5],
    pub text: MenuLabelString,
    unk48: bool,
    unk49: [u8; 7],
}

#[repr(C)]
pub struct FrontEndView {
    menu_window: [u8; 0xa31],
    unka31: [u8; 0x7],
    root_scene: [u8; 0x251c8],
    pub front_end_view_values: NonNull<FrontEndViewValues>,
}

#[repr(C)]
pub struct FrontEndViewValues {
    unk0: [u8; 4],
    pub player_hp: u32,
    pub max_recoverable_hp: u32,
    pub hp_max: u32,
    pub hp_max_uncapped: u32,
    pub enable_hp_rally: bool,
    unk1a: [u8; 3],
    pub fp: u32,
    unk20: [u8; 8],
    pub fp_max: u32,
    pub enable_equip_hud: bool,
    unk2a: [u8; 3],
    pub stamina: u32,
    unk30: [u8; 8],
    pub stamina_max: u32,
    unk38: [u8; 0x1148],
    pub quickmatch_elimination_count: i32,
    unk1184: [u8; 0x48],
    pub enemy_tag_data: [TagHudData; 8],
    pub boss_list_tag_data: [TagHudData; 3],
    pub remote_pc_tag_data: [TagHudData; 7],
    unk26a0: [u8; 0xfb8],
    unk3658: MenuLabelString,
    unk36d8: [u8; 8],
    pub full_screen_message_request_id: i32,
    unk36e4: [u8; 4],
    unk36e8: MenuLabelString,
    unk3720: MenuLabelString,
    unk3758: MenuLabelString,
    unk3790: MenuLabelString,
    unk37c8: MenuLabelString,
    unk3800: [u8; 0x1558],
    pub summoned_spirit_ash_count: u32,
    unk4d5c: [u8; 4],
    pub spirit_ash_display: [CSFeSpiritAshDisplay; 5],
    unk4dd8: [u8; 4],
}

#[repr(C)]
pub struct CSFeSpiritAshDisplay {
    pub field_ins_handle: FieldInsHandle,
    pub hp: u32,
    unkc: u32,
    pub max_hp: u32,
    pub hp_max_uncapped: u32,
}

#[repr(C)]
pub struct MenuLabelString {
    pub raw_string: *const u16,
    pub string: DLString,
}

#[repr(C)]
pub struct TagHudData {
    unk0: [u8; 0x18],
    pub hp: u32,
    pub max_hp: u32,
    pub hp_max_uncapped: u32,
    pub chr_name: MenuLabelString,
    pub role_string: MenuLabelString,
    unk98: [u8; 0x40],
    pub has_sp_effect_592: bool,
    unkd9: [u8; 0xb],
    pub role_name_color: u8,
    unke5: [u8; 0x43],
}

#[repr(C)]
pub struct ChrTagEntry {
    unk0: FSVector4,
    unk10: bool,
    /// Is line of sight to this character blocked?
    pub is_line_of_sight_blocked: bool,
    /// Can this character be locked on to?
    pub is_valid_target: bool,
    pub role_name_color: u8,
    unk14: [u8; 4],
    /// String, containing the role of the character
    pub role_string: DLString,
    /// String, containing the name of the character
    pub name_string: DLString,
    /// The max hp of the character, uncapped
    pub hp_max_uncapped: u32,
    /// The current hp of the character
    pub hp: u32,
    /// The max recoverable hp of the character
    /// Only works if character has malenia rune arc active
    pub max_recoverable_hp: u32,
    /// The max hp of the character
    pub max_hp: u32,
    unk88: [u8; 8],
    pub voice_chat_state: u32,
    unk94: [u8; 4],
    /// Handle to the character
    pub field_ins_handle: FieldInsHandle,
    unka0: [u8; 2],
    /// The team type of the character
    pub team_type: u8,
    /// Does this character have the sp effect 590?
    pub has_sp_effect_590: bool,
    unk_a4: u8,
    /// Is this summon a debug summon
    /// Will replace the name to "Debug" in japanese
    /// and role to white summon
    pub is_debug_summon: bool,
    unka5: [u8; 10],
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x83E0, size_of::<CSFeManImp>());
        assert_eq!(0x20, size_of::<BossHealthDisplayEntry>());
        assert_eq!(0x280, size_of::<SummonMsgQueue>());
        assert_eq!(0x50, size_of::<SummonMsgData>());
        assert_eq!(0xb0, size_of::<ChrTagEntry>());
        assert_eq!(0x128, size_of::<TagHudData>());
        assert_eq!(0x25C08, size_of::<FrontEndView>());
        assert_eq!(0x4D98, size_of::<FrontEndViewValues>());
        assert_eq!(0x18, size_of::<CSFeSpiritAshDisplay>());
        assert_eq!(0x38, size_of::<MenuLabelString>());
    }
}
