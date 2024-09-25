use std::ffi;

use windows::Win32::System::Threading::CRITICAL_SECTION;

#[repr(C)]
pub struct DLPlainLightMutex {
    pub vftable: usize,
    pub critical_section: CRITICAL_SECTION,
}
