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
pub struct WorldAreaTimeClock(u64);

impl WorldAreaTimeClock {
    pub fn year(&self) -> u64 {
        self.0 & 0b111111111111
    }

    pub fn milliseconds(&self) -> u64 {
        (self.0 >> 12) & 0b1111111111
    }

    pub fn month(&self) -> u64 {
        (self.0 >> 22) & 0b1111
    }

    pub fn day_of_week(&self) -> u64 {
        (self.0 >> 26) & 0b111
    }

    pub fn day(&self) -> u64 {
        (self.0 >> 29) & 0b11111
    }

    pub fn hours(&self) -> u64 {
        (self.0 >> 34) & 0b11111
    }

    pub fn minutes(&self) -> u64 {
        (self.0 >> 39) & 0b111111
    }

    pub fn seconds(&self) -> u64 {
        (self.0 >> 45) & 0b111111
    }
}

#[repr(u32)]
pub enum AiSightTimeOfDay {
    Morning = 0x0,
    Noon = 0x1,
    Evening = 0x2,
    Night = 0x3,
    Midnight = 0x4,
}
