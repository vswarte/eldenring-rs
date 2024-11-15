use std::ptr::NonNull;

use crate::pointer::OwningPtr;

use super::FD4ResRep;
use super::resource::FD4ResCap;

#[repr(C)]
#[dlrf::singleton("FD4ParamRepository")]
pub struct FD4ParamRepository {
    /// Resource repository holding the actual param data.
    pub res_rep: FD4ResRep<FD4ParamResCapBody>,
    allocator: usize,
    test: FD4ResCap<FD4ParamResCapBody>,
}

#[repr(C)]
pub struct FD4ParamResCapBody {
    /// Size of data at pointer.
    pub size: u64,
    /// Raw row data for this param file.
    pub data: Option<OwningPtr<ParamFileHeader>>,
}

pub struct ParamFileHeader {
    name_offset: u32,
    data_offset: u16,
    unk6: u16,
    pub paramdef_version: u16,
    pub row_count: u16,
}
