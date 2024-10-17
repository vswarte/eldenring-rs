use crate::DLRFLocatable;

#[repr(C)]
pub struct WorldAreaTime {
    pub unk0: u64,
    pub clock: WorldAreaTimeClock,
    pub unk10: u64,
    pub previous_tick_clock: WorldAreaTimeClock,
    pub unk20: f32,
    pub unk24: f32,
    pub target_hour: u32,
    pub target_minute: u32,
    pub target_second: u32,
    pub unk34: f32,
    pub time_passage_multiplier: f32,
    pub unk3c: f32,
    // TODO: rest
}

impl DLRFLocatable for WorldAreaTime {
    const DLRF_NAME: &'static str = "WorldAreaTime";
}

#[repr(C)]
/// A packed clock as used by the game.
pub struct WorldAreaTimeClock(pub u64);

#[repr(u32)]
pub enum AiSightTimeOfDay {
    Morning = 0x0,
    Noon = 0x1,
    Evening = 0x2,
    Night = 0x3,
    Midnight = 0x4,
}
