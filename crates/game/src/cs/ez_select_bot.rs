use std::ffi;

use crate::dltx::DLWString;

#[repr(C)]
/// Seems to be used in some control flow around engine settings and debug options.
///
/// Source of name: RTTI
pub struct CSEzSelectBot {
    vftable: *const ffi::c_void,
    pub property: CSEzSelectBotString,
}

#[repr(C)]
pub struct CSEzSelectBotString {
    pub allocator: *const ffi::c_void,
    pub string: DLWString,
    unk28: u64,
}
