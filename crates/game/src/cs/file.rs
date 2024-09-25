use std::ffi;

use crate::dl::DLPlainLightMutex;
use crate::fd4::{
    FD4BasicHashString, FD4ResCap, FD4ResCapHolder
};

#[repr(C)]
pub struct CSFile<'a> {
    pub vftable: usize,
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
    pub unkc8_allocator: usize,
    pub unkd0_tree_pointer: usize,
    pub unkd8_tree_size: u32,
    pub unkdc_tree_pad: u32,

    pub mutexes: [&'a CSFileRepositoryMutex; 5],
    pub unk108: usize,
    pub unk110: usize,
    pub unk118: usize,
    pub unk120: usize,
    pub unk128: usize,
}

#[repr(C)]
pub struct CSFileRepositoryMutex {
    pub mutex: DLPlainLightMutex,
    pub unk30: u32,
    pub unk34: u32,
    pub unk38: u32,
    pub unk3c: u32,
    pub unk40: usize,
    pub unk48: usize,
}
