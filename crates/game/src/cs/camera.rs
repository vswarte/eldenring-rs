use crate::{matrix::Matrix4X4, DLRFLocatable};

#[repr(C)]
pub struct CSCamera<'a> {
    pub vftable: usize,
    pub pers_cam_1: &'a mut CSPersCam,
    pub pers_cam_2: &'a mut CSPersCam,
    pub pers_cam_3: &'a mut CSPersCam,
    pub pers_cam_4: &'a mut CSPersCam,

    // 0b00100000 // Copy from pers_cam_4 into pers_cam_1
    // 0b00010000 // Copy from pers_cam_3 into pers_cam_1
    // 0b00001000 // Copy from pers_cam_2 into pers_cam_1
    // 0b00000100 // Copy from pers_cam_4 into pers_cam_1
    // 0b00000010 // Copy from pers_cam_4 into pers_cam_1
    // 0b00000001 // Copy from pers_cam_2 into pers_cam_1
    pub camera_mask: u32,

    pub unk2c: u32,
    pub unk30: usize,
}

impl DLRFLocatable for CSCamera<'_> {
    const DLRF_NAME: &'static str = "CSCamera";
}

#[repr(C)]
pub struct CSCam {
    pub vftable: usize,
    pub unk8: u32,
    pub unkc: u32,
    pub matrix: Matrix4X4,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32,
    pub far_plane: f32,
}

pub type CSPersCam = CSCam;
