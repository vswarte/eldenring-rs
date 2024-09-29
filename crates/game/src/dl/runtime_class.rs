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
    pub allocator1: usize,
    pub allocator2: usize,

    // This is me praying these are always laid out the same way since these
    // two fields are not part of the DLRuntimeClass strictly.
    pub class_name: windows::core::PCSTR,
    pub class_name_wide: windows::core::PWSTR,
}
