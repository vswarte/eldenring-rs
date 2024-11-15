#[repr(C)]
#[derive(Debug)]
#[dlrf::singleton("CSMenuMan")]
pub struct CSMenuMan {
    vftable: usize,
    unk8: [u8; 0x78],
    pub popup_menu: usize,
}
