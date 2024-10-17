use std::{ffi, fmt::Display};

use crate::dl::DLWString;

#[repr(C)]
/// Wraps a string to make it easier to use with hashmaps. Seemingly mostly used in the resource
/// system but has some usage elsewhere too.
///
/// Source of name: RTTI
pub struct FD4BasicHashString {
    pub vftable: usize,
    pub allocator: usize,
    /// The contained string we're hashing for.
    pub string: DLWString,
    /// Hashed representation of the string field.
    pub hash: u32,
    /// Indicates whether or not the hash field is populated.
    pub needs_hashing: u8,
    _pad35: [u8; 7],
}

impl Display for FD4BasicHashString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string.to_string())
    }
}
