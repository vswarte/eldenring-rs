use std::ptr::NonNull;

use crate::pointer::OwnedPtr;
use crate::matrix::{FSVector4};


use super::{CSBulletIns, FieldInsHandle};

#[repr(C)]
#[dlrf::singleton("CSBulletManager")]
pub struct CSBulletManager {
    pub bullets: BufferAndAllocLinkedList<CSBulletIns, 128, 128>,
    unk_bullet_sfx_related: BufferAndAllocLinkedList<[u8; 0x9d0], 64, 192>,
    unk40: BufferAndAllocLinkedList<[u8; 0x4220], 4, 28>,
    chr_cam: usize,
    unk68: u8,
    _pad69: [u8; 7],
    unk70: u64,
    net_packing_vector_bullet_sync_setting: [u8; 0x28],
    net_packing_vector_bullet_emitter_destroy_sync_info: [u8; 0x28],
    net_packing_vector_bullet_inisync_setting: [u8; 0x28],
    net_packing_vector_change_target_req1: [u8; 0x28],
    net_packing_vector_change_target_req2: [u8; 0x28],
    net_packing_vector_bullet_on_hit_sync_info: [u8; 0x28],
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
    sfx_related_in_buffer_count: u32,
    sfx_related_count: u32,
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

        // In buffer, array access
        if index < 0x80 {
            let bullet = &mut self.bullets.prealloc_buffer[index as usize];

            if bullet.field_ins_handle == handle {
                return Some(bullet);
            }
        }
        // Not in buffer, iterate linked list
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


    // TODO All set up, just needs a way to get program location to call game's BULLET_SPAWN
    //
    // RVA_BULLET_SPAWN = 0x1403a2dd0
    /*
    pub fn spawn_bullet(&mut self, spawn_data: &BulletSpawnData) -> Result<(), i32> {
        let spawn_bullet_func: extern "C" fn(
            &mut CSBulletManager,
            &mut i32,
            &BulletSpawnData,
            &mut i32,
        ) = unsafe {
            std::mem::transmute(program.get(RVA_SPAWN_BULLET).unwrap())
        };


        let unk_out = &mut 0;
        let error_out = &mut 0;
        spawn_bullet_func(self, unk_out, spawn_data, error_out);

        if *unk_out == -1 {
            return Err(*error_out);
        }

        Ok(())
    }
     */


    pub fn bullets_iter(&self) -> impl Iterator<Item = &CSBulletIns> {
        let mut current = self.bullets.head;

        std::iter::from_fn(move || {
            let ret = current.map(|ptr| unsafe { ptr.as_ref() });
            current = ret.and_then(|curr| curr.next_bullet);
            ret
        })
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

#[repr(C)]
pub struct BulletSpawnData {
    owner: FieldInsHandle,
    behavior_id: i32,
    magic_id: i32,
    unk10: u32,
    bullet_id: i32,
    goods_id: i32,
    dummy_poly_id: i32,
    /// Replaces owner's target if not -1
    target: FieldInsHandle,
    unk28: u32,
    unk2c: u32,
    unk30: FSVector4,
    unk40: u32,
    unk44: u32,
    pad48: [u8; 8],
    /// Forward vector, only applies if angle vec is 0?
    acceleration_angle: FSVector4,
    unk60: FSVector4,
    /// Forward vector
    angle: FSVector4,
    position: FSVector4,
    unk90: u64,
    unk98: u64,
    unka0: u64,
    pada8: [u8; 8],
    unkb0_struct: [u8; 0x50],
    unk100: u8,
    pad101: [u8; 15],
}
