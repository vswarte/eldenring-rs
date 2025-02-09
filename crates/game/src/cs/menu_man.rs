use windows::Win32::Foundation::BOOL;

#[repr(C)]
#[derive(Debug)]
#[dlrf::singleton("CSMenuMan")]
pub struct CSMenuMan {
    vftable: usize,
    unk8: [u8; 0x78],
    pub popup_menu: usize,
    unk88: [u8; 0xB4],
    /// disables all save menu callbacks
    /// additionally, can disable auto save
    pub disable_save_menu: i32,
}
