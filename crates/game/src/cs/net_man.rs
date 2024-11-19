use crate::{pointer::OwnedPtr, stl::DoublyLinkedList};

use super::{CSEzTask, CSEzUpdateTask, MapId, PlayerIns};

#[repr(C)]
#[dlrf::singleton("CSNetMan")]
pub struct CSNetMan {
    vftable: usize,
    unk8: [u8; 0x60],
    pub sos_db: usize,
    pub wandering_ghost_db: usize,
    pub blood_message_db: OwnedPtr<CSNetBloodMessageDb>,
    pub bloodstain_db: usize,
    pub bonfire_db: usize,
    pub spiritual_statue_db: usize,
    unk98: usize,
    unka0: usize,
    unka8: usize,
    unk_quickmatch: usize,
    pub visitor_db: usize,
    pub penalty_manager: usize,
    pub update_task: CSEzUpdateTask<Self>,
    unkf0: u32,
    unkf4: u32, // Probably padding
    unkf8: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageDb {
    vftable: usize,
    // Contains all CSNetBloodMessageDbItem?
    pub entries: DoublyLinkedList<OwnedPtr<CSNetBloodMessageDbItem>>,
    unk20: usize,
    // Seemingly contains message data for messages created by local user
    pub created_data: DoublyLinkedList<usize>,
    // Contains ???
    unk40: DoublyLinkedList<usize>,
    unk58: usize,
    pub blood_message_ins_man_1: usize,
    pub blood_message_ins_man_2: usize,
    pub discovered_messages: DoublyLinkedList<OwnedPtr<OwnedPtr<CSNetBloodMessageDbItem>>>,
    unk88: [u8; 0xD0],
    pub evaluate_job: usize,
    unk160: usize,
}

#[repr(C)]
pub struct CSNetBloodMessageDbItem {
    vftable: usize,
    unk8: u32,
    unkc: u32,
    unk10: u32,
    pub map_id: MapId,
    unk18: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub angle: f32,
    pub template1: u16,
    pub gesture_param: u16,
    pub part1: u16,
    pub infix: u16,
    pub template2: u16,
    pub part2: u16,
    unk38: u16,
    unk3a: u16,
    unk3c: u16,
    unk3e: u16,
    pub message_id: u64,
    unk48: u32,
}
