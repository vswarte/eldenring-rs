use std::ffi;

use windows::Win32::System::Threading::CRITICAL_SECTION;

#[repr(C)]
/// Source of name: RTTI
pub struct DLPlainLightMutex {
    vftable: usize,
    pub critical_section: CRITICAL_SECTION,
}
