use std::marker::PhantomData;

use crate::{matrix::FSMatrix4x4, DLRFLocatable, Tree};

pub struct EventFlag(u32);

impl From<u32> for EventFlag {
    fn from(value: u32) -> Self {
        EventFlag(value)
    }
}

impl EventFlag {
    pub fn group(&self) -> u32 {
        self.0 / 1000
    }

    pub fn byte(&self) -> u32 {
        (self.0 % 1000) / 8
    }

    pub fn bit(&self) -> u32 {
        7 - ((self.0 % 1000) % 8)
    }
}

#[repr(C)]
/// Manages the event flags for the game.
///
/// Source of name: RTTI
pub struct CSFD4VirtualMemoryFlag<'a> {
    vftable: usize,
    allocator: usize,
    unk10: u32,
    unk14: u32,
    unk18: u32,
    /// Used to determine the event flag group.
    pub event_flag_divisor: u32,
    /// Size of an event flag group in bytes.
    pub event_flag_holder_size: u32,
    /// Amount of event flag groups.
    pub event_flag_holder_count: u32,
    /// Top of the flag block structure.
    pub flag_blocks: *mut FlagBlock,
    /// Describes where to find a flag block.
    pub flag_block_descriptors: Tree<'a, FlagBlockDescriptor<'a>>,
}

impl DLRFLocatable for CSFD4VirtualMemoryFlag<'_> {
    const DLRF_NAME: &'static str = "CSEventFlagMan";
}

impl<'a> CSFD4VirtualMemoryFlag<'a> {
    pub fn set_flag(&self, flag: impl Into<EventFlag>, state: bool) {
        let flag: EventFlag = flag.into();
        let Some(group) = self
            .flag_block_descriptors
            .iter()
            .find(|d| d.group == flag.group()) else {

            return;
        };

        let block = match group.location() {
            FlagBlockLocation::HolderOffset(offset) => {
                unsafe { self.flag_blocks.add(offset as usize).as_mut() }.unwrap()
            },
            FlagBlockLocation::External(block) => block,
        };

        block.set(flag, state)
    }

    pub fn get_flag(&self, flag: impl Into<EventFlag>) -> bool {
        let flag: EventFlag = flag.into();
        let Some(group) = self
            .flag_block_descriptors
            .iter()
            .find(|d| d.group == flag.group()) else {

            return false;
        };

        let block = match group.location() {
            FlagBlockLocation::HolderOffset(offset) => {
                unsafe { self.flag_blocks.add(offset as usize).as_mut() }.unwrap()
            },
            FlagBlockLocation::External(block) => block,
        };

        block.get(flag)
    }
}

#[repr(C)]
/// Describes where to find a flag block.
pub struct FlagBlockDescriptor<'a> {
    pub group: u32,
    unk4: u32,
    pub location_mode: u32,
    unkc: u32,
    /// Describes the location of the flag block together with location_mode.
    location: FlagBlockLocationUnion<'a>,
}

union FlagBlockLocationUnion<'a> {
    holder_offset: u32,
    external_location: &'a mut FlagBlock,
}

impl<'a> FlagBlockDescriptor<'a> {
    pub fn location(&mut self) -> FlagBlockLocation {
        unsafe {
            match self.location_mode {
                1 => FlagBlockLocation::HolderOffset(self.location.holder_offset),
                2 => FlagBlockLocation::External(self.location.external_location),
                _ => panic!("Flag group location_mode was not 1 or 2"),
            }
        }
    }
}

pub enum FlagBlockLocation<'a> {
    HolderOffset(u32),
    External(&'a mut FlagBlock),
}

#[repr(C)]
/// Contains the actual flag bits
pub struct FlagBlock {
    bytes: [u8; 125],
}

impl FlagBlock {
    pub fn set(&mut self, flag: EventFlag, state: bool) {
        let byte = &mut self.bytes[flag.byte() as usize];
        let mask = 0b00000001 << flag.bit();

        *byte = match state {
            true => *byte | mask,
            false => *byte & !mask,
        }
    }

    pub fn get(&self, flag: EventFlag) -> bool {
        let byte = &self.bytes[flag.byte() as usize];
        let mask = 0b00000001 << flag.bit();

        (*byte & mask) != 0
    }
}
