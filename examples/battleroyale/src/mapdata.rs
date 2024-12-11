use std::sync::LazyLock;

use game::{
    cs::MapId,
    position::BlockPoint,
};

use crate::config;

pub fn get(map: u32) -> Option<MapConfiguration> {
    let config = config::retrieve_config().expect("Could not retrieve config");
    config.maps.get(&map.to_string()).map(|m| m.into())
}

pub struct MapConfiguration {
    /// Spawn points for this map.
    pub player_spawn_points: Vec<MapPoint>,
    /// Spawn points for this map.
    pub item_spawn_points: Vec<MapPoint>,
    /// Centers for the shrinking play area boundaries.
    pub ring_centers: Vec<MapPoint>,
    /// Event flags that need to be set while loading this map.
    pub event_flag_overrides: Vec<(u32, bool)>,
}

#[derive(Clone, Debug)]
pub struct MapPoint {
    /// Map ID to load into
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: BlockPoint,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
}
