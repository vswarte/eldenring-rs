// TODO: fact-check the u32
#[repr(u32)]
pub enum MENU_MATCHING_BUDDY {
    Prohibited = 0x0,
    Allowed = 0x1,
}

#[repr(u32)]
pub enum MENU_MATCHING_STAGE {
    Any = 0x0,
    Leyndell = 0x1,
    Limgrave = 0x2,
    Caelid = 0x3,
}

#[repr(C)]
pub struct MenuViewItem {
    pub vfptr: usize,
}

#[repr(C)]
// size: 0x48
pub struct ComboItem<T> {
    pub menu_view_item: MenuViewItem,
    pub value: i32,
    unkc: u32,
    // Used if strings memory is hosted else where (eg: fmg)
    pub string: usize,
    // Used if strings memory is hosted by the combo item itself (player names?)
    pub string_override: usize,
    unk40: i32,
    unk44: u32,
}

#[repr(C)]
pub struct ComboItemListVMT<T> {
    pub destructor: fn(*const ComboItemList),
    pub get_option_count: fn(*const ComboItemList<T>) -> u32,
}

#[repr(C)]
pub struct ComboItemList<T, 'a> {
    pub vmt: &'a ComboItemListVMT<T>,
    pub combo_item: ComboItem<T>,
}

#[repr(C)]
pub struct PropertyComboBoxController<T, 'a> {
    pub vmt: usize,
    // pub vmt: &'a ComboItemListVMT<T>,
    // pub combo_item: ComboItem<T>,
}
