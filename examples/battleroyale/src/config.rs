use std::{collections::HashMap, error::Error, sync::RwLock};

use game::{cs, matrix::FSPoint, position::BlockPoint};
use serde::{Deserialize, Serialize};

pub struct ConfigurationProvider {
    config: RwLock<Configuration>,
}

impl ConfigurationProvider {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let config_str = std::fs::read_to_string("battleroyale.toml")?;
        let maps = toml::from_str::<Configuration>(config_str.as_str())?.maps;

        Ok(Self {
            config: RwLock::new(Configuration {
                maps,
            }),
        })
    }

    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        let handle = std::fs::File::create("./br_player_spawns.csv")?;
        let config = self.config.read().unwrap();
        let writer = csv::Writer::from_writer(handle);

        Ok(())
    }

    /// Retrieve the configuration for a single map entry.
    pub fn map(&self, map: &u32) -> Option<MapConfiguration> {
        self.config.read().unwrap().maps.get(map.to_string().as_str()).cloned()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    maps: HashMap<String, MapConfiguration>,
    // ring: RingConfiguration,
}

#[derive(Clone, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapPoint {
    /// Map ID to load into
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: MapPosition,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapPosition(pub f32, pub f32, pub f32);

impl Into<BlockPoint> for &MapPosition {
    fn into(self) -> BlockPoint {
        BlockPoint(FSPoint(self.0, self.1, self.2))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct MapId(pub i32);

#[derive(Deserialize, Serialize)]
pub struct RingConfiguration {

}

#[derive(Deserialize, Serialize)]
pub struct LootConfiguration {

}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootTable {
    pub drops: Vec<LootTableEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootTableEntry {
    pub weight: u32,
    pub items: Vec<LootTableEntryItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootTableEntryItem {
    pub item: u32,
    pub quantity: u32,
}
