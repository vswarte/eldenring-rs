use std::ffi;

use crate::pointer::OwningPtr;

#[repr(C)]
/// Part of the DLRF namespace, describes some aspects of a tracked class.
///
/// Source of name: RTTI
pub struct DLRuntimeClass {
    vftable: *const ffi::c_void,
    pub base_class: OwningPtr<DLRuntimeClass>,
    unk10: usize,
    unk18: usize,
    unk20: usize,
    unk28: usize,
    unk30: usize,
    allocator1: usize,
    allocator2: usize,
}
