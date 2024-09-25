use std::fmt::Formatter;

use thiserror::Error;

use game::cs::MapId;

const SPAWN_GEOMETRY_AOB: &str = concat!(
    // 1406974b0 40 56           PUSH       RSI
    "01000... 01010110",
    // 1406974b2 57              PUSH       RDI
    "01010111",
    // 1406974b3 48 83 ec 58     SUB        RSP,0x58
    "01001... 10000011 11101100 ........",
    // 1406974b7 80 b9 85        CMP        byte ptr [RCX + 0x185],0x0
    //           01 00 00 00
    "10000000 10111001 10000101 00000001 00000000 00000000 00000000",
    // 1406974be 48 8b f2        MOV        RSI,RDX
    "01001... 10001011 11110010",
    // 1406974c1 48 8b f9        MOV        RDI,RCX
    "01001... 10001011 11111001",
);

const GET_MAP_INDEX_AOB: &str = concat!(
    "01001... 10001011 01010001 00100000",
    "01001... 10001011 11000010",
    "01001... 10001011 01000010 00001000",
    "10000000 01111000 00011001 00000000",
);

const INITIALIZE_SPAWN_GEOMETRY_REQUEST_AOB: &str = concat!(
    "01001... 10001001 01001100 ..100100 00001000",
    "01010111",
    "01001... 10000011 11101100 00110000",
    "01001... 11000111 01000100 ..100100 00100000 11111110 11111111 11111111 11111111",
    "01001... 10001001 01011100 ..100100 01001000",
    "00001111 10110110 11011010",
    "01001... 10001011 11111001",
    "01001... 10000011 11001000 11111111",
    "01001... 10001101 00010101 ........ ........ ........ ........",
);

#[derive(Debug, Error)]
pub enum SpawnAssetError {
    #[error("Spawn geometry fn not found")]
    SpawnGeometryFnNotFound,
    #[error("Get map index fn not found")]
    GetMapIndexFnNotFound,
    #[error("Initialize geometry spawn request fn not found")]
    InitializeGeometrySpawnRequestFnNotFound,
    #[error("WorldGeomMan instance not found")]
    WorldGeomManInstanceNotFound,
    #[error("WorldChrMan instance not found")]
    WorldChrManInstanceNotFound,
    #[error("Main player instance not found")]
    NoMainPlayerInstance,
    #[error("No map index found")]
    NoMapIndexFound,
}

pub struct GeometrySpawnParameters {
    pub map_id: MapId,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
}

#[repr(C)]
struct GeometrySpawnRequest {
    asset_string: [u16; 0x20],
    unk0x40: u32,
    asset_string_ptr: u64,
    unk0x50: u32,
    unk0x54: u32,
    unk0x58: u32,
    unk0x5c: u32,
    unk0x60: u32,
    unk0x64: u32,
    unk0x68: u32,
    unk0x6c: u32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    scale_x: f32,
    scale_y: f32,
    scale_z: f32,
    unk0x94: [u8; 0x6C],
}

impl Default for GeometrySpawnRequest {
    fn default() -> Self {
        Self {
            asset_string: [0u16; 0x20],
            unk0x40: 0,
            asset_string_ptr: 0,
            unk0x50: 0,
            unk0x54: 0,
            unk0x58: 0,
            unk0x5c: 0,
            unk0x60: 0,
            unk0x64: 0,
            unk0x68: 0,
            unk0x6c: 0,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            scale_x: 0.0,
            scale_y: 0.0,
            scale_z: 0.0,
            unk0x94: [0u8; 0x6C],
        }
    }
}

impl std::fmt::Debug for GeometrySpawnRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeometrySpawnRequest")
            .field("ptr", &self.asset_string_ptr)
            .field("asset", &get_asset_name(&self.asset_string))
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

fn get_asset_name(v: &[u16; 0x20]) -> String {
    let mut result = String::new();
    for val in v.iter() {
        let c: u8 = (*val & 0xFF) as u8;
        if c == 0 {
            break;
        } else {
            result.push(c as char);
        }
    }
    result
}

fn set_asset_name(request: &mut GeometrySpawnRequest, name: &str) {
    for (i, char) in name.as_bytes().iter().enumerate() {
        request.asset_string[i] = *char as u16;
    }
}
