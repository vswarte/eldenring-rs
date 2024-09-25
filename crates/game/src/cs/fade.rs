use crate::DLRFLocatable;
use crate::fd4::FD4Time;

#[repr(C)]
pub struct CSFade<'a> {
    pub vftable: usize,
    pub fade_system: &'a mut CSFD4FadeSystem,
    pub fade_plates: [&'a mut CSFD4FadePlate; 9],
    pub unk58: u32,
    pub unk5c: f32,
}

impl DLRFLocatable for CSFade<'_> {
    const DLRF_NAME: &'static str = "CSFade";
}

#[repr(C)]
pub struct CSFD4FadeSystem {
    pub vftable: usize,
}

#[repr(C)]
pub struct CSFD4FadePlate {
    pub vftable: usize,
    pub reference_count: u32,
    _padc: u32,
    pub current_color: CSFD4FadePlateColor,
    pub transition_color: CSFD4FadePlateColor,
    pub target_color: CSFD4FadePlateColor,
    pub fade_timer: FD4Time,
    pub fade_duration: FD4Time,
    pub unk60: u8,
    _pad64: [u8; 7],
    pub allocator: usize,
    pub title: [u16; 8],
    pub unk80: u64,
    pub unk88: u64,
    pub unk90: u64,
    pub unk98: u64,
    pub unka0: u64,
    pub unka8: FD4Time,
    pub unkb8: u64,
}

impl CSFD4FadePlate {
    pub fn fade_out(&mut self, time: f32) {
        self.target_color.a = 1.0;
        self.transition_color.a = 0.0;
        self.fade_duration.time = time;
        self.fade_timer.time = time;
    }

    pub fn fade_in(&mut self, time: f32) {
        self.target_color.a = 0.0;
        self.transition_color.a = 1.0;
        self.fade_duration.time = time;
        self.fade_timer.time = time;
    }
}

#[repr(C)]
pub struct CSFD4FadePlateColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<&CSFD4FadePlateColor> for [f32; 4] {
    fn from(val: &CSFD4FadePlateColor) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

impl From<[f32; 4]> for CSFD4FadePlateColor {
    fn from(val: [f32; 4]) -> Self {
        Self { r: val[0], g: val[1], b: val[2], a: val[3] }
    }
}
