use crate::cs::ChrIns;
use std::ptr::NonNull;

use super::PlayerSessionHolder;

#[repr(C)]
#[dlrf::singleton("WorldChrManDbg")]
pub struct WorldChrManDbg {
    unk0: [u8; 0xa8],
    pub debug_manipulator: usize,
    pub player_session_holder: Option<NonNull<PlayerSessionHolder>>,
    pub cam_override_chr_ins: Option<NonNull<ChrIns>>,
}
