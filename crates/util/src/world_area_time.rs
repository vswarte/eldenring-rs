use game::cs::WorldAreaTime;

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
