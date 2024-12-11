use std::{collections::HashMap, error::Error};

use game::{cs, position::BlockPoint};
use serde::{Deserialize, Serialize};

use crate::mapdata;

pub fn retrieve_config() -> Result<Configuration, Box<dyn Error>> {
    let config_str = std::fs::read_to_string("battleroyale.toml")?;
    Ok(toml::from_str::<Configuration>(config_str.as_str())?)
}

pub fn export_config(configuration: &Configuration) -> Result<(), Box<dyn Error>> {
    let string = toml::to_string_pretty(configuration)?;
    std::fs::write("battleroyale.toml.out", string.as_bytes())?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub maps: HashMap<String, MapConfiguration>,
}

#[derive(Deserialize, Serialize)]
pub struct PainRingConfiguration {}

#[derive(Deserialize, Serialize)]
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

impl From<&mapdata::MapConfiguration> for MapConfiguration {
    fn from(value: &mapdata::MapConfiguration) -> Self {
        Self {
            player_spawn_points: value
                .player_spawn_points
                .iter()
                .map(MapPoint::from)
                .collect(),
            item_spawn_points: value.item_spawn_points.iter().map(MapPoint::from).collect(),
            ring_centers: value.ring_centers.iter().map(MapPoint::from).collect(),
            event_flag_overrides: value.event_flag_overrides.clone(),
        }
    }
}

impl Into<mapdata::MapConfiguration> for &MapConfiguration {
    fn into(self) -> mapdata::MapConfiguration {
        mapdata::MapConfiguration {
            player_spawn_points: self.player_spawn_points.iter().map(|p| p.into()).collect(),
            item_spawn_points: self.item_spawn_points.iter().map(|p| p.into()).collect(),
            ring_centers: self.ring_centers.iter().map(|p| p.into()).collect(),
            event_flag_overrides: self.event_flag_overrides.clone(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MapPoint {
    /// Map ID to load into
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: MapPosition,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
}

impl From<&mapdata::MapPoint> for MapPoint {
    fn from(value: &mapdata::MapPoint) -> Self {
        Self {
            map: MapId(
                value.map.area,
                value.map.block,
                value.map.region,
                value.map.index,
            ),
            position: MapPosition(
                value.position.0 .0,
                value.position.0 .1,
                value.position.0 .2,
            ),
            orientation: value.orientation,
        }
    }
}

impl Into<mapdata::MapPoint> for &MapPoint {
    fn into(self) -> mapdata::MapPoint {
        mapdata::MapPoint {
            map: cs::MapId::from_parts(self.map.0, self.map.1, self.map.2, self.map.3),
            position: BlockPoint::from_xyz(self.position.0, self.position.1, self.position.2),
            orientation: self.orientation,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MapPosition(f32, f32, f32);

#[derive(Deserialize, Serialize)]
pub struct MapId(i8, i8, i8, i8);
