use game::cs::{WorldAreaTime, WorldAreaTimeClock};

pub trait WorldAreaTimeExt {
    fn set_target(&mut self, hour: u32, minute: u32, second: u32);
}

impl WorldAreaTimeExt for WorldAreaTime {
    fn set_target(&mut self, hour: u32, minute: u32, second: u32) {
        self.target_hour = hour;
        self.target_minute = minute;
        self.target_second = second;
    }
}

pub trait WorldAreaTimeClockExt {
    fn year(&self) -> u64;
    fn milliseconds(&self) -> u64;
    fn month(&self) -> u64;
    fn day_of_week(&self) -> u64;
    fn day(&self) -> u64;
    fn hours(&self) -> u64;
    fn minutes(&self) -> u64;
    fn seconds(&self) -> u64;
}

impl WorldAreaTimeClockExt for WorldAreaTimeClock {
    fn year(&self) -> u64 {
        self.0 & 0b111111111111
    }

    fn milliseconds(&self) -> u64 {
        (self.0 >> 12) & 0b1111111111
    }

    fn month(&self) -> u64 {
        (self.0 >> 22) & 0b1111
    }

    fn day_of_week(&self) -> u64 {
        (self.0 >> 26) & 0b111
    }

    fn day(&self) -> u64 {
        (self.0 >> 29) & 0b11111
    }

    fn hours(&self) -> u64 {
        (self.0 >> 34) & 0b11111
    }

    fn minutes(&self) -> u64 {
        (self.0 >> 39) & 0b111111
    }

    fn seconds(&self) -> u64 {
        (self.0 >> 45) & 0b111111
    }
}
