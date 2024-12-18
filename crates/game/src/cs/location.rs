use std::ptr::NonNull;

use vtable_rs::VPtr;
use windows::core::PSTR;

use crate::dlut::DLReferenceCountObjectVmt;

#[vtable_rs::vtable]
pub trait CSFD4LocationPoseVmt : DLReferenceCountObjectVmt {
    fn unk2(&self);

    fn unk3(&self);

    /// Seemingly updates the location pose (eg: updates to snap to skeleton, etc)
    fn update(&mut self);

    fn unk5(&self);

    fn unk6(&self);

    fn unk7(&self);

    fn unk8(&self);

    /// Returns name of location pose specialization.
    fn type_name(&self) -> PSTR;

    fn unk10(&self);
}

/// Tracks the amount of references for 
///
/// Source of name: RTTI
#[repr(C)]
pub struct CSFD4LocationPose {
    pub vftable: VPtr<dyn CSFD4LocationPoseVmt, Self>,
    pub reference_count: u32,
    _padc: u32,
    flags: u32,
    atomic_lock: u32,
    pub node: CSFD4LocationNode,
}

#[repr(C)]
pub struct CSFD4LocationNode {
    pub owner: NonNull<CSFD4LocationPose>,
    pub prev: Option<NonNull<CSFD4LocationNode>>,
    pub next: NonNull<CSFD4LocationNode>,
    unk18: NonNull<CSFD4LocationNode>,
}
