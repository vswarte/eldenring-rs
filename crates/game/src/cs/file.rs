use std::ffi;

use crate::dl::DLPlainLightMutex;
use crate::fd4::{
    FD4BasicHashString, FD4ResCap, FD4ResCapHolder
};

#[repr(C)]
pub struct CSFile<'a> {
    vftable: usize,
    pub file_repository_1: &'a CSFileRepository<'a>,
    // TODO: Incomplete..
}

#[repr(C)]
pub struct CSFileRepository<'a> {
    // TODO: This is actually embedding an FD4FileRepository of size 0x210
    pub repository_res_cap: FD4ResCap<[u8; 0x10]>,
    pub holder1: FD4ResCapHolder<()>,
    pub holder2: FD4ResCapHolder<()>,

    // Some type of btree?
    unkc8_allocator: usize,
    unkd0_tree_pointer: usize,
    unkd8_tree_size: u32,
    unkdc_tree_pad: u32,
    pub mutexes: [&'a CSFileRepositoryMutex; 5],
    unk108: usize,
    unk110: usize,
    unk118: usize,
    unk120: usize,
    unk128: usize,
}

#[repr(C)]
pub struct CSFileRepositoryMutex {
    pub mutex: DLPlainLightMutex,
    unk30: u32,
    unk34: u32,
    unk38: u32,
    unk3c: u32,
    unk40: usize,
    unk48: usize,
}
