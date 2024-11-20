use std::{
    error::Error,
    f32::consts::PI,
    mem::forget,
    sync::{LazyLock, Mutex}
};

use game::{
    cs::{CSCamera, CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, WorldChrMan}, fd4::FD4TaskData, position::ChunkPosition
};
use nalgebra_glm::{Mat4, Vec3};
use thiserror::Error;
use tracing_panic::panic_hook;
use util::{
    camera::CSCamExt, geometry::{CSWorldGeomManExt, GeometrySpawnParameters, SpawnGeometryError}, input::is_key_pressed, singleton::get_instance, task::CSTaskImpExt
};

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
    let task = unsafe { get_instance::<CSTaskImp>() }.unwrap().unwrap();
    let task = task.run_recurring(
        move |_: &FD4TaskData| {
            let mut builder_camera = BUILDER_CAMERA.lock().unwrap();
            if let Some(camera) = unsafe { get_instance::<CSCamera>() }.unwrap() {
                is_key_pressed(0x42).then(|| builder_camera.toggle(camera));

                builder_camera.apply(camera);
            }

            is_key_pressed(0x68).then(|| builder_camera.move_camera(Movement::Forward));
            is_key_pressed(0x62).then(|| builder_camera.move_camera(Movement::Backward));
            is_key_pressed(0x64).then(|| builder_camera.move_camera(Movement::Left));
            is_key_pressed(0x66).then(|| builder_camera.move_camera(Movement::Right));
            is_key_pressed(0x6E).then(|| builder_camera.cycle_asset());
            is_key_pressed(0x65).then(|| builder_camera.place_asset());
        },
        CSTaskGroupIndex::CameraStep,
    );
    forget(task);

    Ok(())
}

const BUILDER_CAMERA_HEIGHT: f32 = 10.0;
const GRID_TILE_SIZE: f32 = 1.0;

const SPAWNABLE_ASSETS: [&str; 10] = [
    "AEG221_307", // Fancy wooden chair
    "AEG221_521", // Big bookcase
    "AEG221_533", // Fancy bed
    "AEG227_005", // Banished knight armor on standtask_data
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
        selected_asset: 0,
        origin: None,
        lookdown: {
            nalgebra_glm::rotate_x(
                &nalgebra_glm::translation(&Vec3::new(0.0, BUILDER_CAMERA_HEIGHT, 0.0)),
                PI / 2.0,
            )
        },
        previous_camera_coordinates: Default::default(),
        target_camera_coordinates: Default::default(),
    })
});

pub struct BuilderCamera {
    selected_asset: usize,
    origin: Option<Mat4>,
    lookdown: Mat4,
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

        // let offset = (self.target_camera_coordinates - self.previous_camera_coordinates);

        let camera_worldspace = nalgebra_glm::translate(
            &origin,
            &self.target_camera_coordinates,
        );

        let camera_matrix = camera_worldspace * self.lookdown;
        camera.pers_cam_1.matrix = camera_matrix.into();
        camera.pers_cam_2.matrix = camera_matrix.into();
        camera.pers_cam_3.matrix = camera_matrix.into();
        camera.pers_cam_4.matrix = camera_matrix.into();
    }

    fn move_camera(&mut self, movement: Movement) {
        self.previous_camera_coordinates = self.target_camera_coordinates;

        match movement {
            Movement::Forward => self.target_camera_coordinates.z += GRID_TILE_SIZE,
            Movement::Backward => self.target_camera_coordinates.z -= GRID_TILE_SIZE,
            Movement::Right => self.target_camera_coordinates.x += GRID_TILE_SIZE,
            Movement::Left => self.target_camera_coordinates.x -= GRID_TILE_SIZE,
            Movement::Up => self.target_camera_coordinates.y += GRID_TILE_SIZE,
            Movement::Down => self.target_camera_coordinates.y -= GRID_TILE_SIZE,
        };
    }

    fn toggle(&mut self, camera: &mut CSCamera) {
        if self.origin.is_some() {
            self.origin = None;
        } else {
            self.origin = Some(camera.pers_cam_1.matrix.clone().into());
        }
    }

    fn place_asset(&self) -> Result<(), AssetPlaceError> {
        // Dont do shit when we're not in builder mode
        if self.origin.is_none() {
            return Ok(());
        }

        let main_player = unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|w| w.main_player.as_ref())
            .ok_or(AssetPlaceError::MainPlayerMissing)?;

        // Get the camera's position relative to the physics space center
        let camera = unsafe { get_instance::<CSCamera>() }
            .unwrap()
            .ok_or(AssetPlaceError::CSCameraMissing)?;

        let world_geom_man = unsafe { get_instance::<CSWorldGeomMan>() }
            .unwrap()
            .ok_or(AssetPlaceError::WorldGeomManMissing)?;

        let player_physics_pos = main_player.chr_ins.module_container.physics.position;
        let camera_physics_pos = camera.pers_cam_1.position();
        let player_chunk_pos = &main_player.chunk_position.xyz();
        let physics_pos_delta = (camera_physics_pos - player_physics_pos).xyz();

        // Calculated chunk-relative position of camera
        let camera_chunk_pos = ChunkPosition::from_xyz(
            player_chunk_pos.0 + physics_pos_delta.0,
            player_chunk_pos.1 + physics_pos_delta.1,
            player_chunk_pos.2 + physics_pos_delta.2,
        ).xyz();

        world_geom_man.spawn_geometry(
            SPAWNABLE_ASSETS[self.selected_asset],
            &GeometrySpawnParameters {
                map_id: main_player.chr_ins.map_id_1,
                position: ChunkPosition::from_xyz(
                    camera_chunk_pos.0,
                    player_chunk_pos.1,
                    camera_chunk_pos.2,
                ),
                rot_x: 0.0,
                rot_y: 61.0 * (PI / 180.0),
                rot_z: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
            },
        )?;

        Ok(())
    }

    fn cycle_asset(&mut self) {
        self.selected_asset = (self.selected_asset + 1) % SPAWNABLE_ASSETS.len();
        tracing::info!("Cycled asset to {}", SPAWNABLE_ASSETS[self.selected_asset]);
        // unsafe { Self::notify() };
    }
}

#[derive(Debug, Error)]
pub enum AssetPlaceError {
    #[error("Missing WorldChrMan")]
    WorldChrManMissing,
    #[error("Missing WorldGeomMan")]
    WorldGeomManMissing,
    #[error("Missing CSCamera")]
    CSCameraMissing,
    #[error("Missing main player")]
    MainPlayerMissing,
    #[error("Geometry spawn error. {0}.")]
    GeometrySpawn(#[from] SpawnGeometryError),
}
