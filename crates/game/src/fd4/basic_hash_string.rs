use std::{ffi, fmt::Display};

use crate::dl::DLWString;

#[repr(C)]
pub struct FD4BasicHashString {
    pub vftable: usize,
    pub allocator: usize,
    pub string: DLWString,
    pub unk1: usize,
    pub hash: u32,
    pub needs_hashing: u8,
    pub pad: [u8; 7],
}

impl Display for FD4BasicHashString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string.to_string())
    }
}
