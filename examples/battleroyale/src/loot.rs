use game::cs::{MapId, MapItemMan};
use rand::{distributions::WeightedIndex, prelude::*};
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

use crate::{config::LootSpawnPoint, ProgramLocationProvider};
use crate::{
    config::{
        ConfigurationProvider, LootTableEntry, LootTableEntryItem, MapConfiguration, MapPosition,
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
    config: Arc<ConfigurationProvider>,

    /// Did the current map get the initial items spawned already?
    has_provisioned_map: bool,
}

impl LootGenerator {
    pub fn new(config: Arc<ConfigurationProvider>) -> Self {
        Self {
            config,
            has_provisioned_map: Default::default(),
        }
    }

    /// Generate spawn locations and corresponding loot for current map init.
    pub fn initial_map_loot(&self, map: &MapConfiguration) -> Vec<(LootSpawnPoint, LootTableEntry)> {
        // For now just populate all spawn locations.
        map.item_spawn_points
            .iter()
            .map(|point| (point.clone(), self.pick_loot(point.pool)))
            .collect::<Vec<_>>()
    }

    /// Pick a random loot table entry taking into account the weighting and item pools.
    fn pick_loot(&self, pool: u32) -> LootTableEntry {
        let loot = self.config.loot();
        let pool_loot = loot.iter().filter(|l| l.pool == pool).collect::<Vec<_>>();

        let weights = pool_loot.iter().map(|f| f.weight).collect::<Vec<u32>>();

        let distribution = WeightedIndex::new(weights.as_slice()).unwrap();
        let mut rng = thread_rng();

        pool_loot[distribution.sample(&mut rng)].clone()
    }
}
