use std::fmt::Display;

use super::MapId;

#[derive(Debug)]
/// Used to reference a specific FieldIns managed by its respective (external) domain.
pub struct FieldInsSelector(pub u32);

#[repr(C)]
/// Used throughout the game engine to refer to characters, geometry, bullets, hits and more.
///
/// Source of name: Destructor reveals this being a field in FieldIns and it's used as a means of
/// naming some FieldIns derivant everywhere where raw pointers cannot be shared.
pub struct FieldInsHandle {
    pub selector: FieldInsSelector,
    pub map_id: MapId,
}

impl Display for FieldInsHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.selector.0 == u32::MAX {
            write!(f, "FieldIns(None)")
        } else {
            write!(f, "FieldIns({}, {}, {})", self.map_id, self.selector.container(), self.selector.index())
        }
    }
}

impl FieldInsSelector {
    /// Extracts the type map index
    pub fn mapping_entry_index(&self) -> u32 {
        0xF & (self.0 >> 0x1C)
    }

    /// Retrieves the type map entry for this selector.
    fn mapping(&self) -> &'static FieldInsMapping {
        &FIELD_INS_TYPE_MAPPING[self.mapping_entry_index() as usize]
    }
    
    /// Extracts the container for this FieldInsSelector. Used to ex: determine the
    /// appropriate ChrSet to be calling into for a given NPC. Not used for every type.
    pub fn container(&self) -> u32 {
        self.mapping().container_mask & self.0 >> (self.mapping().container_shift & 0b00111111)
    }
    
    /// Extracts the index within the container for a given FieldIns.
    pub fn index(&self) -> u32 {
        (self.mapping().index_mask & self.0) & 0xFFFFF
    }
}

struct FieldInsMapping {
    container_mask: u32,
    container_shift: u32,
    index_mask: u32,
    unkc: u32,
}

const FIELD_INS_TYPE_MAPPING: &[FieldInsMapping] = &[
    // HIT
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x5,
    },
    // CHR
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x2,
    },
    // OBJ
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x1,
    },
    // BULLET
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xE,
    },
    // SFX
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xE,
    },
    // SOUND
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xE,
    },
    // GEOM
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xD,
    },
    // MAP
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0x0,
    },
    // GEOM(Hit)
    FieldInsMapping {
        container_mask: 0xFF,
        container_shift: 0x14,
        index_mask: 0xFFFFF,
        unkc: 0xD,
    },
];

