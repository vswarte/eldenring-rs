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
            config: RwLock::new(Self::fetch_config()?)
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
            let mut reader = csv::Reader::from_reader(File::open("./battleroyale/0_event_flags.csv")?);

            reader
                .records()
                .map(|r| {
                    let r = r?;
                    Ok((
                        r[1].parse::<u32>()?,
                        r[2].parse::<bool>()?,
                    ))
                })
                .collect::<Result<Vec<(u32, bool)>, Box<dyn Error>>>()?
        };

        Ok(Configuration {
            loot: read_loot_collection(File::open(
                "./battleroyale/0_loot.csv",
            )?)?,
            maps: HashMap::from([(
                String::from("0"),
                MapConfiguration {
                    player_spawn_points: read_point_collection(File::open(
                        "./battleroyale/0_player_spawns.csv",
                    )?)?,
                    item_spawn_points: read_point_collection(File::open(
                        "./battleroyale/0_item_spawns.csv",
                    )?)?,
                    ring_centers: read_point_collection(File::open(
                        "./battleroyale/0_ring_centers.csv",
                    )?)?,
                    event_flag_overrides,
                },
            )]),
        })
    }

    pub fn export(&self) -> Result<(), Box<dyn Error>> {
        let config = self.config.read().unwrap();

        // let test = read_loot_collection(File::open("./battleroyale/0_loot.csv")?)?;
        // dbg!(test);

        // Write maps
        config.maps.iter().for_each(|(map, config)| {
            // Write event flag overrides
            {
                let handle = File::create(format!("./battleroyale/{map}_event_flags.csv")).unwrap();
                let mut writer = csv::Writer::from_writer(handle);
                writer
                    .write_record(&["Description", "Event Flag", "State"])
                    .unwrap();

                config
                    .event_flag_overrides
                    .iter()
                    .for_each(|(flag, value)| {
                        writer
                            .write_record(&[String::new(), flag.to_string(), value.to_string()])
                            .unwrap();
                    });
                writer.flush().unwrap();
            }

            write_point_collection(
                File::create(format!("./battleroyale/{map}_player_spawns.csv")).unwrap(),
                config.player_spawn_points.as_slice(),
            )
            .unwrap();

            write_point_collection(
                File::create(format!("./battleroyale/{map}_item_spawns.csv")).unwrap(),
                config.item_spawn_points.as_slice(),
            )
            .unwrap();

            write_point_collection(
                File::create(format!("./battleroyale/{map}_ring_centers.csv")).unwrap(),
                config.ring_centers.as_slice(),
            )
            .unwrap();
        });

        Ok(())
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

fn read_point_collection<R>(reader: R) -> Result<Vec<MapPoint>, Box<dyn Error>>
where
    R: Read,
{
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let map = i32::from_str_radix(&r[1], 16)?;
            Ok(MapPoint {
                map: MapId(map),
                position: MapPosition {
                    0: r[2].parse::<f32>()?,
                    1: r[3].parse::<f32>()?,
                    2: r[4].parse::<f32>()?,
                },
                orientation: r[5].parse::<f32>()?,
            })
        })
        .collect::<Result<Vec<MapPoint>, Box<dyn Error>>>()?;

    Ok(result)
}

fn write_point_collection<W, P>(writer: W, points: P) -> Result<(), Box<dyn Error>>
where
    W: Write,
    P: AsRef<[MapPoint]>,
{
    let mut writer = csv::Writer::from_writer(writer);
    writer.write_record(&["Description", "Map", "X", "Y", "Z", "Angle"])?;

    points.as_ref().iter().for_each(|p| {
        writer
            .write_record(&[
                String::new(),
                format!("{:x}", p.map.0),
                p.position.0.to_string(),
                p.position.1.to_string(),
                p.position.2.to_string(),
                p.orientation.to_string(),
            ])
            .unwrap();
    });

    writer.flush()?;

    Ok(())
}

fn read_loot_collection<R>(reader: R) -> Result<Vec<LootTableEntry>, Box<dyn Error>>
where
    R: Read,
{
    let mut reader = csv::Reader::from_reader(reader);
    let result = reader
        .records()
        .map(|r| {
            let r = r?;

            let weight = r[1].parse::<u32>()?;

            let mut items = vec![];
            for i in 0..10 {
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
                items,
            })
        })
        .collect::<Result<Vec<LootTableEntry>, Box<dyn Error>>>()?;

    Ok(result)
}

pub fn write_loot_collection<W, L>(writer: W, loot: L) -> Result<(), Box<dyn Error>>
where
    W: Write,
    L: AsRef<[LootTableEntry]>,
{
    let mut writer = csv::Writer::from_writer(writer);
    writer.write_record(&[
        "Description",
        "Weight",
        "Rarity",
        "Item ID 1",
        "Item quantity 1",
        "Item ID 2",
        "Item quantity 2",
        "Item ID 3",
        "Item quantity 3",
        "Item ID 4",
        "Item quantity 4",
        "Item ID 5",
        "Item quantity 5",
        "Item ID 6",
        "Item quantity 6",
        "Item ID 7",
        "Item quantity 7",
        "Item ID 8",
        "Item quantity 8",
        "Item ID 9",
        "Item quantity 9",
        "Item ID 10",
        "Item quantity 10",
    ])?;

    loot.as_ref().iter().for_each(|e| {
        let mut record = Vec::from([String::new(), e.weight.to_string(), String::from("0")]);

        for i in 0..9 {
            let item_entry = e.items.get(i).unwrap_or(&LootTableEntryItem {
                item: -1,
                quantity: 0,
            });

            record.push(format!("{:08x}", item_entry.item));
            record.push(format!("{}", item_entry.quantity));
        }

        writer
            .write_record(record.as_slice())
            .unwrap();
    });

    writer.flush()?;

    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    loot: Vec<LootTableEntry>,
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
pub struct RingConfiguration {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootTableEntry {
    pub weight: u32,
    pub items: Vec<LootTableEntryItem>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LootTableEntryItem {
    pub item: i32,
    pub quantity: u32,
}
