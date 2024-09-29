use crate::{fd4::resource, DLRFLocatable};

use super::resource::{FD4ResCap, FD4ResCapHolder};

#[repr(C)]
pub struct FD4ParamRepository {
    pub res_cap: FD4ResCap<()>,
    pub unk68: [u8; 0x10],
    pub res_cap_holder: FD4ResCapHolder<FD4ParamResCapBody>,
    pub allocator: usize,
    pub test: FD4ResCap<FD4ParamResCapBody>,
}

impl DLRFLocatable for FD4ParamRepository {
    const DLRF_NAME: &'static str = "FD4ParamRepository";
}

#[repr(C)]
pub struct FD4ParamResCapBody {
    pub unk0: [u8; 0x10],
    pub file_size: u64,
    pub header: *const ParamFileHeader,
    pub unk28: usize,
    pub allocator: usize,
    pub unk38: [u8; 0x10],
}

pub struct ParamFileHeader {
    name_offset: u32,
    data_offset: u16,
    _unk6: u16,
    pub paramdef_version: u16,
    pub row_count: u16,
}
