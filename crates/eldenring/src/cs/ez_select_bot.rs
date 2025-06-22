use std::ffi;

use crate::dltx::DLBasicString;

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
    allocator: *const ffi::c_void,
    pub string: DLBasicString,
    unk28: u64,
}
