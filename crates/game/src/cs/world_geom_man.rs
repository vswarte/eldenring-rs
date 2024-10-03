use std::fmt::Formatter;

use crate::DLRFLocatable;

#[repr(C)]
#[derive(Debug)]
pub struct CSWorldGeomMan {
    pub vftable: usize,
}

impl DLRFLocatable for CSWorldGeomMan {
    const DLRF_NAME: &'static str = "CSWorldGeomMan";
}

#[repr(C)]
pub struct GeometrySpawnRequest {
    pub asset_string: [u16; 0x20],
    pub unk0x40: u32,
    pub asset_string_ptr: u64,
    pub unk0x50: u32,
    pub unk0x54: u32,
    pub unk0x58: u32,
    pub unk0x5c: u32,
    pub unk0x60: u32,
    pub unk0x64: u32,
    pub unk0x68: u32,
    pub unk0x6c: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub unk0x94: [u8; 0x6C],
}

impl GeometrySpawnRequest {
    pub fn asset(&self) -> String {
        let mut result = String::new();
        for val in self.asset_string.iter() {
            let c: u8 = (*val & 0xFF) as u8;
            if c == 0 {
                break;
            } else {
                result.push(c as char);
            }
        }
        result
    }

    // TODO: guard against strings that are too long
    pub fn set_asset(&mut self, asset: &str) {
        for (i, char) in asset.as_bytes().iter().enumerate() {
            self.asset_string[i] = *char as u16;
        }
    }
}

impl std::fmt::Debug for GeometrySpawnRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeometrySpawnRequest")
            .field("asset", &self.asset())
            .field("positionX", &self.pos_x)
            .field("positionY", &self.pos_y)
            .field("positionZ", &self.pos_z)
            .field("rotationX", &self.rot_x)
            .field("rotationY", &self.rot_y)
            .field("rotationZ", &self.rot_z)
            .field("scaleX", &self.scale_x)
            .field("scaleY", &self.scale_y)
            .field("scaleZ", &self.scale_z)
            .finish()
    }
}
