use crate::matrix::FSMatrix4x4;
use crate::pointer::OwningPtr;

#[repr(C)]
/// Source of name: RTTI
#[dlrf::singleton("CSCamera")]
pub struct CSCamera {
    vftable: usize,
    pub pers_cam_1: OwningPtr<CSPersCam>,
    pub pers_cam_2: OwningPtr<CSPersCam>,
    pub pers_cam_3: OwningPtr<CSPersCam>,
    pub pers_cam_4: OwningPtr<CSPersCam>,

    // 0b00100000 // Copy from pers_cam_4 into pers_cam_1
    // 0b00010000 // Copy from pers_cam_3 into pers_cam_1
    // 0b00001000 // Copy from pers_cam_2 into pers_cam_1
    // 0b00000100 // Copy from pers_cam_4 into pers_cam_1
    // 0b00000010 // Copy from pers_cam_4 into pers_cam_1
    // 0b00000001 // Copy from pers_cam_2 into pers_cam_1
    pub camera_mask: u32,

    unk2c: u32,
    unk30: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSCam {
    vftable: usize,
    unk8: u32,
    unkc: u32,
    pub matrix: FSMatrix4x4,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

pub type CSPersCam = CSCam;
