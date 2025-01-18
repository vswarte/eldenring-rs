use std::ptr::NonNull;

use crate::pointer::OwnedPtr;

use super::{CSBulletIns, FieldInsHandle};

#[repr(C)]
#[dlrf::singleton("CSBulletManager")]
pub struct CSBulletManager {
    pub bullets: BufferAndAllocLinkedList<CSBulletIns, 128, 128>,
    pub unk_bullet_sfx_related: BufferAndAllocLinkedList<[u8; 0x9d0], 64, 192>,
    pub unk40: BufferAndAllocLinkedList<[u8; 0x4220], 4, 28>,
    pub chr_cam: usize,
    unk68: u8,
    _pad69: [u8; 7],
    unk70: u64,
    pub net_packing_vector_bullet_sync_setting: [u8; 0x28],
    pub net_packing_vector_bullet_emitter_destroy_sync_info: [u8; 0x28],
    pub net_packing_vector_bullet_inisync_setting: [u8; 0x28],
    pub net_packing_vector_change_target_req1: [u8; 0x28],
    pub net_packing_vector_change_target_req2: [u8; 0x28],
    pub net_packing_vector_bullet_on_hit_sync_info: [u8; 0x28],
    unk168: [u8; 0x18],
    unk180: [u8; 0x20],
    unk1a0: u8,
    _pad1a1: [u8; 7],
    unk1a8_func_table: usize,
    unk1b0_func_table: usize,
    unk1b8_func_table: usize,
    unk1c0: u8,
    _pad1c1: [u8; 3],
    unk1c4: f32,
    unk1c8: f32,
    unk1cc: u32,
    pub sfx_related_in_buffer_count: u32,
    pub sfx_related_count: u32,
    pub bullets_in_buffer_count: u32,
    pub bullets_count: u32,
    unk1e0: u32,
    unk1e4: u32,
    unk1e8: u32,
    unk1f0: u8,
    _pad1f1: [u8; 3],
    unk1f4: f32,
    unk1f8: f32,
    _pad1fc: [u8; 4],
    unk200: [f32; 4],
    unk210: [f32; 4],
    unk220: [f32; 4],
    unk230: [f32; 4],
    unk240: [f32; 4],
    unk250: [u8; 0x20],
    unk270: u8,
    _pad271: [u8; 3],
    unk274: f32,
    unk278: usize,
}

impl CSBulletManager {
    pub fn get_bullet_by_handle(&mut self, handle: FieldInsHandle) -> Option<&mut CSBulletIns> {
        let index = handle.selector.0 & 0xFF;

        if index < 0x80 {
            let bullet = &mut self.bullets.prealloc_buffer[index as usize];

            if bullet.field_ins_handle == handle {
                return Some(bullet);
            }
        }
        else if index == 254 {
            let mut current = &self.bullets.head;
            while !current.is_none() {
                let bullet = unsafe { current.unwrap().as_mut() };

                if bullet.field_ins_handle == handle {
                    return Some(bullet);
                }

                current = &bullet.next_bullet;
            }
        }

        None
    }
}

// TODO Move to a more fitting file? I don't think I've seen this type of struct anywhere else
/// Contains a pre allocated buffer that takes priority when creating a new T, when full
/// starts allocating manually on the heap.
/// Living elements create a linked list.
pub struct BufferAndAllocLinkedList<T, const BUFFER_SIZE: usize, const MAX_ALLOCS: usize> {
    prealloc_buffer: OwnedPtr<[T; BUFFER_SIZE]>,
    head: Option<NonNull<T>>,
    /// If buffer is full, None
    empty_spot: Option<NonNull<T>>,
    /// Amount of allocated Ts alive
    allocated_count: u32,
    /// Doesn't decrease
    allocations_counter: u32,
}
