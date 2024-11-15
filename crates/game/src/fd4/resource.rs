use core::ffi;
use std::ptr::NonNull;

use crate::fd4::FD4BasicHashString;

/// Represents a managed resource.
/// The data it represents is immediately handed over to 
/// other systems and the ResCap serves as a token for unloading things.
/// One such example is gparams where the file associated with a FileCap is
/// parsed, ResCaps (multiple) are created from the FileCap, and the ResCaps
/// individually post the data they represent to associated sub-systems.
/// For GParamResCaps that means posting the such data to the gparam blending
/// system as well as a bunch of other GX structures
///
/// Source of name: RTTI
#[repr(C)]
pub struct FD4ResCap<T> {
    vftable: usize,
    /// Name of the resource contained in the ResCap
    pub name: FD4BasicHashString,
    /// The repository this resource is hosted in.
    pub owning_repository: Option<NonNull<FD4ResCapHolder<T>>>,
    /// Next item in de linked list
    pub next_item: *const FD4ResCap<T>,
    /// Amount of references to this resource.
    pub reference_count: u32,
    unk5c: bool,
    unk5d: bool,
    _pad5e: u16,
    unk60: bool,
    unk61: [u8; 7],
    unk68: usize,
    unk70: u8,
    unk71: [u8; 7],

    /// Actual data contained within the rescap.
    pub data: T,
}

/// Manages a collection of ResCaps by wrapping a FD4ResCapHolder and defines some logic specific
/// to T.
///
/// Source of name: RTTI
#[repr(C)]
pub struct FD4ResRep<T> {
    /// Repositories themselves inherit from ResCaps.
    pub res_cap: FD4ResCap<()>,

    /// Holds a set of ResCaps wrapping T.
    pub res_cap_holder: FD4ResCapHolder<T>,
}

/// Represents a collection of ResCaps/FileCaps.
/// The game relies heavily on hashmaps for asset management.
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
/// |                        FD4ResCapHolder<T>'s map
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
///
/// Source of name: DLRF reflection data
#[repr(C)]
pub struct FD4ResCapHolder<T> {
    vftable: usize,
    allocator: usize,
    pub owning_repository: Option<NonNull<FD4ResCapHolder<T>>>,
    unk18: u32,
    pub bucket_count: u32,
    pub buckets: *const *const FD4ResCap<T>,
}
