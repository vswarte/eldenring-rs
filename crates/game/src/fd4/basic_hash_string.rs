use std::{ffi, fmt::Display};

use crate::dltx::DLBasicString;

#[repr(C)]
/// Wraps a string to make it easier to use with hashmaps. Seemingly mostly used in the resource
/// system but has some usage elsewhere too.
///
/// Source of name: RTTI
pub struct FD4BasicHashString {
    vftable: usize,
    allocator: usize,
    /// The contained string we're hashing for.
    inner: DLBasicString,
    /// Hashed representation of the string field.
    pub hash: u32,
    /// Indicates whether or not the hash field is populated.
    pub needs_hashing: u8,
    _pad35: [u8; 0xB],
}

impl AsRef<DLBasicString> for FD4BasicHashString {
    fn as_ref(&self) -> &DLBasicString {
        &self.inner
    }
}

impl Display for FD4BasicHashString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod test {
    use crate::fd4::FD4BasicHashString;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x40, size_of::<FD4BasicHashString>());
    }
}
