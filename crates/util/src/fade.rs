use game::cs::CSFD4FadePlate;

pub trait CSFD4FadePlateExt {
    fn fade_in(&mut self, time: f32);
    fn fade_out(&mut self, time: f32);
}

impl CSFD4FadePlateExt for CSFD4FadePlate {
    fn fade_in(&mut self, time: f32) {
        self.end_color.a = 0.0;
        self.start_color.a = 1.0;
        self.fade_duration.time = time;
        self.fade_timer.time = time;
    }

    fn fade_out(&mut self, time: f32) {
        self.end_color.a = 1.0;
        self.start_color.a = 0.0;
        self.fade_duration.time = time;
        self.fade_timer.time = time;
    }
}
