use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Read, Write},
    sync::RwLock,
};

use game::{cs, matrix::FSPoint, position::BlockPoint};
use serde::{Deserialize, Serialize};

pub struct ConfigurationProvider {
    config: RwLock<Configuration>,
}

impl ConfigurationProvider {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            config: RwLock::new(Self::fetch_config()?),
        })
    }

    pub fn reload(&self) -> Result<(), Box<dyn Error>> {
        tracing::info!("Reload configuration");
        let config = Self::fetch_config()?;
        *self.config.write().unwrap() = config;
        Ok(())
    }

    fn fetch_config() -> Result<Configuration, Box<dyn Error>> {
        let event_flag_overrides = unsafe {
            let mut reader =
                csv::Reader::from_reader(File::open("./battleroyale/0_event_flags.csv")?);

            reader
                .records()
                .map(|r| {
                    let r = r?;
                    Ok((r[1].parse::<u32>()?, r[2].parse::<bool>()?))
                })
                .collect::<Result<Vec<(u32, bool)>, Box<dyn Error>>>()?
        };

        Ok(Configuration {
            loot: read_loot_collection(File::open("./battleroyale/0_loot.csv")?)?,
            maps: HashMap::from([(
                String::from("0"),
                MapConfiguration {
                    bespoke_monster_spawns: read_bespoke_monster_spawn_collection(File::open(
                        "./battleroyale/0_monster_spawns_bespoke.csv",
                    )?)?,
                    player_spawn_points: read_player_spawn_collection(File::open(
                        "./battleroyale/0_player_spawns.csv",
                    )?)?,
                    item_spawn_points: read_loot_spawn_collection(File::open(
                        "./battleroyale/0_item_spawns.csv",
                    )?)?,
                    ring_centers: read_ring_center_collection(File::open(
                        "./battleroyale/0_ring_centers.csv",
                    )?)?,
                    event_flag_overrides,
                    monster_spawn_points: read_monster_spawn_collection(File::open(
                        "./battleroyale/0_monster_spawns.csv",
                    )?)?,
                    monster_types: read_monster_type_collection(File::open(
                        "./battleroyale/0_monster_types.csv",
                    )?)?,
                },
            )]),
        })
    }

    /// Retrieve the configuration for a single map entry.
    pub fn map(&self, map: &u32) -> Option<MapConfiguration> {
        self.config
            .read()
            .unwrap()
            .maps
            .get(map.to_string().as_str())
            .cloned()
    }

    pub fn loot(&self) -> Vec<LootTableEntry> {
        self.config.read().unwrap().loot.clone()
    }
}

fn read_player_spawn_collection<R>(reader: R) -> Result<Vec<PlayerSpawnPoint>, Box<dyn Error>>
where
    R: Read,
{
    tracing::info!("Parsing player spawns");
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let map = i32::from_str_radix(&r[1], 16)?;
            Ok(PlayerSpawnPoint {
                map: MapId(map),
                position: MapPosition {
                    0: r[2].parse::<f32>()?,
                    1: r[3].parse::<f32>()?,
                    2: r[4].parse::<f32>()?,
                },
                orientation: r[5].parse::<f32>()?,
            })
        })
        .collect::<Result<Vec<PlayerSpawnPoint>, Box<dyn Error>>>()?;

    Ok(result)
}

fn read_ring_center_collection<R>(reader: R) -> Result<Vec<RingCenterPoint>, Box<dyn Error>>
where
    R: Read,
{
    tracing::info!("Parsing ring spawns");
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let map = i32::from_str_radix(&r[1], 16)?;
            Ok(RingCenterPoint {
                map: MapId(map),
                position: MapPosition {
                    0: r[2].parse::<f32>()?,
                    1: r[3].parse::<f32>()?,
                    2: r[4].parse::<f32>()?,
                },
            })
        })
        .collect::<Result<Vec<RingCenterPoint>, Box<dyn Error>>>()?;

    Ok(result)
}

fn read_loot_spawn_collection<R>(reader: R) -> Result<Vec<LootSpawnPoint>, Box<dyn Error>>
where
    R: Read,
{
    tracing::info!("Parsing loot spawns");
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let map = i32::from_str_radix(&r[1], 16)?;
            Ok(LootSpawnPoint {
                map: MapId(map),
                position: MapPosition {
                    0: r[2].parse::<f32>()?,
                    1: r[3].parse::<f32>()?,
                    2: r[4].parse::<f32>()?,
                },
                orientation: r[5].parse::<f32>()?,
                pool: r[6].parse::<u32>()?,
            })
        })
        .collect::<Result<Vec<LootSpawnPoint>, Box<dyn Error>>>()?;

    Ok(result)
}

fn read_loot_collection<R>(reader: R) -> Result<Vec<LootTableEntry>, Box<dyn Error>>
where
    R: Read,
{
    tracing::info!("Parsing loot table");
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let weight = r[1].parse::<u32>()?;
            let pool = r[2].parse::<u32>()?;

            let mut items = vec![];
            for i in 0..4 {
                let item_id = &r[3 + (i * 2) + 0];
                if item_id == "ffffffff" {
                    continue;
                }

                let item = i32::from_str_radix(item_id, 16)?;
                let quantity = r[3 + (i * 2) + 1].parse::<u32>()?;

                items.push(LootTableEntryItem { item, quantity });
            }

            Ok(LootTableEntry {
                weight,
                pool,
                items,
            })
        })
        .collect::<Result<Vec<LootTableEntry>, Box<dyn Error>>>()?;

    Ok(result)
}

fn read_bespoke_monster_spawn_collection<R>(
    reader: R,
) -> Result<Vec<BespokeMonsterSpawnPoint>, Box<dyn Error>>
where
    R: Read,
{
    tracing::info!("Parsing bespoke monsters");
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let map = i32::from_str_radix(&r[1], 16)?;
            Ok(BespokeMonsterSpawnPoint {
                map: MapId(map),
                position: MapPosition {
                    0: r[2].parse::<f32>()?,
                    1: r[3].parse::<f32>()?,
                    2: r[4].parse::<f32>()?,
                },
                orientation: r[5].parse::<f32>()?,
                asset: r[6].to_string(),
                npc_id: r[7].parse::<i32>()?,
                think_id: r[8].parse::<i32>()?,
                scaling_sp_effect: r[9].parse::<u32>()?,
                item_pool: r[10].parse::<u32>()?,
                spawn_count: r[11].parse::<u32>()?,
            })
        })
        .collect::<Result<Vec<BespokeMonsterSpawnPoint>, Box<dyn Error>>>()?;

    Ok(result)
}

fn read_monster_spawn_collection<R>(reader: R) -> Result<Vec<MonsterSpawnPoint>, Box<dyn Error>>
where
    R: Read,
{
    tracing::info!("Parsing monster spawns");
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let map = i32::from_str_radix(&r[1], 16)?;
            Ok(MonsterSpawnPoint {
                map: MapId(map),
                position: MapPosition {
                    0: r[2].parse::<f32>()?,
                    1: r[3].parse::<f32>()?,
                    2: r[4].parse::<f32>()?,
                },
                orientation: r[5].parse::<f32>()?,
                pool: r[6].parse::<u32>()?,
            })
        })
        .collect::<Result<Vec<MonsterSpawnPoint>, Box<dyn Error>>>()?;

    Ok(result)
}

fn read_monster_type_collection<R>(reader: R) -> Result<Vec<MonsterType>, Box<dyn Error>>
where
    R: Read,
{
    tracing::info!("Parsing monster types");
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            Ok(MonsterType {
                asset: r[1].to_string(),
                npc_id: r[2].parse::<i32>()?,
                think_id: r[3].parse::<i32>()?,
                scaling_sp_effect: r[4].parse::<u32>()?,
                item_pool: r[5].parse::<u32>()?,
                weight: r[6].parse::<u32>()?,
                spawn_count: r[7].parse::<u32>()?,
                pool: r[8].parse::<u32>()?,
            })
        })
        .collect::<Result<Vec<MonsterType>, Box<dyn Error>>>()?;

    Ok(result)
}

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    loot: Vec<LootTableEntry>,
    maps: HashMap<String, MapConfiguration>,
    // ring: RingConfiguration,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct MapConfiguration {
    /// Player spawn points for this map.
    pub player_spawn_points: Vec<PlayerSpawnPoint>,
    /// Item points for this map.
    pub item_spawn_points: Vec<LootSpawnPoint>,
    /// Centers for the shrinking play area boundaries.
    pub ring_centers: Vec<RingCenterPoint>,
    /// Event flags that need to be set while loading this map.
    pub event_flag_overrides: Vec<(u32, bool)>,
    /// Fixed location mobs.
    pub bespoke_monster_spawns: Vec<BespokeMonsterSpawnPoint>,
    pub monster_spawn_points: Vec<MonsterSpawnPoint>,
    pub monster_types: Vec<MonsterType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerSpawnPoint {
    /// Map ID to load into.
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: MapPosition,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RingCenterPoint {
    /// Map ID to load into.
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: MapPosition,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootSpawnPoint {
    /// Map ID to load into.
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: MapPosition,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
    /// Pool for selecting the loot from.
    pub pool: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonsterSpawnPoint {
    /// Map ID to load into.
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: MapPosition,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
    /// Pool for selecting the monster from.
    pub pool: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MonsterType {
    /// Asset ID to spawn with. Ex: c5240.
    pub asset: String,
    /// NPC ID for params.
    pub npc_id: i32,
    /// NPC think ID for params.
    pub think_id: i32,
    /// Speffect applied to monster on spawn for scaling.
    pub scaling_sp_effect: u32,
    /// Pool to select drops from.
    pub item_pool: u32,
    /// Weight for randomization.
    pub weight: u32,
    /// Amount of monsters to spawn at once.
    pub spawn_count: u32,
    /// Pool for matching against.
    pub pool: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BespokeMonsterSpawnPoint {
    /// Map ID to load into.
    pub map: MapId,
    /// Position on the block to spawn the player at.
    pub position: MapPosition,
    /// Angle on the y axis in radians for the player to spawn with.
    pub orientation: f32,
    /// Asset ID to spawn with. Ex: c5240.
    pub asset: String,
    /// NPC ID for params.
    pub npc_id: i32,
    /// NPC think ID for params.
    pub think_id: i32,
    /// Speffect applied to monster on spawn for scaling.
    pub scaling_sp_effect: u32,
    /// Pool to select drops from.
    pub item_pool: u32,
    /// Amount of monsters to spawn at once.
    pub spawn_count: u32,
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

impl Into<game::cs::MapId> for &MapId {
    fn into(self) -> game::cs::MapId {
        game::cs::MapId(self.0)
    }
}

impl From<&game::cs::MapId> for MapId {
    fn from(value: &game::cs::MapId) -> Self {
        Self(value.0)
    }
}

#[derive(Deserialize, Serialize)]
pub struct RingConfiguration {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootTableEntry {
    pub weight: u32,
    pub pool: u32,
    pub items: Vec<LootTableEntryItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootTableEntryItem {
    pub item: i32,
    pub quantity: u32,
}
