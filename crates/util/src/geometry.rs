use std::{fmt::Formatter, mem::transmute, sync::LazyLock};

use pelite::pattern;
use pelite::pattern::Atom;
use pelite::pe::Pe;
use thiserror::Error;

use game::cs::{CSWorldGeomMan, GeometrySpawnRequest, MapId};

use crate::program::Program;

const CS_WORLD_GEOM_MAN_MAP_DATA_BY_MAP_ID_PATTERN: &[Atom] =
    pattern!("83 cb 02 89 5c 24 20 48 8d 54 24 38 e8 $ { ' }");
const INITIALIZE_SPAWN_GEOMETRY_REQUEST_PATTERN: &[Atom] = pattern!("b2 08 48 8d 4d 00 e8 $ { ' }");
const SPAWN_GEOMETRY_PATTERN: &[Atom] =
    pattern!("8b 01 89 85 d8 00 00 00 48 8d 55 00 49 8b ce e8 $ { ' }");

#[derive(Debug, Error)]
pub enum SpawnGeometryError {
    #[error("No map data found")]
    MapDataUnavailable,
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

pub trait WorldGeometryFacade {
    fn spawn_geometry(&self, asset: &str, parameters: &GeometrySpawnParameters) -> Result<(), SpawnGeometryError>;
}

impl WorldGeometryFacade for CSWorldGeomMan {
    fn spawn_geometry(
        &self,
        asset: &str,
        parameters: &GeometrySpawnParameters,
    ) -> Result<(), SpawnGeometryError> {
        const CS_WORLD_GEOM_MAN_MAP_DATA_BY_MAP_ID_VA: LazyLock<u64> = LazyLock::new(|| {
            let program = unsafe { Program::current() };
            let mut matches = [0u32; 2];

            if !program
                .scanner()
                .finds_code(CS_WORLD_GEOM_MAN_MAP_DATA_BY_MAP_ID_PATTERN, &mut matches)
            {
                panic!("Could not find CS_WORLD_GEOM_MAN_MAP_DATA_BY_MAP_ID_PATTERN or found duplicates.");
            }

            program.rva_to_va(matches[1]).unwrap()
        });

        const INITIALIZE_SPAWN_GEOMETRY_REQUEST_VA: LazyLock<u64> = LazyLock::new(|| {
            let program = unsafe { Program::current() };
            let mut matches = [0u32; 2];

            if !program
                .scanner()
                .finds_code(INITIALIZE_SPAWN_GEOMETRY_REQUEST_PATTERN, &mut matches)
            {
                panic!("Could not find INITIALIZE_SPAWN_GEOMETRY_REQUEST_PATTERN or found duplicates.");
            }

            program.rva_to_va(matches[1]).unwrap()
        });

        const SPAWN_GEOMETRY_VA: LazyLock<u64> = LazyLock::new(|| {
            let program = unsafe { Program::current() };
            let mut matches = [0u32; 2];

            if !program
                .scanner()
                .finds_code(SPAWN_GEOMETRY_PATTERN, &mut matches)
            {
                panic!("Could not find SPAWN_GEOMETRY_PATTERN or found duplicates.");
            }

            program.rva_to_va(matches[1]).unwrap()
        });

        tracing::debug!("CS::CSWorldGeomMan::MapDataByMapId VA {:x}", *CS_WORLD_GEOM_MAN_MAP_DATA_BY_MAP_ID_VA);
        tracing::debug!("InitializeSpawnGeometryRequest VA {:x}", *INITIALIZE_SPAWN_GEOMETRY_REQUEST_VA);
        tracing::debug!("SpawnGeometry VA {:x}", *SPAWN_GEOMETRY_VA);

        let map_data_by_map_id = unsafe {
            transmute::<_, fn(&CSWorldGeomMan, &MapId) -> u64>(*CS_WORLD_GEOM_MAN_MAP_DATA_BY_MAP_ID_VA)
        };

        let initialize_spawn_geometry_request = unsafe {
            transmute::<_, fn(&mut GeometrySpawnRequest, u32)>(*INITIALIZE_SPAWN_GEOMETRY_REQUEST_VA)
        };

        let spawn_geometry = unsafe {
            transmute::<_, fn(u64, &GeometrySpawnRequest) -> u64>(*SPAWN_GEOMETRY_VA)
        };

        let mut request = GeometrySpawnRequest {
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
        };

        initialize_spawn_geometry_request(&mut request, 0x5);
        request.set_asset(asset);
        request.pos_x = parameters.pos_x;
        request.pos_y = parameters.pos_y;
        request.pos_z = parameters.pos_z;
        request.rot_x = parameters.rot_x;
        request.rot_y = parameters.rot_y;
        request.rot_z = parameters.rot_z;
        request.scale_x = parameters.scale_x;
        request.scale_y = parameters.scale_y;
        request.scale_z = parameters.scale_z;

        tracing::info!("Populated spawn request: {request:#?}");

        // TODO: make this a nice as_ref call or something
        let map_data_ptr = map_data_by_map_id(self, &parameters.map_id);
        tracing::info!("Map Data ptr: {:?} -> {map_data_ptr:#x?}", &parameters.map_id);
        if map_data_ptr == 0x0 {
            return Err(SpawnGeometryError::MapDataUnavailable)
        }

        let geom = spawn_geometry(map_data_ptr, &request);
        tracing::info!("GeomIns: {geom:#x?}");

        Ok(())
    }
}
