use game::cs::{MapId, MapItemMan};
use rand::prelude::*;
use std::{
    marker::Sync,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};
use util::singleton::get_instance;

// Spawn some loot around the place
const LOOT_SPAWN_INTERVAL: Duration = Duration::from_secs(10);

use crate::ProgramLocationProvider;
use crate::{
    config::{
        write_loot_collection, ConfigurationProvider, LootTableEntry, LootTableEntryItem,
        MapConfiguration, MapPosition,
    },
    rva::RVA_SPAWN_DROPPED_ITEM,
};

impl LootTableEntryItem {
    const fn new(item: i32, quantity: u32) -> Self {
        Self { item, quantity }
    }
}

/// Generates and spawns random loot over the map
pub struct LootGenerator {
    location: Arc<ProgramLocationProvider>,
    config: Arc<ConfigurationProvider>,

    /// Did the current map get the initial items spawned already?
    has_provisioned_map: bool,
}

impl LootGenerator {
    pub fn new(location: Arc<ProgramLocationProvider>, config: Arc<ConfigurationProvider>) -> Self {
        Self {
            location,
            config,
            has_provisioned_map: Default::default(),
        }
    }

    pub fn update(&mut self) {
        let map = self.config.map(&0).unwrap();

        // First update on the map should provision it
        if !self.has_provisioned_map {
            self.has_provisioned_map = true;
            tracing::info!("Provisioning map");
            self.provision_map(&map);
        }
    }

    /// Place random items on map at start of match.
    pub fn provision_map(&self, map: &MapConfiguration) {
        let loot_table = self.config.loot();
        let points = map.item_spawn_points.clone();
        let location_dropped_item = self.location.get(RVA_SPAWN_DROPPED_ITEM).unwrap();

        std::thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let spawn_points = {
                let mut points = points.clone();
                points.shuffle(&mut rng);
                points.into_iter()
            };

            spawn_points.for_each(|point| {
                // Throttle to not exhaust fixed-bound packet queue for packet 44.
                std::thread::sleep(Duration::from_millis(50));

                tracing::info!("Spawning loot");
                let MapPosition(x, y, z) = point.position;

                let mut entries = [ItemSpawnRequestEntry::default(); 10];
                let loot = loot_table.choose(&mut rng).expect("Loot table has no loot");
                loot.items.iter().enumerate().for_each(|(i, e)| {
                    entries[i] = ItemSpawnRequestEntry {
                        item: e.item,
                        quantity: e.quantity,
                        unk8: 0,
                        unkc: -1,
                    }
                });

                let request = Box::leak(Box::new(ItemSpawnRequest {
                    unk0: 1,
                    unk4: -1,
                    unk8: 0,
                    unkc: 0x0,
                    unk10: -1,
                    unk14: -1,
                    map: MapId(point.map.0),
                    position_x: x,
                    position_y: y,
                    position_z: z,
                    orientation: point.orientation,
                    unk2c: 0,
                    unk30: 0x000001D0,
                    unk34: -1,
                    unk38: -1,
                    unk3c: 0x0000005A,
                    entry_count: loot.items.len() as u32,
                    entries,
                    unke4: -1,
                    unke8: 1,
                    unkec: 0,
                    unkf0: -1,
                    unkf4: 0xFFFFFF00,
                    unkf8: 0,
                    unkfc: 0,
                }));

                let map_item_man = unsafe { get_instance::<MapItemMan>() }.unwrap().unwrap();
                let spawn_item: extern "C" fn(&mut MapItemMan, &ItemSpawnRequest, bool, bool) =
                    unsafe { std::mem::transmute(location_dropped_item) };

                (spawn_item)(map_item_man, request, true, false);
            });
        });
    }

    /// Reset structure after match has finished
    pub fn reset(&mut self) {
        tracing::info!("Resetting loot generator");
        self.has_provisioned_map = false;
    }
}

#[repr(C)]
pub struct ItemSpawnRequest {
    unk0: u32,
    unk4: i32,
    unk8: u32,
    unkc: u32,
    unk10: i32,
    unk14: i32,
    /// Spawn map ID
    map: MapId,
    /// Spawn x
    position_x: f32,
    /// Spawn y
    position_y: f32,
    /// Spawn z
    position_z: f32,
    /// Spawn angle
    orientation: f32,
    unk2c: u32,
    unk30: u32,
    unk34: i32,
    unk38: i32,
    unk3c: u32,
    /// Amount of entries included in this request
    entry_count: u32,
    entries: [ItemSpawnRequestEntry; 10],
    unke4: i32,
    unke8: u32,
    unkec: u32,
    unkf0: i32,
    unkf4: u32,
    unkf8: u32,
    unkfc: u32,
}

#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct ItemSpawnRequestEntry {
    pub item: i32,
    pub quantity: u32,
    pub unk8: u32,
    pub unkc: i32,
}
