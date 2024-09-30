use std::ffi;

use crate::{cs::ChrSetEntry, matrix::Vector4};

use super::MapId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WhoID {
    pub map_id: i32,
    pub chr_selector: i32,
}

#[repr(C)]
pub struct FieldInsHandle {
    pub instance_id: i32,
    pub map_id: MapId,
}

#[repr(C)]
pub struct AtkParamLookupResult {
    behavior_param_id: u32,
    unk_param_def_meta: u32,
    is_player_atk_param: bool,
    _pad9: [u8; 7],
    param_row: usize,
}

#[repr(C)]
pub struct ChrInsVMT {
    // Part of FieldInsBase, retrieves reflection metadata for FD4Component derivants.
    pub get_runtime_metadata: fn(&ChrIns) -> usize,
    // Destructor
    pub destructor: fn(&ChrIns, u32) -> usize,
    // Part of FieldInsBase, ChrIns = 1, CSBulletIns = 3, CSWorldGeomIns = 6, MapIns = 7, CSWorldGeomHitIns = 8, 
    pub get_field_ins_type: fn(&ChrIns) -> u32,
    // Part of FieldInsBase.
    pub use_npc_atk_param: fn(&ChrIns) -> bool,
    // Part of FieldInsBase
    pub get_atk_param_for_behavior: fn(&ChrIns, u32, &mut AtkParamLookupResult) -> u32,
    // Part of FieldInsBase. ChrIns = 0, PlayerIns = 1, EnemyIns = 0, ReplayGhostIns = 1,
    // ReplayEnemyIns = 0, CSBulletIns = 0, CSWorldGeomIns = 0, CSFieldInsBase = 0,
    // CSHamariSimulateChrIns = 0, MapIns = 0, HitIns = 0, CSWorldGeomStaticIns = 0, HitInsBase =
    // 0, CSWorldGeomHitIns = 0, CSWorldGeomDynamicIns = 0, 
    pub use_player_behavior_param: fn(&ChrIns) -> bool,
    // Obfuscated beyond recognition
    pub unk6: fn(&ChrIns),
    // Obfuscated beyond recognition
    pub unk7: fn(&ChrIns),
}

#[repr(C)]
pub struct ChrIns<'a> {
    pub vftable: &'a ChrInsVMT,
    pub field_ins_handle: FieldInsHandle,
    chr_set_entry: usize,
    pub unk18: usize,
    pub unk20: u32,
    pub unk24: u32,
    pub chr_res: usize,
    pub map_id_1: MapId,
    pub map_id_origin_1: i32,
    pub map_id_2: MapId,
    pub map_id_origin_2: i32,
    pub chr_set_cleanup: u32,
    _pad44: u32,
    pub unk48: usize,
    pub chr_model_ins: &'a mut ChrCtrl<'a>,
    pub chr_ctrl: &'a mut ChrCtrl<'a>,
    pub think_param_id: i32,
    pub npc_id_1: i32,
    pub chr_type: i32,
    pub team_type: i32,
    pub who_id: WhoID,
    pub unk78: usize,
    pub unk80_position: Vector4,
    pub unk90_position: Vector4,
    pub unka0_position: Vector4,
    pub chr_update_delta_time: f32,
    pub render_distance: u32,
    pub frames_per_update: u32,
    pub render_visibility: u32,
    pub target_velocity_recorder: usize,
    pub unkc8: usize,
    pub unkd0_position: usize,
    pub unkd8: [u8; 0x88],
    pub last_used_item: i16,
    pub unk162: i16,
    pub unk164: u32,
    pub unk168: u32,
    pub unk16c: u32,
    pub unk170: u32,
    pub unk174: u32,
    pub special_effect: usize,
    pub unk180: usize,
    pub character_id: u32,
    pub unk184: u32,
    pub module_container: &'a mut ChrInsModuleContainer<'a>,
    pub rest: [u8; 0x3E8], 
}

#[repr(C)]
pub struct ChrInsModuleContainer<'a> {
    pub data: usize,
    pub action_flag: usize,
    pub behavior_script: usize,
    pub time_act: usize,
    pub resist: usize,
    pub behavior: usize,
    pub behavior_sync: usize,
    pub ai: usize,
    pub super_armor: usize,
    pub toughness: usize,
    pub talk: usize,
    pub event: usize,
    pub magic: usize,
    pub physics: &'a ChrPhysicsModule<'a>,
    pub fall: usize,
    pub ladder: usize,
    pub action_request: usize,
    pub throw: usize,
    pub hitstop: usize,
    pub damage: usize,
    pub material: usize,
    pub knockback: usize,
    pub sfx: usize,
    pub vfx: usize,
    pub behavior_data: usize,
    pub unkc8: usize,
    pub model_param_modifier: usize,
    pub dripping: usize,
    pub unke0: usize,
    pub ride: usize,
    pub bonemove: usize,
    pub wet: usize,
    pub auto_homing: usize,
    pub above_shadow_test: usize,
    pub sword_arts: usize,
    pub grass_hit: usize,
    pub wheel_rot: usize,
    pub cliff_wind: usize,
    pub navimesh_cost_effect: usize,
}

#[repr(C)]
pub struct ChrPhysicsModule<'a> {
    pub vftable: usize,
    pub owner: &'a mut ChrIns<'a>,
    pub unk10: [u8; 0x40],
    pub unk50_orientation: Vector4,
    pub unk60_orientation: Vector4,
    pub unk70_position: Vector4,
    pub unk80_position: Vector4,
    pub unk90: bool,
    pub unk91: bool,
    pub unk92: bool,
    pub unk93: bool,
}

#[repr(C)]
pub struct ChrCtrl<'a> {
    pub vftable: usize,
    unk8: u64,
    pub owner: &'a ChrIns<'a>,
    pub manipulator: usize,
    unk20: usize,
    ragdoll_ins: usize,
    chr_collision: usize,
    unk38: [u8; 240],
    pub chr_ragdoll_state: u8,
}

#[repr(C)]
pub struct PlayerIns<'a> {
    pub chr_ins: ChrIns<'a>,
    pub player_game_data: &'a PlayerGameData,
    pub chr_manipulator: usize,
    pub unk590: usize,
    pub player_session_holder: usize,
    pub unk5c0: usize,
    pub replay_recorder: usize,
    pub unk5b0: [u8; 0x88],
    pub chr_asm: &'a mut ChrAsm,
    pub chr_asm_model_res: usize,
    pub chr_asm_model_ins: usize,
    pub wtf: [u8; 0x60],
    pub locked_on_enemy_field_ins_handle: FieldInsHandle,
    pub session_manager_player_entry: usize,
    pub map_relative_position: Vector4,
}

#[repr(C)]
pub struct PlayerGameData {
    pub vfptr: usize,
    pub character_type: u32,
    pub unkc: u32,
    pub current_hp: u32,
    pub current_max_hp: u32,
    pub base_max_hp: u32,
    pub current_fp: u32,
    pub current_max_fp: u32,
    pub base_max_fp: u32,
    pub unk28: f32,
    pub current_stamina: u32,
    pub current_max_stamina: u32,
    pub base_max_stamina: u32,
    pub unk38: f32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
    pub unk5c: f32,
    pub unk60: f32,
    pub unk64: f32,
    pub level: u32,
    pub rune_count: u32,
    pub rune_memory: u32,
    pub unk74: u32,
    pub poison_resist: u32,
    pub rot_resist: u32,
    pub bleed_resist: u32,
    pub death_resist: u32,
    pub frost_resist: u32,
    pub sleep_resist: u32,
    pub madness_resist: u32,
    pub unk94: u32,
    pub unk98: u32,
    character_name: [u16; 16],
    pub unkbc: u8,
    pub unkbd: u8,
    pub gender: u8,
    pub archetype: u8,
    pub vow_type: u8,
    pub unkc1: u8,
    pub voice_type: u8,
    pub starting_gift: u8,
    pub unkc4: u8,
    pub unlocked_magic_slots: u8,
    pub unkc6: [u8; 0x19],
    pub furlcalling_finger_remedy_active: u8,
    pub unke0: u8,
    pub unke1: u8,
    pub matching_weapon_level: u8,
    pub white_ring_active: u8,
    pub blue_ring_active: u8,
    pub unke5: [u8; 0x7],
    pub unkec: u32,
    pub unkf0: [u8; 0x4],
    pub solo_breakin_point: u32,
    pub unkf8: [u8; 0x7],
    pub rune_arc_active: u8,
    pub unk100: u8,
    pub max_hp_flask: u8,
    pub max_fp_flask: u8,
    pub unk103: [u8; 0x6],
    pub reached_max_rune_memory: u8,
    pub unk10a: [u8; 0xE],
    pub password: [u16; 0x8],
    pub unk128: u16,
    group_password_1: [u16; 0x8],
    _unk13a: u16,
    group_password_2: [u16; 0x8],
    _unk14c: u16,
    group_password_3: [u16; 0x8],
    _unk15e: u16,
    group_password_4: [u16; 0x8],
    _unk170: u16,
    group_password_5: [u16; 0x8],
    _unk182: u16,
    pub unk184: [u8; 0x34],
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
    pub unk2ac: u32,
    pub equip_game_data: [u8; 0x4b0],
    pub face_data: [u8; 0x170],
    pub equip_inventory_data: usize,
    pub gesture_game_data: usize,
    pub ride_game_data: usize,
    pub unk8e8: usize,
    _unk8f0: [u8; 0x10],
    pub unk900: u32,
    pub hp_estus_rate: f32,
    pub hp_estus_additional: u8,
    _pad909: [u8; 3],
    pub fp_estus_rate: f32,
    pub fp_estus_additional: u8,
    _pad911: [u8; 3],
    pub unk914: [u8; 0x164],
    pub menu_ref_special_effect_1: usize,
    pub menu_ref_special_effect_2: usize,
    pub menu_ref_special_effect_3: usize,
    pub unka90: [u8; 0x58],
}

#[repr(C)]
pub struct PlayerGameDataSpEffect {
    pub sp_effect_id: u32,
    pub duration: f32,
    pub unk8: u32,
    pub unkc: u32,
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
