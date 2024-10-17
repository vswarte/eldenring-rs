use std::ffi;

use crate::dl::DLWString;

#[repr(C)]
/// Seems to be used in some control flow around engine settings and debug options.
///
/// Source of name: RTTI
pub struct CSEzSelectBot {
    pub vftable: *const ffi::c_void,
    pub property: CSEzSelectBotString,
}

#[repr(C)]
pub struct CSEzSelectBotString {
    pub allocator: *const ffi::c_void,
    pub string: DLWString,
    pub unk28: u64,
}
