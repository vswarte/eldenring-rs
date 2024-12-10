use std::ptr::NonNull;
use std::{ffi, usize};

use vtable_rs::VPtr;
use windows::core::PCWSTR;

use crate::cs::ChrSetEntry;
use crate::matrix::FSVector4;
use crate::pointer::OwnedPtr;
use crate::position::{BlockPoint, ChunkPosition4, HavokPosition, Quaternion};
use crate::Vector;

use super::player_game_data::PlayerGameData;
use super::{CSMsbParts, CSMsbPartsEne, CSSessionManagerPlayerEntry, FieldInsBaseVmt, FieldInsHandle, MapId, WorldBlockChr};

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

#[vtable_rs::vtable]
pub trait ChrInsVmt: FieldInsBaseVmt {
    /// Initializes a batch of combat-related modules for a ChrIns as well as initialize the
    /// initiale SpEffect state and a bunch of other stuff.
    fn initialize_character(&mut self);

    fn initialize_model_resources(&mut self);

    fn initialize_character_rendering(&mut self);
}

#[repr(C)]
/// Abstract base class to all characters. NPCs, Enemies, Players, Summons, Ghosts, even gesturing
/// character on bloodmessages inherit from this.
///
/// Source of name: RTTI
pub struct ChrIns {
    pub vftable: VPtr<dyn ChrInsVmt, Self>,
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
    pub chr_model_ins: OwnedPtr<CSChrModelIns>,
    pub chr_ctrl: OwnedPtr<ChrCtrl>,
    pub think_param_id: i32,
    pub npc_id_1: i32,
    pub chr_type: i32,
    pub team_type: i32,
    pub p2p_entity_handle: P2PEntityHandle,
    unk78: usize,
    unk80_position: FSVector4,
    unk90_position: FSVector4,
    unka0_position: FSVector4,
    /// Time in seconds since last update ran for the ChrIns.
    pub chr_update_delta_time: f32,
    pub render_distance: u32,
    /// Amount of frames between updates for this ChrIns.
    pub frames_per_update: u32,
    pub render_visibility: u32,
    pub target_velocity_recorder: usize,
    unkc8: usize,
    unkd0_position: usize,
    unkd8: [u8; 0x88],
    pub last_used_item: i16,
    unk162: i16,
    unk164: u32,
    unk168: u32,
    unk16c: u32,
    unk170: u32,
    unk174: u32,
    /// Container for the speffects applied to this character.
    pub special_effect: OwnedPtr<SpecialEffect>,
    /// Refers to what field ins you were last killed by.
    pub last_killed_by: FieldInsHandle,
    pub character_id: u32,
    unk18c: u32,
    pub module_container: OwnedPtr<ChrInsModuleContainer>,
    rest: [u8; 0x3E8],
}

#[repr(C)]
/// Source of name: RTTI
pub struct SpecialEffect {
    vftable: usize,
    head: Option<OwnedPtr<SpecialEffectEntry>>,
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
    pub data: OwnedPtr<CSChrDataModule>,
    action_flag: usize,
    behavior_script: usize,
    time_act: usize,
    resist: usize,
    behavior: usize,
    behavior_sync: usize,
    ai: usize,
    pub super_armor: OwnedPtr<CSChrSuperArmorModule>,
    pub toughness: OwnedPtr<CSChrToughnessModule>,
    talk: usize,
    pub event: OwnedPtr<CSChrEventModule>,
    magic: usize,
    /// Describes the characters physics-related properties.
    pub physics: OwnedPtr<ChrPhysicsModule>,
    fall: usize,
    ladder: usize,
    action_request: usize,
    throw: OwnedPtr<CSChrThrowModule>,
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
    pub model_param_modifier: OwnedPtr<CSChrModelParamModifierModule>,
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
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x40],
    pub orientation: Quaternion,
    unk60_orientation: Quaternion,
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
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x60],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrModelParamModifierModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub modifiers: Vector<CSChrModelParamModifierModuleEntry>,
}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntry {
    unk0: u8,
    unk1: [u8; 0x3],
    unk4: u32,
    unk8: u32,
    unkc: u32,
    unk10: u64,
    unk18: u32,
    unk1c: u32,
    pub name: PCWSTR,
    unk28: CSChrModelParamModifierModuleEntryValue,
    unk40: CSChrModelParamModifierModuleEntryValue,
    unk58: CSChrModelParamModifierModuleEntryValue,
    unk70: u32,
    unk74: u32,
    unk78: u32,
    unk7c: u32,
    unk80: u64,
    unk88: CSChrModelParamModifierModuleEntryValue,
    unka0: CSChrModelParamModifierModuleEntryValue,
    unkb0: [u8; 0x20],
}

unsafe impl Sync for CSChrModelParamModifierModuleEntry {}
unsafe impl Send for CSChrModelParamModifierModuleEntry {}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntryValue {
    unk0: u32,
    pub value1: f32,
    pub value2: f32,
    pub value3: f32,
    pub value4: f32,
    unk14: u32,
}

#[repr(C)]
pub struct CSChrEventModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x8],
    /// Animation ID that should be played immediately.
    pub request_animation_id: i32,
    /// Current animation ID.
    pub current_animation: i32,
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
pub struct CSChrSuperArmorModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Current super armor of the character, related to poise.
    pub sa_durability: f32,
    /// Maximum super armor of the character.
    pub sa_durability_max: f32,
    unk18: u32,
    /// Time to lost super armor reset.
    pub recover_time: f32,
    unk20: u32,
    unk24: u32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrToughnessModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Current toughness of the character, related to stance break
    pub toughness: f32,
    toughness_unk: f32,
    /// Maximum toughness of the character
    pub toughness_max: f32,
    /// Time to lost toughness reset.
    pub recover_time: f32,
    unk20: [u8; 0x108],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrDataModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub msb_parts: CSMsbPartsEne,
    msb_res_cap: usize,
    unk68: usize,
    unk70: u32,
    unk74: u32,
    unk78: u32,
    pub map_id_origin: u32,
    unk80: u32,
    unk84: u32,
    pub world_block_chr: NonNull<WorldBlockChr<ChrIns>>,
    unk90: [u8; 0x30],
    pub draw_params: u32,
    unkc4: u32,
    // wchar_t[6]
    unkc8: [u8; 0xc],
    unkd4: [u8; 0x64],
    pub hp: i32,
    pub max_hp: i32,
    pub max_uncapped_hp: i32,
    pub base_hp: i32,
    pub fp: i32,
    pub max_fp: i32,
    pub base_fp: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub base_stamina: i32,
    recoverable_hp_1: f32,
    recoverable_hp_2: f32,
    pub recoverable_hp_time: f32,
    unk16c: f32,
    unk170: [u8; 0x28],
    unk198: [u8; 0x3],
    // 2nd bit makes you undamageable
    debug_flags: u8,
    unk19c: [u8; 0x8c],
    // wchar_t*
    character_name: OwnedPtr<ffi::OsString>,
    unk230: [u8; 0x20],
    dl_string: [u8; 0x30],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSPairAnimNode {
    vftable: usize,
    unk8: usize,
    pub owner: OwnedPtr<ChrIns>,
    pub forwarding_recipient: FieldInsHandle,
    unk20: FSVector4,
    unk30: FSVector4,
    unk40: u32,
    unk44: [u8; 0xc],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSThrowNode {
    pub super_pair_anim_node: CSPairAnimNode,
    unk58: [u8; 0x18],
    pub throw_state: u32,
    unk6c: u32,
    unk70: f32,
    unk74: f32,
    unk78: f32,
    unk7c: [u8; 0x34],
    /// available only for main player
    throw_self_esc: usize,
    unkb8: [u8; 0xb8],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrThrowModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub throw_node: OwnedPtr<CSThrowNode>,
    unk18: usize,
    unk20: u32,
    // p2p handle of the target?, need verification
    p2p_entity_handle: P2PEntityHandle,
    // field ins handle of the target?, need verification
    throw_target: usize,
    unk28: [u8; 0x8],
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrCtrl {
    vftable: usize,
    unk8: u64,
    pub owner: NonNull<ChrIns>,
    pub manipulator: usize,
    unk20: usize,
    pub ragdoll_ins: usize,
    pub chr_collision: usize,
    unk38: [u8; 0xb8],
    pub flags: u32,
    unkf1: [u8; 0x34],
    pub chr_ragdoll_state: u8,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSModelIns {
    vftable: usize,
    unk8: usize,
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
    pub player_game_data: OwnedPtr<PlayerGameData>,
    chr_manipulator: usize,
    unk590: usize,
    pub player_session_holder: PlayerSessionHolder,
    unk5c0: usize,
    replay_recorder: usize,
    unk5d0: u32,
    unk5d4: u32,
    pub snipe_mode_draw_alpha_fade_timer: f32,
    unk5bc: u32,
    unk5e0: usize,
    fg_model: usize,
    npc_param: usize,
    think_param: u32,
    unk5fc: u32,
    rng_sp_effect_equip_ctrl: usize,
    wep_sp_effect_equip_ctrl: usize,
    pro_sp_effect_equip_ctrl: usize,
    npc_sp_effect_equip_ctrl: usize,
    unk620: [u8; 0x18],
    pub chr_asm: OwnedPtr<ChrAsm>,
    chr_asm_model_res: usize,
    chr_asm_model_ins: usize,
    unk650: [u8; 0x60],
    pub locked_on_enemy: FieldInsHandle,
    pub session_manager_player_entry: NonNull<CSSessionManagerPlayerEntry>,
    /// Position within the current block.
    pub block_position: BlockPoint,
    /// Angle as radians. Relative to the orientation of the current block.
    pub block_orientation: f32,
}

impl AsRef<ChrIns> for PlayerIns {
    fn as_ref(&self) -> &ChrIns {
        &self.chr_ins
    }
}

#[repr(C)]
/// Source of name: RTTI
pub struct EnemyIns {
    pub chr_ins: ChrIns,
    pub com_manipulator: usize,
    pub net_ai_manipulator: usize,
    pub ride_manipulator: usize,
    unk598: usize,
    pub npc_think_param: i32,
    unk5a4: u32,
    npc_sp_effect_equip_ctrl: usize,
    map_studio_sp_effect_equip_ctrl: usize,
    unk5b8: [u8; 0x28],
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

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerSessionHolder {
    vftable: usize,
    player_debug_session: usize,
    unk10: usize,
    player_netword_session: usize,
    unk18: usize,
}
