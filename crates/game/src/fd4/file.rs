use super::FD4ResCapHeader;

#[repr(C)]
pub struct FD4FileCap<'a, TRes> {
    pub header: FD4ResCapHeader<'a, TRes>,
    pub data: TRes,
}
