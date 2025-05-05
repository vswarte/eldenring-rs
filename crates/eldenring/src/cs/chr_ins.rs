use std::ffi;
use std::ops::Index;
use std::ptr::NonNull;
use std::slice::SliceIndex;
use vtable_rs::VPtr;
use windows::core::PCWSTR;

use crate::cs::MapId;
use crate::fd4::FD4Time;
use crate::position::{BlockPosition, HavokPosition};
use crate::rotation::Quaternion;
use crate::Vector;
use shared::{FSMatrix4x4, FSVector4, OwnedPtr};

use crate::cs::field_ins::{FieldInsBaseVmt, FieldInsHandle};
use crate::cs::gaitem::GaitemHandle;
use crate::cs::network_session::PlayerNetworkSession;
use crate::cs::player_game_data::{ChrAsm, PlayerGameData};
use crate::cs::session_manager::{SessionManagerPlayerEntry, SessionManagerPlayerEntryBase};
use crate::cs::task::{CSEzRabbitNoUpdateTask, CSEzVoidTask};
use crate::cs::world_chr_man::{ChrSetEntry, WorldBlockChr};
use crate::cs::world_geom_man::{CSMsbParts, CSMsbPartsEne};
use crate::cs::ItemId;

use super::{ItemId, NpcSpEffectEquipCtrl, SpecialEffect};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub struct NetChrSyncFlags(pub u8);

impl NetChrSyncFlags {
    pub fn set_unk2(&mut self, val: bool) {
        self.0 = self.0 & 0b11111011 | (val as u8) << 2
    }

    pub const fn unk2(&self) -> bool {
        self.0 & 0b00000100 != 0
    }

    pub fn set_distance_based_network_update_authority(&mut self, val: bool) {
        self.0 = self.0 & 0b11011111 | (val as u8) << 5
    }

    pub const fn distance_based_network_update_authority(&self) -> bool {
        self.0 & 0b00100000 != 0
    }
}

#[repr(i32)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum OmissionMode {
    NoUpdate = -2,
    Normal = 0,
    OneFps = 1,
    FiveFps = 5,
    TwentyFps = 20,
    ThirtyFps = 30,
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
    pub backread_state: u32,
    unk24: u32,
    chr_res: usize,
    pub map_id_1: MapId,
    pub map_id_origin_1: i32,
    pub block_origin_override: MapId,
    pub block_origin: MapId,
    pub chr_set_cleanup: u32,
    _pad44: u32,
    unk48: usize,
    pub chr_model_ins: OwnedPtr<CSChrModelIns>,
    pub chr_ctrl: OwnedPtr<ChrCtrl>,
    pub think_param_id: i32,
    pub npc_id_1: i32,
    pub chr_type: ChrType,
    pub team_type: u8,
    pad6d: [u8; 3],
    pub p2p_entity_handle: P2PEntityHandle,
    unk78: usize,
    unk80_position: FSVector4,
    unk90_position: FSVector4,
    unka0_position: FSVector4,
    /// Time in seconds since last update ran for the ChrIns.
    pub chr_update_delta_time: f32,
    pub omission_mode: OmissionMode,
    /// Amount of frames between updates for this ChrIns.
    /// Uses same values as omission mode.
    pub frames_per_update: OmissionMode,
    unkbc: OmissionMode,
    pub target_velocity_recorder: usize,
    unkc8: usize,
    pub lock_on_target_position: FSVector4,
    unkd8: [u8; 0x80],
    /// Used by TAE's UseGoods to figure out what item to actually apply.
    pub tae_queued_use_item: ItemId,
    unk164: u32,
    unk168: u32,
    unk16c: u32,
    unk170: u32,
    unk174: u32,
    /// Container for the speffects applied to this character.
    pub special_effect: OwnedPtr<SpecialEffect>,
    /// Refers to what field ins you were last hit by.
    pub last_hit_by: FieldInsHandle,
    pub character_id: u32,
    unk18c: u32,
    pub module_container: OwnedPtr<ChrInsModuleContainer>,
    unk198: usize,
    unk1a0: f32,
    unk1a4: f32,
    unk1a8: f32,
    unk1ac: f32,
    unk1b0: f32,
    unk1b4: f32,
    unk1b8: f32,
    unk1bc: f32,
    unk1c0: u32,
    pub chr_flags: ChrInsFlags,
    pub net_chr_sync_flags: NetChrSyncFlags,
    unk1ca: u8,
    unk1cb: u8,
    _pad1cc: u32,
    unk1d0: FSVector4,
    unk1e0: u32,
    pub network_authority: u32,
    pub event_entity_id: u32,
    unk1ec: f32,
    unk1f0: usize,
    pub npc_sp_effect_equip_ctrl: OwnedPtr<NpcSpEffectEquipCtrl>,
    unk200: [u8; 0x18],
    pub character_creator_steam_id: u64,
    /// What asset to use for the mimic veil.
    pub mimicry_asset: i32,
    /// Row ID of the MAP_MIMICRY_ESTABLISHMENT_PARAM, determines stuff like entry and exit
    /// sfx.
    pub mimicry_establishment_param_id: i32,
    unk228: u32,
    unk22c: u32,
    pub chr_fade_multiplier: f32,
    pub chr_fade_multiplier_reset: f32,
    unk238: f32,
    unk23c: f32,
    unk240: f32,
    unk244: f32,
    unk248: f32,
    unk24c: f32,
    unk250: [u8; 0xc],
    unk25c: [u8; 0x20],
    unk27c: [u8; 0x84],
    chr_slot_sys: [u8; 0x40],
    unk340: usize,
    unk348: [u8; 0x40],
    last_received_packet60: u32,
    unk38c: [u8; 0xc],
    hka_pose_importer: usize,
    unk3a0: usize,
    anim_skeleton_to_model_modifier: usize,
    unk3b0: usize,
    cloth_state: [u8; 0x30],
    unk3e8: [u8; 0x28],
    update_data_module_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_chr_ctrl_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_chr_model_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_havok_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_replay_recorder_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    update_behavior_task: CSEzVoidTask<CSEzRabbitNoUpdateTask, ChrIns>,
    unk530_debug_flags: u64,
    unk538: u64,
    unk540: u32,
    pub role_param_id: i32,
    unk548: [u8; 0x38],
}

#[repr(C)]
pub struct ChrInsFlags([u8; 5]);

impl ChrInsFlags {
    // byte 0 bit 0 Skip omission mode updates
    // byte 0 bit 5 No gravity
    // byte 1 bit 3 Enabe render
    // byte 1 bit 7 Death flag
    // byte 2 bit 4 Draw tag offscreen
    // byte 4 bit 2 Trigger falldeath camera (See note below)
    // byte 4 bit 4 Enable character tag
    pub fn set_skip_omission_mode_updates(&mut self, val: bool) {
        self.0[0] = self.0[0] & 0b11111110 | val as u8;
    }
    /// Controls if the character omission mode should be automatically updated
    /// Setting this to true will make the character not update its omission mode
    pub const fn skip_omission_mode_updates(&self) -> bool {
        self.0[0] & 0b00000001 != 0
    }

    pub fn set_no_gravity(&mut self, val: bool) {
        self.0[0] = self.0[0] & 0b11011111 | (val as u8) << 5;
    }
    pub const fn no_gravity(&self) -> bool {
        self.0[0] & 0b00100000 != 0
    }

    pub fn set_enable_render(&mut self, val: bool) {
        self.0[1] = self.0[1] & 0b11110111 | (val as u8) << 3;
    }
    pub const fn enable_render(&self) -> bool {
        self.0[1] & 0b00001000 != 0
    }

    pub fn set_death_flag(&mut self, val: bool) {
        self.0[1] = self.0[1] & 0b01111111 | (val as u8) << 7;
    }
    pub const fn death_flag(&self) -> bool {
        self.0[1] & 0b10000000 != 0
    }
    pub fn set_draw_tag_offscreen(&mut self, val: bool) {
        self.0[2] = self.0[2] & 0b11101111 | (val as u8) << 4;
    }
    /// This flag is used to determine if the character tag (name, hp, etc) should be
    /// rendered on the side of the screen instead of above the character.
    /// Works only on friendly characters tags, not lock on ones.
    pub const fn draw_tag_offscreen(&self) -> bool {
        self.0[2] & 0b00010000 != 0
    }

    pub fn set_trigger_falldeath_camera(&mut self, val: bool) {
        self.0[4] = self.0[4] & 0b11111011 | (val as u8) << 2;
    }
    /// This flag can only trigger death camera, not disable it.
    /// If you want to disable it, check state on ChrCam instead.
    pub const fn trigger_falldeath_camera(&self) -> bool {
        self.0[4] & 0b00000100 != 0
    }
    pub fn set_enable_character_tag(&mut self, val: bool) {
        self.0[4] = self.0[4] & 0b11101111 | (val as u8) << 4;
    }
    /// This flag controls should the character tag (name, hp, etc) be rendered or not.
    pub const fn enable_character_tag(&self) -> bool {
        self.0[4] & 0b00010000 != 0
    }
}

#[repr(C)]
pub struct ChrInsModuleContainer {
    pub data: OwnedPtr<CSChrDataModule>,
    action_flag: usize,
    behavior_script: usize,
    pub time_act: OwnedPtr<CSChrTimeActModule>,
    resist: usize,
    pub behavior: OwnedPtr<CSChrBehaviorModule>,
    behavior_sync: usize,
    ai: usize,
    pub super_armor: OwnedPtr<CSChrSuperArmorModule>,
    pub toughness: OwnedPtr<CSChrToughnessModule>,
    talk: usize,
    pub event: OwnedPtr<CSChrEventModule>,
    magic: usize,
    /// Describes the characters physics-related properties.
    pub physics: OwnedPtr<CSChrPhysicsModule>,
    fall: usize,
    ladder: usize,
    action_request: usize,
    pub throw: OwnedPtr<CSChrThrowModule>,
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
pub struct CSChrPhysicsModule {
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
pub struct CSChrTimeActModuleAnim {
    pub anim_id: i32,
    pub play_time: f32,
    play_time2: f32,
    pub anim_length: f32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrTimeActModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    hvk_anim: usize,
    chr_tae_anim_event: usize,
    /// Circular buffer of animations to play.
    pub anim_queue: [CSChrTimeActModuleAnim; 10],
    /// Index of the next animation to play or update.
    pub write_idx: u32,
    /// Index of the last animation played or updated.
    pub read_idx: u32,
    unkc8: u32,
    unkcc: u32,
    unkd0: u32,
    unkd4: u32,
}

#[repr(C)]
pub struct CSChrBehaviorModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: usize,
    unk18: usize,
    unk20: usize,
    unk28: usize,
    pub root_motion: FSVector4,
    unk40: [u8; 0x20],
    unk60: [u8; 0xa48],
    unkaa8: [u8; 0x58],
    unkb00: [u8; 0xa48],
    unk1548: [u8; 0x68],
    unk15b0: FD4Time,
    unk15c0: [u8; 0xc0],
    pub ground_touch_state: u32,
    unk1684: f32,
    unk1688: f32,
    unk168c: [u8; 0x104],
    unk1790: FSVector4,
    unk17a0: [u8; 0x10],
    chr_behavior_debug_anim_helper: usize,
    unk17b8: [u8; 0x10],
    pub animation_speed: f32,
    unk17cc: [u8; 0x1f4],
}

#[repr(C)]
pub struct CSChrEventModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    unk10: [u8; 0x8],
    /// Id of override animation that should be played on next frame.
    pub request_animation_id: i32,
    /// ID of default idle animation.
    pub idle_anim_id: i32,
    unk20: i32,
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

#[repr(u32)]
pub enum ThrowNodeState {
    Unk1 = 1,
    Unk2 = 2,
    InThrowAttacker = 3,
    InThrowTarget = 4,
    DeathAttacker = 5,
    DeathTarget = 6,
    Unk7 = 7,
    Unk8 = 8,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSThrowNode {
    pub super_pair_anim_node: CSPairAnimNode,
    unk58: [u8; 0x18],
    pub throw_state: ThrowNodeState,
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
    unk38: [u8; 0x88],
    hkxpwv_res_cap: usize,
    unkc8: usize,
    hover_warp_ctrl: usize,
    ai_jump_move_ctrl: usize,
    chr_model_pos_easing: usize,
    unke8: [u8; 0x8],
    pub flags: ChrCtrlFlags,
    unkf4: [u8; 0xc],
    unk100: FSVector4,
    unk110: FSVector4,
    unk120: [u8; 0x8],
    pub chr_ragdoll_state: u8,
    unk12c: f32,
    unk130: [u8; 0x48],
    walk_twist: usize,
    unk180: usize,
    unk188: [u8; 0x4],
    pub weight_type: u32,
    unk190: [u8; 0x10],
    /// Offset from the character's dmypoly for the tag position (name, hp, etc).
    /// Will modify position of the resulting tag.
    pub lock_on_chr_tag_dmypoly_offset: FSVector4,
    unk1b0: [u8; 0x80],
    pub model_matrix: FSMatrix4x4,
    unk270: [u8; 0x40],
    unk2b0: FSVector4,
    unk2c0: FSVector4,
    unk2d0: [u8; 0x4],
    pub scale_size_x: f32,
    pub scale_size_y: f32,
    pub scale_size_z: f32,
    pub offset_y: f32,
    unk2e4: [u8; 0x14],
    unk2f8: usize,
    unk300: [u8; 0x28],
    /// Should the character match undulation of the map?
    /// Fetched from NpcParam
    pub is_undulation: bool,
    /// Should FootIK be used for undulation correction?
    pub use_ik_normal_by_undulation: bool,
    unk32a: [u8; 0x2],
    /// Forward undulation correction angle.
    /// Fetched from NpcParam
    pub forward_undulation_limit_radians: f32,
    /// Backward undulation correction angle.
    /// Fetched from NpcParam
    pub backward_undulation_limit_radians: f32,
    /// Side undulation correction angle.
    /// Fetched from NpcParam
    pub side_undulation: f32,
    /// Speed of undulation correction.
    /// Fetched from NpcParam
    pub undulation_correction_gain: f32,
    unk33c: [u8; 0x14],
    unk350: FSVector4,
    unk360: FSVector4,
    unk370: [u8; 0x10],
    unk380: FSVector4,
    unk390: [u8; 0x19],
    /// Group, deciding how character will collide with other characters.
    /// Fetched from NpcParam
    pub hit_group_and_navimesh: u8,
    hit_group_and_navimesh_unk: u8,
    unk3ab: [u8; 0x5],
    unk3b0: usize,
    unk3b8: [u8; 0x18],
}

#[repr(C)]
pub struct ChrCtrlFlags([u8; 4]);

impl ChrCtrlFlags {
    // byte 0 bit 0 Disable player collision
    // byte 0 bit 1 Disable hit
    // byte 0 bit 2-4 and 6 Disable map collision
    pub fn set_disable_player_collision(&mut self, val: bool) {
        self.0[0] = self.0[0] & 0b11111110 | val as u8;
    }
    pub const fn disable_player_collision(&self) -> bool {
        self.0[0] & 0b00000001 != 0
    }

    pub fn set_disable_hit(&mut self, val: bool) {
        self.0[0] = self.0[0] & 0b11111101 | (val as u8) << 1;
    }
    pub const fn disable_hit(&self) -> bool {
        self.0[0] & 0b00000010 != 0
    }

    pub fn set_disable_map_collision(&mut self, val: bool) {
        self.0[0] = self.0[0] & 0b11111011 | (val as u8) << 2;
    }
    pub const fn disable_map_collision(&self) -> bool {
        self.0[0] & 0b00000100 != 0
    }
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
    unk650: [u8; 0x28],
    /// Set on player spawn and maybe on arena respawn?
    /// Players cannot be hurt if this is above 0.
    pub invincibility_timer_for_net_player: f32,
    unk67c: [u8; 0x34],
    pub locked_on_enemy: FieldInsHandle,
    pub session_manager_player_entry: OwnedPtr<SessionManagerPlayerEntryBase>,
    /// Position within the current block.
    pub block_position: BlockPosition,
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

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerSessionHolder {
    vftable: usize,
    player_debug_session: usize,
    unk10: usize,
    pub player_network_session: OwnedPtr<PlayerNetworkSession>,
    unk18: usize,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Role of character in PvP/PvE.
/// Changes a lot of things, like appearance, what items you can use, etc.
pub enum ChrType {
    None = -1,
    Local = 0,
    WhitePhantom = 1,
    BlackPhantom = 2,
    Ghost = 3,
    Ghost1 = 4,
    Npc = 5,
    GrayPhantom = 8,
    Arena = 13,
    Quickmatch = 14,
    Invader = 15,
    Invader2 = 16,
    BluePhantom = 17,
    Invader3 = 18,
}
