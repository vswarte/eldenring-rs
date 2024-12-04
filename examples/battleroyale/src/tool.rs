use game::cs::WorldChrMan;
use util::singleton::get_instance;

use crate::mapdata::SpawnPoint;

/// Defines some utilities around creating maps
pub fn sample_spawn_point() {
    let Some(main_player) = unsafe { get_instance::<WorldChrMan>() }
        .unwrap()
        .map(|w| w.main_player.as_ref())
        .flatten()
    else {
        return;
    };

    let point = SpawnPoint {
        map: main_player.chr_ins.map_id_1,
        position: main_player.block_position,
        orientation: main_player.block_orientation,
    };

    tracing::info!("Sample spawn point: {point:#?}");
}
