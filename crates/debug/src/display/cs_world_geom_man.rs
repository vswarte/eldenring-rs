use game::cs::CSWorldGeomMan;
use hudhook::imgui::Ui;

use super::DebugDisplay;

impl DebugDisplay for CSWorldGeomMan {
    fn render_debug(&self, _ui: &&mut Ui) {
        // if ui.button("Spawn") {
        //     let result = spawn_asset("AEG099_653", None);
        // }
    }
}

// pub fn spawn_asset(asset: &str, detail_override: Option<GeometrySpawnParameters>) -> Result<u64, SpawnAssetError> {
//     let spawn_geomtry_fn = find_pattern(SPAWN_GEOMETRY_AOB)
//         .ok_or(SpawnAssetError::SpawnGeometryFnNotFound)?;
//     let get_map_index_fn = find_pattern(GET_MAP_INDEX_AOB)
//         .ok_or(SpawnAssetError::GetMapIndexFnNotFound)?;
//     let initialize_geometry_spawn_request_fn = find_pattern(INITIALIZE_SPAWN_GEOMETRY_REQUEST_AOB)
//         .ok_or(SpawnAssetError::InitializeGeometrySpawnRequestFnNotFound)?;
//     let world_geom_man = get_instance::<CSWorldGeomMan>()
//         .map_err(|_| SpawnAssetError::SpawnGeometryFnNotFound)?
//         .ok_or(SpawnAssetError::WorldChrManInstanceNotFound)?;
//     let world_chr_man = get_instance::<WorldChrMan>()
//         .map_err(|_| SpawnAssetError::WorldChrManInstanceNotFound)?
//         .ok_or(SpawnAssetError::WorldChrManInstanceNotFound)?;
//
//     let main_player = if world_chr_man.main_player as usize == 0x0 {
//         return Err(SpawnAssetError::NoMainPlayerInstance);
//     } else {
//         world_chr_man.main_player as usize
//     };
//
//     let mut spawn_request = GeometrySpawnRequest::default();
//     let initialize_spawn_request = unsafe { mem::transmute::<_, fn(*const GeometrySpawnRequest, u32) -> u64>(initialize_geometry_spawn_request_fn) };
//
//     // Have the game fill in the default spawn properties
//     initialize_spawn_request(&spawn_request as *const GeometrySpawnRequest, 0x5);
//
//     // Specify what asset we want to spawn
//     set_asset_name(&mut spawn_request, asset);
//
//     let spawn_parameters = detail_override
//         .unwrap_or(get_default_geometry_spawn_parameters(main_player));
//
//     spawn_request.pos_x = spawn_parameters.pos_x;
//     spawn_request.pos_y = spawn_parameters.pos_y;
//     spawn_request.pos_z = spawn_parameters.pos_z;
//     spawn_request.rot_x = spawn_parameters.rot_x;
//     spawn_request.rot_y = spawn_parameters.rot_y;
//     spawn_request.rot_z = spawn_parameters.rot_z;
//     spawn_request.scale_x = spawn_parameters.scale_x;
//     spawn_request.scale_y = spawn_parameters.scale_y;
//     spawn_request.scale_z = spawn_parameters.scale_z;
//
//     // Acquire the structure we'll be inserting the geometry into
//     let get_map_index = unsafe { mem::transmute::<_, fn(usize, *const i32) -> u64>(get_map_index_fn) };
//
//     let map_index = get_map_index(
//         world_geom_man as *mut CSWorldGeomMan as usize,
//         &spawn_parameters.map_id as *const MapId as *const i32,
//     );
//
//     if map_index == 0x0 {
//         return Err(SpawnAssetError::NoMapIndexFound);
//     }
//
//     let spawn = unsafe { mem::transmute::<_, fn(u64, *const GeometrySpawnRequest) -> u64>(spawn_geomtry_fn) };
//
//     Ok(spawn(map_index, &spawn_request as *const GeometrySpawnRequest))
// }
//
// pub fn get_default_geometry_spawn_parameters(player_ins: usize) -> GeometrySpawnParameters {
//     unsafe {
//         GeometrySpawnParameters {
//             map_id: *((player_ins + 0x30) as *const MapId),
//             pos_x: *((player_ins + 0x6C0) as *const f32),
//             pos_y: *((player_ins + 0x6C4) as *const f32),
//             pos_z: *((player_ins + 0x6C8) as *const f32),
//             rot_x: 0.0,
//             rot_y: 0.0,
//             rot_z: 0.0,
//             scale_x: 1.0,
//             scale_y: 1.0,
//             scale_z: 1.0,
//         }
//     }
// }
