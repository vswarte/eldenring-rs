use std::ffi;

use crate::{cs::ChrSetEntry, matrix::Vector4};

use super::MapId;

#[repr(C)]
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
pub struct ChrInsVftable {
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
    pub vftable: usize,
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
    pub rest: [u8; 0x3D8], 
}

#[repr(C)]
pub struct PlayerIns<'a> {
    pub chr_ins: ChrIns<'a>,
    pub unk570: usize,
    pub unk578: usize,
    pub player_game_data: usize,
    pub chr_manipulator: usize,
    pub unk590: usize,
    pub player_session_holder: usize,
    pub unk5c0: usize,
    pub replay_recorder: usize,
    pub unk5b0: [u8; 0x88],
    pub chr_asm: usize,
    pub chr_asm_model_res: usize,
    pub chr_asm_model_ins: usize,
    pub wtf: [u8; 0x60],
    pub locked_on_enemy_field_ins_handle: FieldInsHandle,
    pub session_manager_player_entry: usize,
    pub map_relative_position: Vector4,
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
