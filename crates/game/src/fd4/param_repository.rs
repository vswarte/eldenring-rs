use crate::{fd4::resource, DLRFLocatable};

use super::resource::{FD4ResCap, FD4ResCapHolder};

#[repr(C)]
pub struct FD4ParamRepository {
    pub res_cap: FD4ResCap<u64>,
    pub res_cap_holder: FD4ResCapHolder<()>,
    pub allocator: usize,
}

impl DLRFLocatable for FD4ParamRepository {
    const DLRF_NAME: &'static str = "FD4ParamRepository";
}
