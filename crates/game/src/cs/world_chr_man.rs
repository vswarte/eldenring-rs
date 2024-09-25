use std::ffi;
use std::marker::PhantomData;

use crate::cs::ChrIns;
use crate::DLRFLocatable;

use super::PlayerIns;

#[repr(C)]
pub struct WorldChrMan<'a> {
    pub vftable: usize,
    unk8: usize,
    pub world_area_chr: [WorldAreaChr<'a>; 28],
    pub world_block_chr: [WorldBlockChr<'a>; 192],
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
    pub player_chr_set: ChrSet<'a>,
    // Ghosts and such
    pub ghost_chr_set: ChrSet<'a>,
    // Spirit ash characters
    pub summon_buddy_chr_set: ChrSet<'a>,
    // Debug characters
    pub debug_chr_set: ChrSet<'a>,
    // All other enemies
    pub open_field_chr_set: OpenFieldChrSet<'a>,

    pub unk1cc58: [u8; 0x18b0],
    pub main_player: *mut PlayerIns<'a>,

    pub unk1e510: [u8; 0x28],
    pub summon_buddy_manager: *mut SummonBuddyManager<'a>,
    // TODO: rest
}

impl DLRFLocatable for WorldChrMan<'_> {
    const DLRF_NAME: &'static str = "WorldChrMan";
}

#[repr(C)]
pub struct WorldAreaChr<'a> {
    pub base: WorldAreaChrBase,
    pub world_area_info: usize,
    pub unk18: u32,
    pub unk1c: u32,
    pub world_block_chr: &'a WorldBlockChr<'a>,
}

#[repr(C)]
pub struct WorldAreaChrBase {
    pub vftable: usize,
    pub world_area_info: usize,
}

#[repr(C)]
pub struct WorldBlockChr<'a> {
    pub vftable: usize,
    pub world_block_info1: usize,
    unk10: [u8; 0x68],
    pub chr_set: ChrSet<'a>,
    unkd0: [u8; 0x40],
    pub world_block_info2: usize,
    pub chr_set_ptr: &'a mut ChrSet<'a>,
    pub allocator: usize,
    unk128: [u8; 0x30],
    pub map_id: MapId,
    unk15c: u32,
}

#[repr(C)]
pub struct ChrSet<'a> {
    pub vftable: usize,
    pub unk8: i32,
    pub unkc: i32,
    pub capacity: i32,
    pub unk14: u32,
    pub entries: *const ChrSetEntry<'a>,
    pub unk20: i32,
    pub unk24: u32,
    pub list1: UnkBtree,
    pub list2: UnkBtree,
}

pub struct ChrSetIter<'a> {
    remaining: usize,
    current: *const ChrSetEntry<'a>,
}

impl<'a> Iterator for ChrSetIter<'a> {
    type Item = ChrSetIterElement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            loop {
                let chr_ins = (*self.current).chr_ins.as_mut();

                self.current = self.current.wrapping_add(1);
                self.remaining -= 1;

                if let Some(chr_ins) = chr_ins {
                    return Some(ChrSetIterElement { chr_ins });
                }
            }
        }

        None
    }
}

pub struct ChrSetIterElement<'a> {
    pub chr_ins: &'a mut ChrIns<'a>,
}

#[repr(C)]
pub struct ChrSetEntry<'a> {
    pub chr_ins: *mut ChrIns<'a>,
    pub unk8: u32,
    pub unkc: u32,
}

#[repr(C)]
pub struct OpenFieldChrSet<'a> {
    pub base: ChrSet<'a>,
    unk58: UnkBtree,
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
pub struct UnkBtree {
    pub vftable: usize,
    pub head: usize,
    pub entry_count: u32,
    _pad14: u32,
}

#[repr(C)]
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

#[repr(C)]
pub struct SummonBuddyManager<'a> {
    pub vftable: usize,
    pub unk8: usize,
    pub unk10: usize,
    pub unk18: usize,
    pub to_spawn_buddy_param: i32,
    pub spawned_buddy_param: i32,
    pub unk28: usize,
    pub chr_set: &'a ChrSet<'a>,
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
