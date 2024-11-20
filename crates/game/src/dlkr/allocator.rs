use vtable_rs::VPtr;

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
