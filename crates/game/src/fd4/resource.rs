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
pub struct FD4ResCap<T>
where
    T: AsRef<FD4ResCap<T>>,
{
    vftable: usize,
    /// Name of the resource contained in the ResCap
    pub name: FD4BasicHashString,
    /// The repository this resource is hosted in.
    pub owning_repository: Option<NonNull<FD4ResCapHolder<T>>>,
    /// Next item in the linked list
    pub next_item: Option<NonNull<T>>,
    /// Amount of references to this resource.
    pub reference_count: u32,
    unk5c: u32,
    unk60: bool,
    unk61: [u8; 7],
    unk68: usize,
    unk70: u8,
    unk71: [u8; 7],
}

/// Manages a collection of ResCaps by wrapping a FD4ResCapHolder and defines some logic specific
/// to T.
///
/// Source of name: RTTI
#[repr(C)]
pub struct FD4ResRep<T>
where
    T: AsRef<FD4ResCap<T>>,
{
    /// Repositories themselves inherit from ResCaps.
    pub res_cap: FD4ResCap<Self>,

    /// Holds a set of ResCaps wrapping T.
    pub res_cap_holder: FD4ResCapHolder<T>,
}

impl<T> AsRef<FD4ResCap<Self>> for FD4ResRep<T>
where
    T: AsRef<FD4ResCap<T>>,
{
    fn as_ref(&self) -> &FD4ResCap<Self> {
        &self.res_cap
    }
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
///
/// Source of name: DLRF reflection data
#[repr(C)]
pub struct FD4ResCapHolder<T>
where
    T: AsRef<FD4ResCap<T>>,
{
    vftable: usize,
    allocator: usize,
    pub owning_repository: Option<NonNull<FD4ResCapHolder<T>>>,
    unk18: u32,
    pub bucket_count: u32,
    buckets: NonNull<Option<NonNull<T>>>,
}

impl<T> FD4ResCapHolder<T>
where
    T: AsRef<FD4ResCap<T>>,
{
    pub fn entries<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        let bucket_base = self.buckets;
        let mut current_element = unsafe { bucket_base.as_ref() };

        let bucket_count = self.bucket_count as isize;
        let mut current_bucket = 0isize;

        std::iter::from_fn(move || unsafe {
            // Find first non-empty bucket
            while current_element.is_none() && current_bucket < bucket_count - 1 {
                current_bucket += 1;
                current_element = bucket_base.offset(current_bucket).as_ref();
            }

            if let Some(element) = current_element {
                let cap: &FD4ResCap<T> = element.as_ref().as_ref();
                current_element = &cap.next_item;
                Some(element.as_ref())
            } else {
                None
            }
        })
    }
}

/// Represents file load state for this FD4FileCap.
#[repr(u8)]
pub enum FD4FileCapState {
    Initial = 0x0,
    Queued = 0x1,
    Processing = 0x2,
    Unknown = 0x3,
    Ready = 0x4,
}

pub struct FD4FileCapUnk89Properties(u8);

impl FD4FileCapUnk89Properties {
    fn unk1(&self) -> bool {
        self.0 & 0b00000010 != 0
    }

    // Set on creation
    fn unk5(&self) -> bool {
        self.0 & 0b00100000 != 0
    }

    fn set_unk1(&mut self, state: bool) {
        match state {
            true => self.0 |= 0b00000010,
            false => self.0 &= !0b00000010,
        }
    }

    fn set_unk5(&mut self, state: bool) {
        match state {
            true => self.0 |= 0b00100000,
            false => self.0 &= !0b00100000,
        }
    }

    pub fn file_load_queue_index(&self) -> u8 {
        self.0 >> 2 & 0b00000111
    }

    pub fn set_file_load_queue_index(&mut self, slot: u8) {
        self.0 |= (slot & 0b00000111) << 2
    }
}

pub struct FD4FileCapUnk8AProperties(u16);

impl FD4FileCapUnk8AProperties {
    pub fn use_secondary_repository(&self) -> bool {
        self.0 & 0b00000010 != 0
    }

    pub fn set_use_secondary_repository(&mut self, state: bool) {
        match state {
            true => self.0 |= 0b00000010,
            false => self.0 &= !0b00000010,
        }
    }

    pub fn mutex_index(&self) -> u16 {
        self.0 >> 3
    }

    pub fn set_mutex_index(&mut self, slot: u16) {
        self.0 &= 0b00000111;
        self.0 |= slot << 3;
    }
}

/// Represents a file resource be it on-disk or virtual. Responsible for parsing the files bytes
/// and spawning ResCaps for the parsed resources.
///
/// Source of name: RTTI
#[repr(C)]
pub struct FD4FileCap<T>
where
    T: AsRef<FD4ResCap<T>>,
{
    pub res_cap: FD4ResCap<T>,
    load_process: usize,
    load_task: usize,
    pub load_state: FD4FileCapState,
    unk89: FD4FileCapUnk89Properties,
    unk8a: FD4FileCapUnk8AProperties,
    unk8c: u32,
}

impl<T> AsRef<FD4ResCap<T>> for FD4FileCap<T>
where
    T: AsRef<FD4ResCap<T>>,
{
    fn as_ref(&self) -> &FD4ResCap<T> {
        &self.res_cap
    }
}
