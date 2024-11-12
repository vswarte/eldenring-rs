use crate::pointer::OwningPtr;
use crate::DLRFLocatable;
use crate::fd4::FD4Time;

#[repr(C)]
/// Controls fades in the game. Used for cutscene transitions and such.
///
/// Source of name: RTTI
pub struct CSFade {
    vftable: usize,
    pub fade_system: OwningPtr<CSFD4FadeSystem>,
    /// Holds the individual fade plates, these control the actual drawing of the dimming.
    pub fade_plates: [OwningPtr<CSFD4FadePlate>; 9],
    pub unk58: u32,
    pub unk5c: f32,
}

impl DLRFLocatable for CSFade {
    const DLRF_NAME: &'static str = "CSFade";
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSFD4FadeSystem {
    vftable: usize,
}

#[repr(C)]
/// A fade plate 
///
/// Source of name: RTTI
pub struct CSFD4FadePlate {
    vftable: usize,
    pub reference_count: u32,
    _padc: u32,
    /// Stores the currently interpolated color.
    pub current_color: CSFD4FadePlateColor,
    /// Stores the color we're transitioning away from.
    pub start_color: CSFD4FadePlateColor,
    /// Stores the color we're transitioning towards.
    pub end_color: CSFD4FadePlateColor,
    /// Stores the amount of seconds pending until the LERP to end_color is finished. 
    pub fade_timer: FD4Time,
    /// Stores the time a transition to the target color should take in total.
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
