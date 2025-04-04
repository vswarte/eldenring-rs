use std::ptr::NonNull;

use crate::pointer::OwnedPtr;

use super::resource::FD4ResCap;
use super::FD4ResRep;

#[repr(C)]
#[dlrf::singleton("FD4ParamRepository")]
pub struct FD4ParamRepository {
    /// Resource repository holding the actual param data.
    pub res_rep: FD4ResRep<FD4ParamResCap>,
    allocator: usize,
}

#[repr(C)]
pub struct FD4ParamResCap {
    pub res_cap: FD4ResCap<Self>,

    /// Size of data at pointer.
    pub size: u64,
    /// Raw row data for this param file.
    pub data: OwnedPtr<ParamData>,
}

impl AsRef<FD4ResCap<Self>> for FD4ParamResCap {
    fn as_ref(&self) -> &FD4ResCap<Self> {
        &self.res_cap
    }
}

#[repr(C)]
pub struct ParamData {
    pub header: ParamFileHeader,
}

impl ParamData {
    pub fn row_descriptors(&self) -> &[ParamRowDescriptor] {
        unsafe {
            // The row descriptors are right after the header.
            std::slice::from_raw_parts(
                (self as *const _ as usize + size_of::<ParamFileHeader>()) as *const _,
                self.header.row_count as usize,
            )
        }
    }

    /// Retrieve a param entry by its ID and type.
    ///
    /// # Safety
    /// Caller has to ensure that the param type passed to T is valid for the param file.
    pub unsafe fn get<T: Sized>(&self, id: u32) -> Option<&T> {
        let offset = self
            .row_descriptors()
            .iter()
            .find(|r| r.id == id)?
            .data_offset;

        unsafe { Some(&*((self as *const _ as usize + offset) as *const T)) }
    }

    /// Retrieve a param entry mutable by its ID and type.
    ///
    /// # Safety
    /// Caller has to ensure that the param type passed to T is valid for the param file.
    pub unsafe fn get_mut<T: Sized>(&mut self, id: u32) -> Option<&mut T> {
        let offset = self
            .row_descriptors()
            .iter()
            .find(|r| r.id == id)?
            .data_offset;

        unsafe { Some(&mut *((self as *const _ as usize + offset) as *mut T)) }
    }
}

#[repr(C)]
pub struct ParamFileHeader {
    name_offset: u32,
    unk4: u32,
    pub paramdef_version: u16,
    pub row_count: u16,
    unkc: u32,
    unk10: u64,
    unk18: u64,
    unk20: u64,
    unk28: u64,
    data_offset: u64,
    unk38: u64,
}

#[repr(C)]
pub struct ParamRowDescriptor {
    pub id: u32,
    _pad4: u32,
    pub data_offset: usize,
    pub name_offset: usize,
}
