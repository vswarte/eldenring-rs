use crate::DLRFLocatable;

#[repr(C)]
#[derive(Debug)]
pub struct CSMenuMan {
    vftable: usize,
    pub unk8: [u8; 0x78],
    pub popup_menu: usize,
}

impl DLRFLocatable for CSMenuMan {
    const DLRF_NAME: &'static str = "CSMenuMan";
}
