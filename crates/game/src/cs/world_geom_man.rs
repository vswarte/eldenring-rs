use crate::DLRFLocatable;

#[repr(C)]
#[derive(Debug)]
pub struct CSWorldGeomMan {
    pub vftable: usize,
}

impl DLRFLocatable for CSWorldGeomMan {
    const DLRF_NAME: &'static str = "CSWorldGeomMan";
}
