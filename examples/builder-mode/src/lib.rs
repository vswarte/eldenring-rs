// use std::{f32::consts::PI, mem, sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Mutex, OnceLock}};
//
// use nalgebra::{Matrix1x4, Matrix3x1, Matrix4x1, Perspective3, Projective3, RowVector4};
// use nalgebra_glm::{Mat3, Mat4, Vec3};
// use retour::static_detour;
// use thiserror::Error;
//
// const BUILDER_CAMERA_HEIGHT: f32 = 7.5;
// const GRID_TILE_SIZE: f32 = 1.0;
// const TIMER_VALUE_PER_UPDATE: f32 = 0.1;
//
// use crate::{game::{cs::{CSCamera, CSMenuMan, WorldChrMan}, matrix::Vector4}, geometry::{GeometrySpawnParameters, SpawnAssetError}};
// use util::{self, singleton::{get_instance, DLRFLocatable}};
//
// const SPAWNABLE_ASSETS: [&'static str; 10] = [
//     "AEG221_307", // Fancy wooden chair
//     "AEG221_521", // Big bookcase
//     "AEG221_533", // Fancy bed
//     "AEG227_005", // Banished knight armor on stand
//     "AEG228_130", // Wooden bed with headrest and messy sheets
//     "AEG228_152", // Wooden duo bookcase with golden accents
//     "AEG228_245", // Fancy wooden table with books and golden candlesticks
//     "AEG220_202", // Statue
//     "AEG220_592", // Stone bowl
//     "AEG220_938", // Perfumer incense burner
// ];
//
// #[derive(Debug, Error)]
// pub enum BuilderError {
//     #[error("Builder is not active")]
//     NotActive,
//     #[error("Could not spawn asset {0:?}")]
//     SpawnAsset(#[from] SpawnAssetError),
//     #[error("Missign a base {0}")]
//     MissingBase(&'static str),
// }
//
// // 1403b98f0
// const CAMERA_UPDATE_AOB: &str = concat!(
//     "10001011 01000001 00101000",
//     "01001... 10001011 11001001",
//     "10101000 00100000",
//     "01110100 ........",
//     "01001... 10001011 01010001 00100000",
//     "01001... 10001011 01000001 00001000",
//     "10001011 01000010 01010000",
//     "01000... 10001001 01000000 01010000",
//     "10001011 01000010 01010100",
//     "01000... 10001001 01000000 01010100",
//     "10001011 01000010 01011000",
// );
//
// static_detour! {
//     static CAMERA_UPDATE: extern "C" fn(usize);
// }
//
// pub static BUILDER_CAMERA: OnceLock<Mutex<BuilderCamera>> = OnceLock::new();
//
// pub struct BuilderCamera {
//     hook_initialized: bool,
//     active: bool,
//     selected_asset: usize,
//
//     origin: Option<Mat4>,
//     lookdown: Mat4,
//
//     interpolation_timer: f32,
//     previous_camera_coordinates: Vec3,
//     target_camera_coordinates: Vec3,
// }
//
// impl BuilderCamera {
//     pub fn new() -> Self {
//         Self {
//             hook_initialized: Default::default(),
//             active: Default::default(),
//             selected_asset: 0,
//             origin: None,
//             lookdown: {
//                 nalgebra_glm::rotate_x(
//                     &nalgebra_glm::translation(&&Vec3::new(0.0, BUILDER_CAMERA_HEIGHT, 0.0)),
//                     PI / 2.0,
//                 )
//             },
//
//             interpolation_timer: Default::default(),
//             previous_camera_coordinates: Default::default(),
//             target_camera_coordinates: Default::default(),
//         }
//     }
//
//     pub fn set_origin(&mut self, origin_translation: Vec3) {
//         self.origin = Some({
//             nalgebra_glm::rotate_y(
//                 &nalgebra_glm::translation(&origin_translation.xyz()),
//                 61.0 * (PI / 180.0),
//             )
//         });
//     }
// }
//
// unsafe fn camera_update_hook(camera: usize) {
//     if let Some(camera) = (camera as *mut CSCamera).as_mut() {
//         BUILDER_CAMERA
//             .get().unwrap()
//             .lock().unwrap()
//             .apply(camera);
//     }
// }
//
// pub enum Movement {
//     Forward,
//     Backward,
//     Left,
//     Right,
//     Up,
//     Down,
// }
//
// impl BuilderCamera {
//     pub unsafe fn toggle(&mut self) {
//         if !CAMERA_UPDATE.is_enabled() {
//             if !self.hook_initialized {
//                 self.initialize_hook();
//             }
//
//             self.active = true;
//             self.interpolation_timer = 0.0;
//             self.previous_camera_coordinates = Default::default();
//             self.target_camera_coordinates = Default::default();
//             CAMERA_UPDATE.enable().unwrap();
//         } else {
//             self.active = false;
//             self.origin = None;
//             self.previous_camera_coordinates = Default::default();
//             self.target_camera_coordinates = Default::default();
//             CAMERA_UPDATE.disable().unwrap();
//         }
//     }
//
//     unsafe fn notify() {
//         let menu_man = util::singleton::get_instance::<CSMenuMan>()
//             .unwrap()
//             .unwrap();
//
//         let spawn_modal = unsafe { mem::transmute::<_, fn(usize, u32, bool)>(0x1407dcb90 as usize) };
//         spawn_modal(menu_man.popup_menu, 2, true);
//     }
//
//     unsafe fn initialize_hook(&mut self) {
//         let ptr = find_pattern(CAMERA_UPDATE_AOB)
//             .unwrap();
//
//         CAMERA_UPDATE.initialize(
//             std::mem::transmute(ptr),
//             |camera| camera_update_hook(camera),
//         ).unwrap();
//         self.hook_initialized = true;
//     }
//
//     fn apply(&mut self, camera: &mut CSCamera) {
//         let current: Mat4 = camera.pers_cam_1.matrix.clone().into();
//
//         if self.origin.is_none() {
//             CAMERA_UPDATE.call(camera as *const CSCamera as usize);
//             self.set_origin(current.column(3).xyz());
//         }
//
//         if self.interpolation_timer < 1.0 {
//             self.interpolation_timer += 0.10;
//         }
//
//         if let Some(origin) = self.origin.as_ref() {
//             let animated_delta = (self.target_camera_coordinates - self.previous_camera_coordinates) * self.interpolation_timer;
//             let camera_worldspace = nalgebra_glm::translate(
//                 origin,
//                 &(self.previous_camera_coordinates + animated_delta)
//             );
//
//             let camera_matrix = camera_worldspace * self.lookdown;
//             camera.pers_cam_1.matrix = camera_matrix.clone().into();
//             camera.pers_cam_2.matrix = camera_matrix.clone().into();
//             camera.pers_cam_3.matrix = camera_matrix.clone().into();
//             camera.pers_cam_4.matrix = camera_matrix.clone().into();
//         }
//     }
//
//     pub fn move_camera(&mut self, movement: Movement) {
//         self.previous_camera_coordinates = self.target_camera_coordinates;
//
//         match movement {
//             Movement::Forward => self.target_camera_coordinates.z += GRID_TILE_SIZE,
//             Movement::Backward => self.target_camera_coordinates.z -= GRID_TILE_SIZE,
//             Movement::Right => self.target_camera_coordinates.x += GRID_TILE_SIZE,
//             Movement::Left => self.target_camera_coordinates.x -= GRID_TILE_SIZE,
//             Movement::Up => self.target_camera_coordinates.y += GRID_TILE_SIZE,
//             Movement::Down => self.target_camera_coordinates.y -= GRID_TILE_SIZE,
//         };
//
//         self.interpolation_timer = 0.0;
//     }
//
//     pub fn place_asset(&self) -> Result<(), BuilderError> {
//         if self.active == false {
//             return Err(BuilderError::NotActive);
//         }
//
//         let world_chr_man = get_instance::<WorldChrMan>()
//             .unwrap()
//             .ok_or(BuilderError::MissingBase(WorldChrMan::DLRF_NAME))?;
//
//         let main_player = unsafe {
//             world_chr_man.main_player
//                 .as_ref()
//                 .ok_or(BuilderError::MissingBase("Main player"))?
//         };
//
//         // Get the chr's positon relative to the map center
//         let Vector4(
//             player_map_pos_x,
//             player_map_pos_y,
//             player_map_pos_z,
//             _,
//         ) = main_player.map_relative_position;
//
//         // Get the chr's position relative to the physics space (broad phase?) center
//         let Vector4(
//             player_physics_pos_x,
//             player_physics_pos_y,
//             player_physics_pos_z,
//             _,
//         ) = main_player.chr_ins.module_container.physics.unk70_position;
//
//         // Get the camera's position relative to the physics space center
//         let camera = get_instance::<CSCamera>().unwrap().ok_or(BuilderError::MissingBase(CSCamera::DLRF_NAME))?;
//         let Vector4(
//             camera_physics_pos_x,
//             camera_physics_pos_y,
//             camera_physics_pos_z,
//             _,
//         ) = camera.pers_cam_1.matrix.3;
//
//         // Figure out the camera's world coords and use them for the spawn
//         // TODO: raycast?
//
//         let pos_x = player_map_pos_x + (camera_physics_pos_x - player_physics_pos_x);
//         let pos_y = player_map_pos_y /*+ (camera_physics_pos_y - player_physics_pos_y)*/;
//         let pos_z = player_map_pos_z + (camera_physics_pos_z - player_physics_pos_z);
//
//         let map_id = main_player.chr_ins.map_id_1;
//         let parameters = GeometrySpawnParameters {
//             map_id,
//             pos_x,
//             pos_y,
//             pos_z,
//             rot_x: 0.0,
//             rot_y: 61.0 * (PI / 180.0),
//             rot_z: 0.0,
//             scale_x: 1.0,
//             scale_y: 1.0,
//             scale_z: 1.0,
//         };
//
//         spawn_asset(SPAWNABLE_ASSETS[self.selected_asset], Some(parameters))?;
//
//         Ok(())
//     }
//
//     pub fn cycle_asset(&mut self) {
//         self.selected_asset = (self.selected_asset + 1) % SPAWNABLE_ASSETS.len();
//         unsafe { Self::notify() };
//     }
// }
//
// // TODO: move somewhere else
// pub fn find_pattern(input: &str) -> Option<usize> {
//     let text_section = broadsword::runtime::get_module_section_range("eldenring.exe", ".text")
//         .or_else(|_| broadsword::runtime::get_module_section_range("start_protected_game.exe", ".text"))
//         .unwrap();
//
//     let scan_slice = unsafe {
//         std::slice::from_raw_parts(
//             text_section.start as *const u8,
//             text_section.end - text_section.start,
//         )
//     };
//
//     let pattern = broadsword::scanner::Pattern::from_bit_pattern(input).ok()?;
//     let result = broadsword::scanner::threaded::scan(scan_slice, &pattern, None)?;
//     Some(text_section.start + result.location)
// }
