use crate::{fd4::resource, DLRFLocatable};

use super::resource::{FD4ResCap, FD4ResCapHolder};

#[repr(C)]
pub struct FD4ParamRepository {
    pub res_cap: FD4ResCap<()>,
    unk68: [u8; 0x10],
    pub res_cap_holder: FD4ResCapHolder<FD4ParamResCapBody>,
    allocator: usize,
    pub test: FD4ResCap<FD4ParamResCapBody>,
}

impl DLRFLocatable for FD4ParamRepository {
    const DLRF_NAME: &'static str = "FD4ParamRepository";
}

#[repr(C)]
pub struct FD4ParamResCapBody {
    unk0: [u8; 0x10],
    pub file_size: u64,
    pub header: *const ParamFileHeader,
    unk28: usize,
    pub allocator: usize,
    unk38: [u8; 0x10],
}

pub struct ParamFileHeader {
    name_offset: u32,
    data_offset: u16,
    unk6: u16,
    pub paramdef_version: u16,
    pub row_count: u16,
}
