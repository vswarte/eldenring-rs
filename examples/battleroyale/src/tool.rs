use game::cs::WorldChrMan;
use util::singleton::get_instance;

use crate::mapdata::MapPoint;

/// Defines some utilities around creating maps
pub fn sample_spawn_point() {
    let Some(main_player) = unsafe { get_instance::<WorldChrMan>() }
        .unwrap()
        .and_then(|w| w.main_player.as_ref())
    else {
        return;
    };

    let point = MapPoint {
        map: main_player.chr_ins.map_id_1,
        position: main_player.block_position,
        orientation: main_player.block_orientation,
    };

    tracing::info!("Sample spawn point: {point:#?}");
}
