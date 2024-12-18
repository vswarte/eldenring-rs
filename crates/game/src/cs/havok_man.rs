use crate::pointer::OwnedPtr;

// Source of name: RTTI
#[dlrf::singleton("CSHavokMan")]
#[repr(C)]
pub struct CSHavokMan {
    pub unk0: [u8; 0x98],
    pub phys_world: OwnedPtr<CSPhysWorld>,
}

// Source of name: RTTI
#[repr(C)]
pub struct CSPhysWorld {
    // TODO
}
