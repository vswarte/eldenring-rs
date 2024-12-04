use std::ptr::NonNull;

use crate::{dltx::DLString, fd4::FD4StepBaseInterface, pointer::OwnedPtr, stl::DoublyLinkedList};

use super::{CSEzTask, CSEzUpdateTask, MapId, PlayerIns};

#[repr(C)]
#[dlrf::singleton("CSNetMan")]
pub struct CSNetMan {
    vftable: usize,
    unk8: [u8; 0x60],
    sos_db: usize,
    wandering_ghost_db: usize,
    /// Keeps track of all all bloodmessages in the world as well as any rating and created
    /// bloodmessages.
    pub blood_message_db: OwnedPtr<CSNetBloodMessageDb>,
    bloodstain_db: usize,
    bonfire_db: usize,
    spiritual_statue_db: usize,
    unk98: usize,
    unka0: usize,
    unka8: usize,
    /// Keeps track of quickmatch gamemode state.
    pub quickmatch_manager: OwnedPtr<QuickmatchManager>,
    visitor_db: usize,
    penalty_manager: usize,
    /// Task that updates the structure (pulls in new data from server, spawn received signs,
    /// stains and messages, spawns ghost replays, etc)
    pub update_task: CSEzUpdateTask<Self>,
    unkf0: u32,
    unkf4: u32, // Probably padding
    unkf8: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageDb {
    vftable: usize,
    // Contains all CSNetBloodMessageDbItem?
    pub entries: DoublyLinkedList<OwnedPtr<CSNetBloodMessageDbItem>>,
    unk20: usize,
    /// Seemingly contains message data for messages created by local user
    pub created_data: DoublyLinkedList<usize>,
    // Contains ???
    unk40: DoublyLinkedList<usize>,
    unk58: usize,
    blood_message_ins_man_1: usize,
    blood_message_ins_man_2: usize,
    pub discovered_messages: DoublyLinkedList<OwnedPtr<OwnedPtr<CSNetBloodMessageDbItem>>>,
    unk88: [u8; 0xD0],
    /// Hosts any ongoing jobs for evaluations.
    evaluate_job: usize,
    unk160: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageDbItem {
    vftable: usize,
    unk8: u32,
    unkc: u32,
    unk10: u32,
    pub map_id: MapId,
    unk18: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub angle: f32,
    pub template1: u16,
    pub gesture_param: u16,
    pub part1: u16,
    pub infix: u16,
    pub template2: u16,
    pub part2: u16,
    unk38: u16,
    unk3a: u16,
    unk3c: u16,
    unk3e: u16,
    pub message_id: u64,
    unk48: u32,
}

#[repr(C)]
pub struct QuickmatchManager {
    /// Stepper that updates the games quickmatch state.
    pub quickmatching_ctrl: OwnedPtr<CSQuickMatchingCtrl>,
    /// Keeps track of quickmatch settings as well as any participants.
    pub battle_royal_context: OwnedPtr<CSBattleRoyalContext>,
    battle_royal_context_active: Option<NonNull<CSBattleRoyalContext>>,
    unk18: u32,
    /// List of speffects applied to the players during various states for the battle.
    pub utility_sp_effects: [u32; 10],
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSQuickMatchingCtrl {
    pub base: FD4StepBaseInterface<15, Self>,
    unk18: [u8; 0x28],
    pub current_state: u32,
    pub requested_state: u32,
    // TODO: rest....
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSBattleRoyalContext {
    pub quickmatch_context: CSQuickMatchContext,
    /// Required players to be in lobby before quickmatch can kick-off.
    pub match_player_count: u32,
    unkb4: u32,
    unkb8: u32,
    /// Map ID as an enum (1, 2, 3).
    pub map: u32,
    /// Password used for the quickmatch lobby.
    pub password: DLString,
    /// Seems involved in some map ID sanity checks.
    unkf0: bool,
    unkf1: u8,
    unkf2: u8,
    unkf3: u8,
    unkf4: u32,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSQuickMatchContext {
    vtable: usize,
    /// Encodes the battle type (1v1, 2v2, 3v3, etc)
    pub match_settings: u32,
    /// Map for this map as an integer, 45000000 as an example.
    pub match_map: u32,
    unk10: u32,
    unk14: f32,
    unk18: f32,
    unk1c: f32,
    unk20: u32,
    unk24: u32,
    unk28: u64,
    unk30: u64,
    unk38: u64,
    unk40: u64,
    unk48: u64,
    unk50: u64,
    unk58: u64,
    unk60: u64,
    unk68: u64,
    unk70: u64,
    unk78: u64,
    /// All quickmatch participants.
    pub participants: DoublyLinkedList<QuickmatchParticipant>,
    unk98: u8,
    /// Seems to be indicative of why some QM lobby failed
    pub error_state: u8,
    unk9a: u8,
    unk9b: u8,
    unk9c: u32,
    unka0: u32,
    unka4: u32,
    unka8: u32,
    unkac: u32,
}

#[repr(C)]
pub struct QuickmatchParticipant {
}
