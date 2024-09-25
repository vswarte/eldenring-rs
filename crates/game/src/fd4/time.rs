#[repr(C)]
#[derive(Debug)]
pub struct FD4Time {
    pub vftable: usize,
    pub time: f32,
    _padc: u32,
}
