use std::{
    cell::RefCell,
    error::Error,
    f32::consts::PI,
    sync::{LazyLock, Mutex},
};

use game::cs::{CSCamera, CSTaskGroupIndex, CSTaskImp};
use nalgebra_glm::{Mat4, Vec3};
use tracing_panic::panic_hook;
use util::{input::is_key_pressed, singleton::get_instance, task::TaskRuntime};

#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == 1 {
        std::panic::set_hook(Box::new(panic_hook));

        let appender = tracing_appender::rolling::never("./", "builder-mode.log");
        tracing_subscriber::fmt().with_writer(appender).init();

        std::thread::spawn(|| {
            // Give the CRT init a bit of leeway
            std::thread::sleep(std::time::Duration::from_secs(5));
            init().expect("Could not initialize mod");
        });
    }

    true
}

fn init() -> Result<(), Box<dyn Error>> {
    while get_instance::<CSTaskImp>().ok().flatten().is_none() {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    let task = get_instance::<CSTaskImp>().unwrap().unwrap();
    std::mem::forget(task.run_task(
        |_, _| {
            let mut builder_camera = BUILDER_CAMERA.lock().unwrap();
            is_key_pressed(0x68).then(|| builder_camera.move_camera(Movement::Forward));
            is_key_pressed(0x62).then(|| builder_camera.move_camera(Movement::Backward));
            is_key_pressed(0x64).then(|| builder_camera.move_camera(Movement::Left));
            is_key_pressed(0x66).then(|| builder_camera.move_camera(Movement::Right));
            is_key_pressed(0x6B).then(|| builder_camera.cycle_asset());

            if let Some(camera) = get_instance::<CSCamera>().unwrap() {
                is_key_pressed(0x42).then(|| builder_camera.toggle(camera));

                builder_camera.apply(camera);
            }
        },
        CSTaskGroupIndex::CameraStep,
    ));

    Ok(())
}

const BUILDER_CAMERA_HEIGHT: f32 = 7.5;
const GRID_TILE_SIZE: f32 = 1.0;
const TIMER_VALUE_PER_UPDATE: f32 = 0.1;

const SPAWNABLE_ASSETS: [&'static str; 10] = [
    "AEG221_307", // Fancy wooden chair
    "AEG221_521", // Big bookcase
    "AEG221_533", // Fancy bed
    "AEG227_005", // Banished knight armor on stand
    "AEG228_130", // Wooden bed with headrest and messy sheets
    "AEG228_152", // Wooden duo bookcase with golden accents
    "AEG228_245", // Fancy wooden table with books and golden candlesticks
    "AEG220_202", // Statue
    "AEG220_592", // Stone bowl
    "AEG220_938", // Perfumer incense burner
];

// TODO: drop mutex since its not necessary
pub static BUILDER_CAMERA: LazyLock<Mutex<BuilderCamera>> = LazyLock::new(|| {
    Mutex::new(BuilderCamera {
        active: Default::default(),
        selected_asset: 0,
        origin: None,
        lookdown: {
            nalgebra_glm::rotate_x(
                &nalgebra_glm::translation(&&Vec3::new(0.0, BUILDER_CAMERA_HEIGHT, 0.0)),
                PI / 2.0,
            )
        },

        interpolation_timer: Default::default(),
        previous_camera_coordinates: Default::default(),
        target_camera_coordinates: Default::default(),
    })
});

pub struct BuilderCamera {
    active: bool,
    selected_asset: usize,

    origin: Option<Mat4>,
    lookdown: Mat4,

    interpolation_timer: f32,
    previous_camera_coordinates: Vec3,
    target_camera_coordinates: Vec3,
}

impl BuilderCamera {
    pub fn set_origin(&mut self, origin_translation: Vec3) {
        self.origin = Some({
            nalgebra_glm::rotate_y(
                &nalgebra_glm::translation(&origin_translation.xyz()),
                61.0 * (PI / 180.0),
            )
        });
    }
}

pub enum Movement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

impl BuilderCamera {
    fn apply(&mut self, camera: &mut CSCamera) {
        let Some(origin) = self.origin else {
            return;
        };

        if self.interpolation_timer < 1.0 {
            self.interpolation_timer += 0.10;
        }

        let animated_delta = (
            self.target_camera_coordinates - self.previous_camera_coordinates
        ) * self.interpolation_timer;

        let camera_worldspace = nalgebra_glm::translate(
            &origin,
            &(self.previous_camera_coordinates + animated_delta),
        );

        let camera_matrix = camera_worldspace * self.lookdown;
        camera.pers_cam_1.matrix = camera_matrix.clone().into();
        camera.pers_cam_2.matrix = camera_matrix.clone().into();
        camera.pers_cam_3.matrix = camera_matrix.clone().into();
        camera.pers_cam_4.matrix = camera_matrix.clone().into();
    }

    pub fn move_camera(&mut self, movement: Movement) {
        self.previous_camera_coordinates = self.target_camera_coordinates;

        match movement {
            Movement::Forward => self.target_camera_coordinates.z += GRID_TILE_SIZE,
            Movement::Backward => self.target_camera_coordinates.z -= GRID_TILE_SIZE,
            Movement::Right => self.target_camera_coordinates.x += GRID_TILE_SIZE,
            Movement::Left => self.target_camera_coordinates.x -= GRID_TILE_SIZE,
            Movement::Up => self.target_camera_coordinates.y += GRID_TILE_SIZE,
            Movement::Down => self.target_camera_coordinates.y -= GRID_TILE_SIZE,
        };

        self.interpolation_timer = 0.0;
    }

    pub fn toggle(&mut self, camera: &mut CSCamera) {
        if self.origin.is_some() {
            self.origin = None;
        } else {
            self.origin = Some(camera.pers_cam_1.matrix.clone().into());
        }
    }

    // pub fn place_asset(&self) -> Result<(), BuilderError> {
    //     if self.active == false {
    //         return Err(BuilderError::NotActive);
    //     }
    //
    //     let world_chr_man = get_instance::<WorldChrMan>()
    //         .unwrap()
    //         .ok_or(BuilderError::MissingBase(WorldChrMan::DLRF_NAME))?;
    //
    //     let main_player = unsafe {
    //         world_chr_man.main_player
    //             .as_ref()
    //             .ok_or(BuilderError::MissingBase("Main player"))?
    //     };
    //
    //     // Get the chr's positon relative to the map center
    //     let Vector4(
    //         player_map_pos_x,
    //         player_map_pos_y,
    //         player_map_pos_z,
    //         _,
    //     ) = main_player.map_relative_position;
    //
    //     // Get the chr's position relative to the physics space (broad phase?) center
    //     let Vector4(
    //         player_physics_pos_x,
    //         player_physics_pos_y,
    //         player_physics_pos_z,
    //         _,
    //     ) = main_player.chr_ins.module_container.physics.unk70_position;
    //
    //     // Get the camera's position relative to the physics space center
    //     let camera = get_instance::<CSCamera>().unwrap().ok_or(BuilderError::MissingBase(CSCamera::DLRF_NAME))?;
    //     let Vector4(
    //         camera_physics_pos_x,
    //         camera_physics_pos_y,
    //         camera_physics_pos_z,
    //         _,
    //     ) = camera.pers_cam_1.matrix.3;
    //
    //     // Figure out the camera's world coords and use them for the spawn
    //     // TODO: raycast?
    //
    //     let pos_x = player_map_pos_x + (camera_physics_pos_x - player_physics_pos_x);
    //     let pos_y = player_map_pos_y /*+ (camera_physics_pos_y - player_physics_pos_y)*/;
    //     let pos_z = player_map_pos_z + (camera_physics_pos_z - player_physics_pos_z);
    //
    //     let map_id = main_player.chr_ins.map_id_1;
    //     let parameters = GeometrySpawnParameters {
    //         map_id,
    //         pos_x,
    //         pos_y,
    //         pos_z,
    //         rot_x: 0.0,
    //         rot_y: 61.0 * (PI / 180.0),
    //         rot_z: 0.0,
    //         scale_x: 1.0,
    //         scale_y: 1.0,
    //         scale_z: 1.0,
    //     };
    //
    //     spawn_asset(SPAWNABLE_ASSETS[self.selected_asset], Some(parameters))?;
    //
    //     Ok(())
    // }

    pub fn cycle_asset(&mut self) {
        self.selected_asset = (self.selected_asset + 1) % SPAWNABLE_ASSETS.len();
        // unsafe { Self::notify() };
    }
}

// TODO: move somewhere else
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
