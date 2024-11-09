use crate::cs::ChrIns;
use crate::DLRFLocatable;

#[repr(C)]
pub struct WorldChrManDbg<'a> {
    pub unk0: [u8; 0xa8],
    pub manipulator: usize,
    pub player_session_holder: usize,
    pub cam_override_chr_ins: Option<&'a mut ChrIns<'a>>,
}

impl DLRFLocatable for WorldChrManDbg<'_> {
    const DLRF_NAME: &'static str = "WorldChrManDbg";
}
