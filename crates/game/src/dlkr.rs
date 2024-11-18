use std::ffi;

use vtable_rs::VPtr;
use windows::Win32::System::Threading::{DeleteCriticalSection, EnterCriticalSection, InitializeCriticalSection, LeaveCriticalSection, CRITICAL_SECTION};

#[vtable_rs::vtable]
pub trait DLPlainLightMutexVmt {
    fn destructor(&mut self, param_2: bool);
}

#[repr(C)]
/// Source of name: RTTI
pub struct DLPlainLightMutex {
    pub vftable: VPtr<dyn DLPlainLightMutexVmt, Self>,
    pub critical_section: CRITICAL_SECTION,
}

impl Default for DLPlainLightMutex {
    fn default() -> Self {
        let mut ins = Self {
            vftable: Default::default(),
            critical_section: Default::default(),
        };

        unsafe { InitializeCriticalSection(&mut ins.critical_section) }

        ins
    }
}

impl Drop for DLPlainLightMutex {
    fn drop(&mut self) {
        unsafe { DeleteCriticalSection(&mut self.critical_section) }
    }
}

impl DLPlainLightMutex {
    pub fn lock(&mut self) {
        unsafe { EnterCriticalSection(&mut self.critical_section) }
    }

    pub fn unlock(&mut self) {
        unsafe { LeaveCriticalSection(&mut self.critical_section) }
    }

}

impl DLPlainLightMutexVmt for DLPlainLightMutex {
    extern "C" fn destructor(&mut self,param_2:bool) {
        tracing::warn!("Called mutex destructor");
    }
}

#[vtable_rs::vtable]
pub trait DLAllocatorVmt {
    fn destructor(&mut self, param_2: bool);

    /// Getter for the allocator ID.
    fn allocator_id(&self) -> u32;

    fn unk10(&self);

    fn heap_flags(&self) -> &u64;

    fn heap_capacity(&self) -> usize;

    fn heap_size(&self) -> usize;

    fn backing_heap_capacity(&self) -> usize;

    fn heap_allocation_count(&self) -> usize;

    /// Retrieves allocation size for a specific allocation.
    fn allocation_size(&self, allocation: *const u8) -> usize;

    fn allocate(&mut self, size: usize) -> *const u8;

    fn allocate_aligned(&mut self, size: usize, alignment: usize) -> *const u8;

    fn reallocate(&mut self, allocation: *const u8, size: usize) -> *const u8;

    fn reallocate_aligned(&mut self, allocation: *const u8, size: usize, alignment: usize) -> *const u8;

    fn deallocate(&mut self, allocation: *const u8);

    fn allocate_second(&mut self, size: usize) -> *const u8;

    fn allocate_aligned_second(&mut self, size: usize, alignment: usize) -> *const u8;

    fn reallocate_second(&mut self, allocation: *const u8, size: usize) -> *const u8;

    fn reallocate_aligned_second(&mut self, allocation: *const u8, size: usize, alignment: usize) -> *const u8;

    fn deallocate_second(&mut self, allocation: *const u8);

    fn unka0(&self) -> bool;

    fn allocation_belongs_to_first_allocator(&mut self, allocation: *const u8) -> bool;

    fn allocation_belongs_to_second_allocator(&mut self, allocation: *const u8) -> bool;

    fn lock(&mut self);

    fn unlock(&mut self);

    fn get_memory_block_for_allocation(&mut self, allocation: *const u8) -> *const u8;
}

pub struct DLAllocator {
    pub vftable: VPtr<dyn DLAllocatorVmt, Self>,
}
