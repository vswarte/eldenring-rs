use crate::cs::ChrIns;
use crate::pointer::OwningPtr;
use crate::DLRFLocatable;

#[repr(C)]
pub struct WorldChrManDbg {
    pub unk0: [u8; 0xa8],
    pub manipulator: usize,
    pub player_session_holder: usize,
    pub cam_override_chr_ins: Option<OwningPtr<ChrIns>>,
}

impl DLRFLocatable for WorldChrManDbg {
    const DLRF_NAME: &'static str = "WorldChrManDbg";
}
