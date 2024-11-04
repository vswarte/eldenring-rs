use std::ffi;
use std::fmt::Display;
use std::marker::PhantomData;
use std::mem::transmute;

use crate::cs::ChrIns;
use crate::{DLRFLocatable, Tree};

use super::{FieldInsHandle, PlayerIns};

#[repr(C)]
/// Source of name: RTTI
pub struct WorldChrMan<'a> {
    vftable: usize,
    unk8: usize,
    pub world_area_chr: [WorldAreaChr<'a, ChrIns<'a>>; 28],
    pub world_block_chr: [WorldBlockChr<'a, ChrIns<'a>>; 192],
    pub world_grid_area_chr: [WorldGridAreaChr; 6],
    pub world_area_info_owner: usize,

    pub world_area_chr_list_count: u32,
    unk10d9c: u32,
    pub world_area_chr_ptr: usize,

    pub world_block_chr_list_count: u32,
    unk10dac: u32,
    pub world_block_chr_ptr: usize,

    pub world_grid_area_chr_list_count: u32,
    unk10dbc: u32,
    pub world_grid_area_chr_ptr: usize,

    pub world_area_list: [&'a WorldAreaChrBase; 34],
    pub world_area_list_count: u32,
    unk10edc: u32,

    // Player characters
    pub player_chr_set: ChrSet<'a, PlayerIns<'a>>,
    // Ghosts and such
    pub ghost_chr_set: ChrSet<'a, ChrIns<'a>>,
    // Spirit ash characters
    pub summon_buddy_chr_set: ChrSet<'a, ChrIns<'a>>,
    // Debug characters
    pub debug_chr_set: ChrSet<'a, ChrIns<'a>>,
    // All other enemies
    pub open_field_chr_set: OpenFieldChrSet<'a>,

    pub unk1cc58: [u8; 0x18b0],
    pub main_player: Option<&'a mut PlayerIns<'a>>,

    pub unk1e510: [u8; 0x28],
    pub summon_buddy_manager: Option<&'a SummonBuddyManager<'a>>,
    // TODO: rest
}

impl DLRFLocatable for WorldChrMan<'_> {
    const DLRF_NAME: &'static str = "WorldChrMan";
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaChr<'a, T> {
    pub base: WorldAreaChrBase,
    pub world_area_info: usize,
    pub unk18: u32,
    pub unk1c: u32,
    pub world_block_chr: &'a WorldBlockChr<'a, T>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaChrBase {
    vftable: usize,
    pub world_area_info: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldBlockChr<'a, T> {
    vftable: usize,
    pub world_block_info1: usize,
    unk10: [u8; 0x68],
    pub chr_set: ChrSet<'a, T>,
    unkd0: [u8; 0x40],
    pub world_block_info2: usize,
    pub chr_set_ptr: &'a mut ChrSet<'a, T>,
    pub allocator: usize,
    unk128: [u8; 0x30],
    pub map_id: MapId,
    unk15c: u32,
}

#[repr(C)]
pub struct ChrSetVMT<'a, T> {
    /// Gets the max amount of ChrInses this ChrSet can hold.
    pub get_capacity: extern "C" fn(*const ChrSet<'a, T>) -> u32,

    /// Wrapped version of get_chr_ins_by_index which also validates the 
    /// index against the ChrSet capacity.
    pub safe_get_chr_ins_by_index: extern "C" fn(*const ChrSet<'a, T>, u32) -> *mut ChrIns<'a>,

    /// Retrieves a ChrIns from the ChrSet by its index. Avoid using this. 
    /// Prefer using safe_get_chr_ins_by_index.
    pub get_chr_ins_by_index: extern "C" fn(*const ChrSet<'a, T>, u32) -> *mut ChrIns<'a>,

    /// Retrieves a ChrIns from the ChrSet by its FieldIns handle.
    pub get_chr_ins_by_handle: extern "C" fn(*const ChrSet<'a, T>, u64) -> *mut ChrIns<'a>,

    /// Wrapped version of get_chr_ins_by_index which also validates the 
    /// index against the ChrSet capacity.
    pub safe_get_chr_set_entry_by_index: extern "C" fn(*const ChrSet<'a, T>, u32) -> *mut ChrSetEntry<ChrIns<'a>>,

    /// Retrieves a ChrSetEntry from the ChrSet by its index. Avoid using this. 
    /// Prefer using safe_get_chr_ins_by_index.
    pub get_chr_set_entry_by_index: extern "C" fn(*const ChrSet<'a, T>, u32) -> *mut ChrSetEntry<ChrIns<'a>>,

    /// Retrieves a ChrSetEntry from the ChrSet by its index. Avoid using this. 
    /// Prefer using safe_get_chr_ins_by_index.
    pub get_chr_set_entry_by_handle: extern "C" fn(*const ChrSet<'a, T>, u64) -> *mut ChrSetEntry<ChrIns<'a>>,

    /// Retrieves the FieldIns handle of the ChrIns at the index in the ChrSet.
    pub get_index_by_field_ins_handle: extern "C" fn(*const ChrSet<'a, T>, u64) -> u32,

    /// Deallocates all ChrInses hosted by the ChrSet.
    pub free_chr_list: extern "C" fn(*const ChrSet<'a, T>),

    unk48: extern "C" fn(*const ChrSet<'a, T>),

    unk50: extern "C" fn(*const ChrSet<'a, T>),

    unk58: extern "C" fn(*const ChrSet<'a, T>, usize),

    unk60: extern "C" fn(*const ChrSet<'a, T>, usize),

    unk68: extern "C" fn(*const ChrSet<'a, T>, usize, usize, u8, u8),
}

#[repr(C)]
/// Source of name: RTTI
pub struct ChrSet<'a, T> {
    vftable: &'a ChrSetVMT<'a, T>,
    pub index: i32,
    pub unkc: i32,
    pub capacity: u32,
    _pad14: u32,
    pub entries: *const ChrSetEntry<T>,
    pub unk20: i32,
    _pad24: u32,
    pub unk30: [u8; 0x30],
}

impl<'a, T> ChrSet<'a, T> {
    pub unsafe fn characters(&'a self) -> impl Iterator<Item = &'a mut T> {
        let mut current_entry = self.entries;
        let end = unsafe { current_entry.add(self.capacity as usize) };

        std::iter::from_fn(move || {
            if current_entry == end {
                None
            } else {
                let chr_ins = (*current_entry).chr_ins;
                current_entry.add(1);
                chr_ins.as_mut()
            }
        })
    }
}

#[repr(C)]
pub struct ChrSetEntry<T> {
    pub chr_ins: *mut T,
    pub unk8: u16,
    pub unka: u8,
    _padb: [u8; 5],
}

#[repr(C)]
/// Source of name: RTTI
pub struct OpenFieldChrSet<'a> {
    pub base: ChrSet<'a, ChrIns<'a>>,
    // TODO: type needs fact-checking
    unk58: Tree<()>,
    unk70: f32,
    pad74: u32,
    list1: [OpenFieldChrSetList1Entry<'a>; 1500],
    unk5e38: u32,
    unk5e3c: u32,
    unk5e40: u32,
    unk5e44: u32,
    list2: [OpenFieldChrSetList2Entry; 1500],
    unkbc08: u64,
    unkbc10: u64,
}

#[repr(C)]
pub struct OpenFieldChrSetList1Entry<'a> {
    pub unk0: u64,
    pub chr_ins: &'a mut ChrIns<'a>,
}

#[repr(C)]
pub struct OpenFieldChrSetList2Entry {
    pub unk0: u64,
    pub unk8: u32,
    pub unkc: u32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldGridAreaChr {
    pub base: WorldAreaChrBase,
    pub world_grid_area_info: usize,
    pub allocator: usize,
    pub head: usize,
    pub capacity: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MapId {
    pub index: u8,
    pub region: u8,
    pub block: u8,
    pub area: u8,
}

impl Display for MapId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "m{}_{}_{}_{}", self.area, self.block, self.region, self.index)
    }
}

#[repr(C)]
/// Source of name: "SummonBuddy" mentioned in DLRF metadata for the update fn.
pub struct SummonBuddyManager<'a> {
    vftable: usize,
    pub unk8: usize,
    pub unk10: usize,
    pub unk18: usize,
    pub to_spawn_buddy_param: i32,
    pub spawned_buddy_param: i32,
    pub unk28: usize,
    pub chr_set: &'a ChrSet<'a, ChrIns<'a>>,
    pub unk38: [u8; 0xb0],
    pub warp: *const SummonBuddyManagerWarp,
}

#[repr(C)]
pub struct SummonBuddyManagerWarp {
    pub allocator: usize,
    pub root_node: usize,
    pub unk10: usize,
    pub trigger_time_ray_block: f32,
    pub trigger_dist_to_player: f32,
    pub trigger_threshold_time_path_stacked: f32,
    pub trigger_threshold_range_path_stacked: f32,
    pub unk28: [u8; 0x10]
}
