use std::ffi;
use std::ptr::NonNull;

use crate::dlkr::DLPlainLightMutex;
use crate::fd4::{
    FD4BasicHashString, FD4FileCap, FD4ResCap, FD4ResCapHolder, FD4ResRep
};
use crate::pointer::OwnedPtr;
use crate::DoublyLinkedList;

/// Manages files used by the file, both virtual and on-disk.
#[repr(C)]
pub struct CSFile {
    vftable: usize,
    pub file_repository_1: OwnedPtr<CSFileRepository>,
    // TODO: Incomplete..
}

/// Manages a set of files as well as keeps track of load state and such.
#[repr(C)]
pub struct CSFileRepository {
    pub res_rep: FD4ResRep<UntypedFileCap>,
    pub holder2: FD4ResCapHolder<UntypedFileCap>,
    unkc8: DoublyLinkedList<()>,
    pub mutexes: [OwnedPtr<CSFileRepositoryMutex>; 5],
    file_load_event_queues: [OwnedPtr<usize>; 5],
}

#[repr(C)]
pub struct CSFileRepositoryMutex {
    pub mutex: DLPlainLightMutex,
    unk30: u32,
    unk34: u32,
    unk38: u32,
    unk3c: u32,
    unk40: usize,
    unk48: usize,
}

#[repr(C)]
/// Used to represent file caps without a concrete type.
pub struct UntypedFileCap {
    pub file_cap: FD4FileCap<Self>,
}

impl AsRef<FD4ResCap<Self>> for UntypedFileCap {
    fn as_ref(&self) -> &FD4ResCap<Self> {
        &self.file_cap.res_cap
    }

    // TODO: add downcasting logic
}
