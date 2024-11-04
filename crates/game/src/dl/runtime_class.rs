use std::ffi;

#[repr(C)]
/// Part of the DLRF namespace, describes some aspects of a tracked class.
///
/// Source of name: RTTI
pub struct DLRuntimeClass {
    vftable: *const ffi::c_void,
    pub base_class: *const DLRuntimeClass,
    pub unk10: usize,
    pub unk18: usize,
    pub unk20: usize,
    pub unk28: usize,
    pub unk30: usize,
    pub allocator1: usize,
    pub allocator2: usize,
}
