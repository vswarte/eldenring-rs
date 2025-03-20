use std::ptr::NonNull;

use crate::pointer::OwnedPtr;

use super::resource::FD4ResCap;
use super::FD4ResRep;

#[repr(C)]
#[dlrf::singleton("FD4ParamRepository")]
pub struct FD4ParamRepository {
    /// Resource repository holding the actual param data.
    pub res_rep: FD4ResRep<FD4ParamResCap>,
    allocator: usize,
}

#[repr(C)]
pub struct FD4ParamResCap {
    pub res_cap: FD4ResCap<Self>,

    /// Size of data at pointer.
    pub size: u64,
    /// Raw row data for this param file.
    pub data: Option<OwnedPtr<ParamFileHeader>>,
}

impl AsRef<FD4ResCap<Self>> for FD4ParamResCap {
    fn as_ref(&self) -> &FD4ResCap<Self> {
        &self.res_cap
    }
}

pub struct ParamFileHeader {
    name_offset: u32,
    data_offset: u16,
    unk6: u16,
    pub paramdef_version: u16,
    pub row_count: u16,
}
