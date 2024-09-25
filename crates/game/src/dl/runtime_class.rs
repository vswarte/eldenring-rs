use std::ffi;

#[repr(C)]
pub struct DLRuntimeClass {
    pub vftable: *const ffi::c_void,
    pub base_class: *const DLRuntimeClass,
    pub unk10: usize,
    pub unk18: usize,
    pub unk20: usize,
    pub unk28: usize,
    pub unk30: usize,
    pub class_name: windows::core::PCSTR,
    pub class_name_wide: windows::core::PWSTR,
}
