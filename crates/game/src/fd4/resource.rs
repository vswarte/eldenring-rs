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
pub struct FD4ResCap<'a, TRes> {
    pub header: FD4ResCapHeader<'a, TRes>,
    pub data: TRes,
}

#[repr(C)]
pub struct FD4ResCapHeader<'a, TRes> {
    pub vftable: usize,
    pub name: FD4BasicHashString,
    pub owning_repository: &'a FD4ResCapHolder<'a, TRes>,
    pub next_item: *const FD4ResCap<'a, TRes>,
    pub reference_count: u32,
    pad5c: [u8; 4], // TODO: Actually contains two bools
    pub debug_menu_item: usize,
}

/// Represents a collection of ResCaps/FileCaps.
/// The game relies heavily on traditional hashmaps for asset management.
/// The resources name gets turned in a u32 using some FNV variant. That hash
/// is then modulo'd by the repository's capacity to find the appropriate slot 
/// in the map's first layer.
/// In the case of collision on lookups it will start cycling through the 
/// linked list for the matched slot and compare the full resource name hashes.
///
/// This fnv hashing itself is actually facilitated by FD4BasicHashString.
/// In the case of a collision on insertion it will make the entry you are 
/// seeking to insert the new head.
/// 
/// Slot# = fnv(resource name) % holder capacity
///
/// ```
/// +----------------------------------------------------------------------....
/// |                        FD4ResCapHolder<R,T>'s map                    
/// +-------------------------------------------------------+--------------....
/// |  Slot 0          |  Slot 1          |  Slot 2         |  Slot 4
/// +------------------+------------------+-----------------+--------------....
/// |  FD4ResCap<T>    |  FD4ResCap<T>    |                 |  FD4ResCap<T>
/// |  FD4ResCap<T>    |                  |                 |  FD4ResCap<T>
/// |  FD4ResCap<T>    |                  |                 |
/// |                  |                  |                 |
/// |                  |                  |                 |
/// +------------------+------------------+-----------------+--------------....
/// ```
#[repr(C)]
pub struct FD4ResCapHolder<'a, TRes> {
    pub vftable: usize,
    pub allocator: usize,
    pub owning_repository: usize,
    pub unk18: u32,
    pub capacity: u32,
    pub map: &'a FD4ResCap<'a, TRes>,
}
