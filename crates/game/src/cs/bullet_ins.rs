use std::ptr::NonNull;
use std::ffi::OsStr;
use vtable_rs::VPtr;
use crate::matrix::FSVector4;
use crate::position::HavokPosition;
use crate::rotation::Quaternion;

use super::{FieldInsBaseVmt, FieldInsHandle, CSBulletTargetingSystemOwner, CSTargetingSystemBase, };

pub struct BulletParamLookupResult {
    pub param_row: usize,
    pub row_id: u32,
    version: u8,
    _padd: [u8; 3],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBulletIns {
    /// No new virtual functions
    pub vftable: VPtr<dyn FieldInsBaseVmt, Self>,
    pub field_ins_handle: FieldInsHandle,
    pub physics: BulletPhysics,
    unk50: u32,
    unk54: u32,
    unk58: u32,
    unk5c: u32,
    sfx_ctrl: [u8; 0x420],
    unk480: u32,
    unk484: u32,
    unk488_sound_type: u32,
    unk48c_sound_type: i32,
    unk490_sound_type: i32,
    unk494_sound_type: i32,
    unk498_sound_type_ptr: usize,
    unk4a0: u32,
    unk4a4: u32,
    unk4a8: u32,
    unk4ac: u32,
    unk4b0_struct: [u8; 0x660],
    pub time_alive: f32,
    pub life: f32,
    pub life_max: f32,
    pub accel_time: f32,
    pub create_interval_time_left: f32,
    pub create_time_timer: f32,
    unkb28: u32,
    unkb2c: u32,
    pub targeting_owner: CSBulletTargetingSystemOwner,
    pub targeting_system: CSTargetingSystemBase,
    unkd30_struct: [u8; 0x50],
    unkd80: u8,
    unkd81: u8,
    unkd82: u8,
    unkd83: u8,
    unkd84: u8,
    unkd85: u8,
    unkd86: u8,
    unkd87: u8,
    unkd88: i32,
    pub wait_state: CSBulletWaitState,
    pub fly_state: CSBulletFlyState,
    pub exp_state: CSBulletExplosionState,
    pub next_bullet: Option<NonNull<CSBulletIns>>,
    unke20: u64,
    unke28: u64,
}

#[repr(C)]
pub struct BulletPhysics {
    pub position: HavokPosition,
    pub orientation: Quaternion,
    pub velocity: FSVector4,
    pub velocity2: FSVector4,
}

#[vtable_rs::vtable]
pub trait CSBulletStateVmt {
    fn destructor(&mut self, should_free: bool);

    fn set_bullet_param_and_get_hit_bullet(&mut self, row_id: u32) -> i32;

    fn on_update(&mut self, bullet: &mut CSBulletIns, dt: f32);

    fn on_creation(&mut self, bullet: &mut CSBulletIns);

    fn on_death(&mut self, bullet: &mut CSBulletIns);

    fn unk28(&mut self, param_row: usize, param_3: usize);

    fn unk30(&self) -> bool;

    fn unk38(&self) -> bool;

    fn unk40(&self) -> bool;

    fn get_state_label(&self) -> &OsStr;
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBulletState {
    pub vftable: VPtr<dyn CSBulletStateVmt, Self>,
    pub param: BulletParamLookupResult,
    unk18: f32,
    unk1c: f32,
    unk20: u32,
    unk24: u32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBulletWaitState {
    pub base: CSBulletState,
    unk28: u32,
    _pad2c: [u8; 4]
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBulletFlyState {
    pub base: CSBulletState,
    unk28: u16,
    unk2a: u8,
    _pad2c: [u8; 5],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBulletExplosionState {
    pub base: CSBulletState,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBulletNULLState {
    pub base: CSBulletState,
}
