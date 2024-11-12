use std::ffi;
use std::ptr::NonNull;

use windows::core::PCWSTR;

use crate::cs::ChrSetEntry;
use crate::matrix::FSVector4;
use crate::pointer::OwningPtr;
use crate::position::{ChunkPosition, HavokPosition};
use crate::Vector;

use super::player_game_data::PlayerGameData;
use super::{FieldInsHandle, MapId};

#[repr(C)]
#[derive(Debug, Clone)]
/// Used for communicating about characters in the networking layer. This handle is essentially the
/// same as FieldInsHandle but has its MapID and selector swapped. In packets this might be packed
/// into map_id (4 bytes) + chr_selector (3 bytes). According to Sekiro's debug asserts the packed
/// version is referred to as the "whoid".
pub struct P2PEntityHandle {
    pub map_id: i32,
    pub chr_selector: i32,
}

#[repr(C)]
pub struct AtkParamLookupResult {
    behavior_param_id: u32,
    unk_param_def_meta: u32,
    is_player_atk_param: bool,
    _pad9: [u8; 7],
    param_row: usize,
}

// #[repr(C)]
// pub struct ChrInsVMT {
//     // Part of FieldInsBase, retrieves reflection metadata for FD4Component derivants.
//     pub get_runtime_metadata: fn(&ChrIns) -> usize,
//     // Destructor
//     pub destructor: fn(&ChrIns, u32) -> usize,
//     // Part of FieldInsBase, ChrIns = 1, CSBulletIns = 3, CSWorldGeomIns = 6, MapIns = 7, CSWorldGeomHitIns = 8,
//     pub get_field_ins_type: fn(&ChrIns) -> u32,
//     // Part of FieldInsBase.
//     pub use_npc_atk_param: fn(&ChrIns) -> bool,
//     // Part of FieldInsBase
//     pub get_atk_param_for_behavior: fn(&ChrIns, u32, &mut AtkParamLookupResult) -> u32,
//     // Part of FieldInsBase. ChrIns = 0, PlayerIns = 1, EnemyIns = 0, ReplayGhostIns = 1,
//     // ReplayEnemyIns = 0, CSBulletIns = 0, CSWorldGeomIns = 0, CSFieldInsBase = 0,
//     // CSHamariSimulateChrIns = 0, MapIns = 0, HitIns = 0, CSWorldGeomStaticIns = 0, HitInsBase =
//     // 0, CSWorldGeomHitIns = 0, CSWorldGeomDynamicIns = 0,
//     pub use_player_behavior_param: fn(&ChrIns) -> bool,
//     // Obfuscated beyond recognition
//     pub unk6: fn(&ChrIns),
//     // Obfuscated beyond recognition
//     pub unk7: fn(&ChrIns),
// }

#[repr(C)]
/// Abstract base class to all characters. NPCs, Enemies, Players, Summons, Ghosts, even gesturing
/// character on bloodmessages inherit from this.
///
/// Source of name: RTTI
pub struct ChrIns {
    vftable: usize,
    pub field_ins_handle: FieldInsHandle,
    chr_set_entry: usize,
    unk18: usize,
    unk20: u32,
    unk24: u32,
    chr_res: usize,
    pub map_id_1: MapId,
    pub map_id_origin_1: i32,
    pub map_id_2: MapId,
    pub map_id_origin_2: i32,
    pub chr_set_cleanup: u32,
    _pad44: u32,
    unk48: usize,
    pub chr_model_ins: OwningPtr<CSChrModelIns>,
    pub chr_ctrl: OwningPtr<ChrCtrl>,
    pub think_param_id: i32,
    pub npc_id_1: i32,
    pub chr_type: i32,
    pub team_type: i32,
    pub p2p_entity_handle: P2PEntityHandle,
    unk78: usize,
    pub unk80_position: FSVector4,
    pub unk90_position: FSVector4,
    pub unka0_position: FSVector4,
    pub chr_update_delta_time: f32,
    pub render_distance: u32,
    pub frames_per_update: u32,
    pub render_visibility: u32,
    pub target_velocity_recorder: usize,
    unkc8: usize,
    pub unkd0_position: usize,
    unkd8: [u8; 0x88],
    pub last_used_item: i16,
    unk162: i16,
    unk164: u32,
    unk168: u32,
    unk16c: u32,
    unk170: u32,
    unk174: u32,
    /// Container for the speffects applied to this character.
    pub special_effect: OwningPtr<SpecialEffect>,
    /// Refers to what field ins you were last killed by.
    pub last_killed_by: FieldInsHandle,
    pub character_id: u32,
    unk18c: u32,
    pub module_container: OwningPtr<ChrInsModuleContainer>,
    rest: [u8; 0x3E8],
}

#[repr(C)]
/// Source of name: RTTI
pub struct SpecialEffect {
    vftable: usize,
    head: Option<OwningPtr<SpecialEffectEntry>>,
    /// ChrIns this SpecialEffect structure belongs to.
    pub owner: NonNull<ChrIns>,
    unk18: usize,
    unk20: [u8; 0x118],
}

impl SpecialEffect {
    /// Yields an iterator over all the SpEffect entries contained in this SpecialEffect instance.
    pub fn entries(&self) -> impl Iterator<Item = &SpecialEffectEntry> {
        let mut current = self.head.as_ref().map(|e| e.as_ptr());

        std::iter::from_fn(move || {
            let ret = current.map(|c| unsafe { c.as_ref() }).flatten();
            current = unsafe { ret?.next.map(|e| e.as_ptr()) };
            ret
        })
    }
}

#[repr(C)]
pub struct SpecialEffectEntry {
    /// The param row this speffect entry uses.
    param_data: usize,
    /// The param ID for this speffect entry.
    pub param_id: u32,
    _padc: u32,
    pub accumulator_info: SpecialEffectEntryAccumulatorInfo,
    /// The next param entry in the doubly linked list.
    next: Option<NonNull<SpecialEffectEntry>>,
    /// The previous param entry in the doubly linked list.
    previous: Option<NonNull<SpecialEffectEntry>>,
    /// Time to go until the speffect is removed.
    pub duration: f32,
    pub duration2: f32,
    /// How long it takes the speffect before removing itself.
    pub total_duration: f32,
    pub interval_timer: f32,
    unk50: [u8; 0x28],
}

#[repr(C)]
/// Source of name: RTTI
pub struct SpecialEffectEntryAccumulatorInfo {
    unk0: usize,
    pub upper_trigger_count: i32,
    pub effect_on_upper_or_higher: i32,
    pub lower_trigger_count: i32,
    pub effect_on_lower_or_below: i32,
    unk18: i32,
    unk1c: u32,
}

#[repr(C)]
pub struct ChrInsModuleContainer {
    data: usize,
    action_flag: usize,
    behavior_script: usize,
    time_act: usize,
    resist: usize,
    behavior: usize,
    behavior_sync: usize,
    ai: usize,
    super_armor: usize,
    toughness: usize,
    talk: usize,
    event: OwningPtr<CSChrEventModule>,
    magic: usize,
    /// Describes the characters physics-related properties.
    pub physics: OwningPtr<ChrPhysicsModule>,
    fall: usize,
    ladder: usize,
    action_request: usize,
    throw: usize,
    hitstop: usize,
    damage: usize,
    material: usize,
    knockback: usize,
    sfx: usize,
    vfx: usize,
    behavior_data: usize,
    unkc8: usize,
    /// Describes a number of render-related inputs, like the color for the phantom effect and
    /// equipment coloring effects.
    pub model_param_modifier: OwningPtr<CSChrModelParamModifierModule>,
    dripping: usize,
    unke0: usize,
    ride: usize,
    bonemove: usize,
    /// Describes if your character is wet for rendering as well as applying speffects.
    wet: usize,
    auto_homing: usize,
    above_shadow_test: usize,
    sword_arts: usize,
    grass_hit: usize,
    wheel_rot: usize,
    cliff_wind: usize,
    navimesh_cost_effect: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrPhysicsModule {
    vftable: usize,
    /// ChrIns this ChrModule belongs to.
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x40],
    pub orientation: FSVector4,
    unk60_orientation: FSVector4,
    pub position: HavokPosition,
    unk80_position: HavokPosition,
    unk90: bool,
    unk91: bool,
    unk92: bool,
    unk93: bool,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrWetModule {
    vftable: usize,
    /// ChrIns this ChrModule belongs to.
    pub owner: NonNull<ChrIns>,
    pub unk10: [u8; 0x60],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrModelParamModifierModule {
    vftable: usize,
    /// ChrIns this ChrModule belongs to.
    pub owner: NonNull<ChrIns>,
    pub modifiers: Vector<CSChrModelParamModifierModuleEntry>,
}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntry {
    pub unk0: u8,
    unk1: [u8; 0x3],
    pub unk4: u32,
    pub unk8: u32,
    pub unkc: u32,
    pub unk10: u64,
    pub unk18: u32,
    pub unk1c: u32,
    pub name: PCWSTR,
    pub unk28: CSChrModelParamModifierModuleEntryValue,
    pub unk40: CSChrModelParamModifierModuleEntryValue,
    pub unk58: CSChrModelParamModifierModuleEntryValue,
    pub unk70: u32,
    pub unk74: u32,
    pub unk78: u32,
    pub unk7c: u32,
    pub unk80: u64,
    pub unk88: CSChrModelParamModifierModuleEntryValue,
    pub unka0: CSChrModelParamModifierModuleEntryValue,
    pub unkb0: [u8; 0x20],
}

unsafe impl Sync for CSChrModelParamModifierModuleEntry {}
unsafe impl Send for CSChrModelParamModifierModuleEntry {}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntryValue {
    pub unk0: u32,
    pub value1: f32,
    pub value2: f32,
    pub value3: f32,
    pub value4: f32,
    pub unk14: u32,
}

#[repr(C)]
pub struct CSChrEventModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x8],
    pub event_animation: i32,
    pub last_animation: i32,
    pub init_stay_id: i32,
    unk24: u32,
    pub ez_state_request_ladder: i32,
    unk2c: [u8; 0xB],
    pub msg_map_list_call: i32,
    unk3c: u32,
    pub flags: u8, // bit in pos 1 is iframes
    unk41: [u8; 0xA],
    pub ez_state_request_ladder_output: i32,
    unk50: [u8; 0x27],
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrCtrl {
    vftable: usize,
    unk8: u64,
    /// ChrIns this ChrCtrl belongs to.
    pub owner: NonNull<ChrIns>,
    pub manipulator: usize,
    unk20: usize,
    pub ragdoll_ins: usize,
    pub chr_collision: usize,
    unk38: [u8; 240],
    pub chr_ragdoll_state: u8,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSModelIns {
    vftable: usize,
    pub unk8: usize,
    pub model_item: usize,
    pub model_disp_entity: usize,
    pub location_entity: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrModelIns {
    pub model_ins: CSModelIns,
}

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerIns {
    pub chr_ins: ChrIns,
    pub player_game_data: OwningPtr<PlayerGameData>,
    chr_manipulator: usize,
    unk590: usize,
    player_session_holder: usize,
    unk5c0: usize,
    replay_recorder: usize,
    unk5b0: [u8; 0x88],
    pub chr_asm: OwningPtr<ChrAsm>,
    chr_asm_model_res: usize,
    chr_asm_model_ins: usize,
    unk650: [u8; 0x60],
    pub locked_on_enemy: FieldInsHandle,
    session_manager_player_entry: usize,
    pub chunk_position: ChunkPosition,
}

pub const CHR_ASM_SLOT_WEAPON_LEFT_1: usize = 0;
pub const CHR_ASM_SLOT_WEAPON_RIGHT_1: usize = 1;
pub const CHR_ASM_SLOT_WEAPON_LEFT_2: usize = 2;
pub const CHR_ASM_SLOT_WEAPON_RIGHT_2: usize = 3;
pub const CHR_ASM_SLOT_WEAPON_LEFT_3: usize = 4;
pub const CHR_ASM_SLOT_WEAPON_RIGHT_3: usize = 5;
pub const CHR_ASM_SLOT_ARROW_1: usize = 6;
pub const CHR_ASM_SLOT_BOLT_1: usize = 7;
pub const CHR_ASM_SLOT_ARROW_2: usize = 8;
pub const CHR_ASM_SLOT_BOLT_2: usize = 9;
pub const CHR_ASM_SLOT_PROTECTOR_HEAD: usize = 12;
pub const CHR_ASM_SLOT_PROTECTOR_CHEST: usize = 13;
pub const CHR_ASM_SLOT_PROTECTOR_HANDS: usize = 14;
pub const CHR_ASM_SLOT_PROTECTOR_LEGS: usize = 15;
pub const CHR_ASM_SLOT_ACCESSORY_1: usize = 17;
pub const CHR_ASM_SLOT_ACCESSORY_2: usize = 18;
pub const CHR_ASM_SLOT_ACCESSORY_3: usize = 19;
pub const CHR_ASM_SLOT_ACCESSORY_4: usize = 20;
pub const CHR_ASM_SLOT_ACCESSORY_COVENANT: usize = 21;

#[repr(C)]
/// Describes how the character should be rendered in terms of selecting the
/// appropriate parts to be rendered.
///
/// Source of name: RTTI in earlier games (vmt has been removed from ER after some patch?)
pub struct ChrAsm {
    unk0: i32,
    unk4: i32,
    /// Determines how you're holding your weapon. 1 is one-handed, 3 is dual wielded.
    pub arm_style: u32,
    /// Points to the slot in the equipment list used for rendering the left-hand weapon.
    /// 0 for primary, 1 for secondary, 2 for tertiary.
    pub left_weapon_slot: u32,
    /// Points to the slot in the equipment list used for rendering the right-hand weapon.
    /// 0 for primary, 1 for secondary, 2 for tertiary.
    pub right_weapon_slot: u32,
    /// Points to the slot in the equipment list used for rendering the left-hand arrow.
    /// 0 for primary, 1 for secondary.
    pub left_arrow_slot: u32,
    /// Points to the slot in the equipment list used for rendering the right-hand arrow.
    /// 0 for primary, 1 for secondary.
    pub right_arrow_slot: u32,
    /// Points to the slot in the equipment list used for rendering the left-hand bolt.
    /// 0 for primary, 1 for secondary.
    pub left_bolt_slot: u32,
    /// Points to the slot in the equipment list used for rendering the right-hand bolt.
    /// 0 for primary, 1 for secondary.
    pub right_bolt_slot: u32,
    /// Holds references to the inventory slots for each equipment piece.
    pub gaitem_handles: [u32; 22],
    /// Holds the param IDs for each equipment piece.
    pub equipment_param_ids: [i32; 22],
    unkd4: u32,
    unkd8: u32,
    _paddc: [u8; 12],
}
