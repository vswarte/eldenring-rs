use std::{
    error::Error,
    mem::forget,
    thread::{sleep, spawn},
    time::Duration,
};

use game::{cs::{CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, MapId, WorldChrMan}, fd4::FD4TaskData, position::{BlockPoint, HavokPosition}};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing_panic::panic_hook;
use util::{
    geometry::{CSWorldGeomManExt, GeometrySpawnParameters},
    input::is_key_pressed,
    singleton::get_instance,
    task::CSTaskImpExt,
};

#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == 1 {
        std::panic::set_hook(Box::new(panic_hook));

        let appender = tracing_appender::rolling::never("./", "procedural-dungeons.log");
        tracing_subscriber::fmt().with_writer(appender).init();

        spawn(|| {
            // Give the CRT init a bit of leeway
            sleep(Duration::from_secs(5));

            init().expect("Could not initialize mod");
        });
    }

    true
}

fn init() -> Result<(), Box<dyn Error>> {
    let cs_task = unsafe { get_instance::<CSTaskImp>() }.unwrap().unwrap();

    // let prefabs = vec![
    //     Prefab {
    //         name: String::from("hallway-1"),
    //         spawn: Some((0.0, 0.2, 0.0)),
    //         components: vec![
    //             PrefabComponent {
    //                 asset: String::from("AEG020_210"),
    //                 position: (0.0, 0.0, 0.0),
    //                 rotation: (0.0, 0.0, 0.0),
    //             }
    //         ],
    //     },
    // ];
    //
    // let ron = ron::to_string(&prefabs).unwrap();
    // std::fs::write("./prefabs.ron", ron);

    let task = cs_task.run_recurring(
        move |_: &FD4TaskData| {
            let Some(world_geom_man) = unsafe { get_instance::<CSWorldGeomMan>() }.unwrap() else {
                return;
            };

            if is_key_pressed(0x68) {
                spawn(move || {
                    let Some(main_player) = unsafe { get_instance::<WorldChrMan>() }
                        .unwrap()
                        .and_then(|w| unsafe { w.main_player.as_mut() })
                    else {
                        return;
                    };

                    // Generate a dungeon
                    let pos = main_player.block_position.xyz();
                    let dungeon = Dungeon::generate(
                        main_player.chr_ins.map_id_1,
                        (pos.0, pos.1 - 200.0, pos.2),
                    )
                    .unwrap();

                    // Warp player to dungeon
                    let current_pos = main_player.chr_ins.module_container.physics.position.xyz();
                    let new_pos = HavokPosition::from_xyz(
                        current_pos.0,
                        current_pos.1 - 199.8,
                        current_pos.2,
                    );

                    main_player.chr_ins.module_container.physics.position = new_pos;
                });
            }
        },
        CSTaskGroupIndex::FrameBegin,
    );

    forget(task);

    Ok(())
}

#[derive(Debug, Error)]
pub enum DungeonError {
    #[error("Could not find a suitable starting prefab")]
    NoStartingPrefab,
}

struct Dungeon {
    /// Specifies the game's map ID used for the spawn as well as the coordinate system.
    map: MapId,
    /// Map-relative coordinates for the dungeon.
    center: (f32, f32, f32),
    /// Spawn position in world space
    spawn: (f32, f32, f32),
}

impl Dungeon {
    fn generate(map: MapId, center: (f32, f32, f32)) -> Result<Self, DungeonError> {
        // Parse config. TODO: can be passed into the thing.
        let prefabs_str = std::fs::read_to_string("prefabs.ron").unwrap();
        let prefabs: Vec<Prefab> = ron::from_str(&prefabs_str).unwrap();

        // Always start with a prefab on which the player can spawn.
        let start = prefabs
            .iter().find(|p| p.spawn.is_some())
            .ok_or(DungeonError::NoStartingPrefab)?;

        // Calculate the players entry into the dungeon
        let spawn = {
            let prefab_spawn = start.spawn.unwrap();

            (
                center.0 + prefab_spawn.0,
                center.1 + prefab_spawn.1,
                center.2 + prefab_spawn.2,
            )
        };

        let dungeon = Self { map, center, spawn };
        dungeon.spawn_prefab(start);
        Ok(dungeon)
    }

    fn spawn_prefab(&self, prefab: &Prefab) {
        // TODO: get this outta here lmao
        let world_geom_man = unsafe { get_instance::<CSWorldGeomMan>() }.unwrap().unwrap();
        let prefab_origin = &self.center;

        tracing::info!("Spawning prefab {}", prefab.name);
        prefab
            .components
            .iter()
            .map(|c| {
                (
                    &c.asset,
                    Box::leak(Box::new(GeometrySpawnParameters {
                        map_id: self.map,
                        position: BlockPoint::from_xyz(
                            prefab_origin.0 + c.position.0,
                            prefab_origin.1 + c.position.1,
                            prefab_origin.2 + c.position.2,
                        ),
                        rot_x: c.rotation.0,
                        rot_y: c.rotation.1,
                        rot_z: c.rotation.2,
                        scale_x: 1.0,
                        scale_y: 1.0,
                        scale_z: 1.0,
                    })),
                )
            })
            .map(|(a, p)| world_geom_man.spawn_geometry(a, p))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
    }
}

// # Assets
//
// ## Catacombs - Maze pieces
// AEG020_210 Straight hallway
// AEG020_211 Straight hallway, slightly longer
// AEG020_212 Straight hallway, tall
// AEG020_230 T-split
// AEG020_220 Corner hallway
// AEG020_222 Corner hallway, one wall open
// AEG020_260 Corner hallway, bigger
//
// ## Catacombs - Decoration
// AEG020_213 Straight hallway gate (probably for AEG020_211)
//
// AEG020_496 Large piece, probably unusable

#[derive(Serialize, Deserialize)]
struct Prefab {
    /// Name for the prefab.
    name: String,
    /// Optional spawn location within the prefab. If filled the prefab can be used as a spawn
    /// location for dungeon entry.
    spawn: Option<(f32, f32, f32)>,
    /// Components that make up the prefab.
    components: Vec<PrefabComponent>,
}

#[derive(Serialize, Deserialize)]
struct PrefabComponent {
    /// Names the asset that is spawned for this component
    asset: String,
    position: (f32, f32, f32),
    rotation: (f32, f32, f32),
}
