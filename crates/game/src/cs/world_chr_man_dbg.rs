use crate::cs::ChrIns;
use std::ptr::NonNull;

#[repr(C)]
#[dlrf::singleton("WorldChrManDbg")]
pub struct WorldChrManDbg {
    pub unk0: [u8; 0xa8],
    pub debug_manipulator: usize,
    pub player_session_holder: usize,
    pub cam_override_chr_ins: Option<NonNull<ChrIns>>,
}
