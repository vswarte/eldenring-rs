use game::cs::WorldChrMan;
use util::singleton::get_instance;

/// Defines some utilities around creating maps
pub fn sample_spawn_point() {
    let Some(main_player) = unsafe { get_instance::<WorldChrMan>() }
        .unwrap()
        .and_then(|w| w.main_player.as_ref())
    else {
        return;
    };

    tracing::info!(
        "Sampled point: map = {:x}. pos = ({} {} {}). angle = {}",
        main_player.chr_ins.map_id_1.0,
        main_player.block_position.0.1,
        main_player.block_position.0.1,
        main_player.block_position.0.2,
        main_player.block_orientation,
    );
}
