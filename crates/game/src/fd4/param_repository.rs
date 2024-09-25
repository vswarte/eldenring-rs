use crate::fd4::resource;

use super::resource::{FD4ResCap, FD4ResCapHolder};

#[repr(C)]
pub struct FD4ParamRepository<'a> {
    pub repository_res_cap: FD4ResCap<'a, [u8; 0x10]>,
    pub map: FD4ResCapHolder<'a, ()>,
}
