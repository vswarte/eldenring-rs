use crate::pointer::OwnedPtr;

// Source of name: RTTI
#[dlrf::singleton("CSHavokMan")]
#[repr(C)]
pub struct CSHavokMan {
    vftable: usize,
    unk8: [u8; 0x90],
    pub phys_world: OwnedPtr<CSPhysWorld>,
}

// Source of name: RTTI
#[repr(C)]
pub struct CSPhysWorld {
    // TODO
}
