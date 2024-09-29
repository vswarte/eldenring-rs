use core::ffi;

use crate::DLRFLocatable;
use crate::fd4::FD4BasicHashString;

/// Represents a managed resource.
/// The data it represents is immediately handed over to 
/// other systems and the ResCap serves as a token for unloading things.
/// One such example is gparams where the file associated with a FileCap is
/// parsed, ResCaps (multiple) are created from the FileCap, and the ResCaps
/// individually post the data they represent to associated sub-systems.
/// For GParamResCaps that means posting the such data to the gparam blending
/// system as well as a bunch of other GX structures
#[repr(C)]
pub struct FD4ResCap<TRes> {
    pub header: FD4ResCapHeader<TRes>,
    pub data: TRes,
}

#[repr(C)]
pub struct FD4ResCapHeader<TRes> {
    vftable: usize,
    pub name: FD4BasicHashString,
    owning_repository: *const FD4ResCapHolder<TRes>,
    next_item: *const FD4ResCap<TRes>,
    reference_count: u32,
    unk5c: bool,
    unk5d: bool,
    _pad5e: u16,
    debug_menu_item: usize,
}

/// Represents a collection of ResCaps/FileCaps.
/// The game relies heavily on traditional hashmaps for asset management.
/// The resources name gets turned in a u32 using some FNV variant. That hash
/// is then modulo'd by the repository's capacity to find the appropriate bucket. 
/// In the case of collision on lookups it will start cycling through the 
/// linked list for the matched slot and compare the full resource name hashes.
///
/// This fnv hashing itself is actually facilitated by FD4BasicHashString.
/// In the case of a collision on insertion it will make the entry you are 
/// seeking to insert the new head.
/// 
/// Bucket # = fnv(resource name) % bucket count
///
/// ```
/// +----------------------------------------------------------------------....
/// |                        FD4ResCapHolder<R,T>'s map                    
/// +-------------------------------------------------------+--------------....
/// |  Bucket 0        |  Bucket 1        |  Bucket 2       |  Bucket 3
/// +------------------+------------------+-----------------+--------------....
/// |  FD4ResCap<T>    |  FD4ResCap<T>    |                 |  FD4ResCap<T>
/// |  FD4ResCap<T>    |                  |                 |  FD4ResCap<T>
/// |  FD4ResCap<T>    |                  |                 |
/// |                  |                  |                 |
/// |                  |                  |                 |
/// +------------------+------------------+-----------------+--------------....
/// ```
#[repr(C)]
pub struct FD4ResCapHolder<TRes> {
    vftable: usize,
    allocator: usize,
    owning_repository: usize,
    pub unk18: u32,
    pub bucket_count: u32,
    pub buckets: *const *const FD4ResCap<TRes>,
}

impl<TRes> FD4ResCapHolder<TRes> {
    pub unsafe fn entries(&self) -> impl Iterator<Item = &FD4ResCap<TRes>> {
        let bucket_base =  self.buckets;
        let mut current_element = unsafe { *bucket_base }; 
        let bucket_count = self.bucket_count as isize;
        let mut current_bucket = 0isize;

        tracing::info!("bucket_base = {bucket_base:x?}");
        std::iter::from_fn(move || unsafe {
            // If we dont have an element but we haven't finished the map yet
            // we need to advance to the next bucket until we've found another
            // element.
            while current_element.is_null() && current_bucket < bucket_count - 1 {
                tracing::trace!("Seeking next slot. current_element = {current_element:x?}, current_bucket = {current_bucket}");
                current_bucket += 1;

                let current_bucket_base = bucket_base.offset(current_bucket as isize);
                current_element = if !current_bucket_base.is_null(){
                    *bucket_base.offset(current_bucket)
                } else {
                    std::ptr::null()
                };
            }

            // Move down the bucket if there is an element
            if let Some(element) = current_element.as_ref() {
                tracing::trace!("Found element. current_element = {current_element:x?}");
                current_element = element.header.next_item;
                Some(element)
            } else {
                current_element = std::ptr::null();
                None
            }
        })
    }
}
