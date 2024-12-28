use std::{cell::RefCell, sync::Arc, time::Duration};

use game::cs::{CSEventFlagMan, MapId, MapItemMan};
use util::singleton::get_instance;

use crate::{
    config::{
        ConfigurationProvider, LootSpawnPoint, LootTableEntry, MapConfiguration, MapPosition,
    },
    context::GameModeContext,
    gamestate::GameStateProvider,
    loot::LootGenerator,
    rva::RVA_SPAWN_DROPPED_ITEM,
    ProgramLocationProvider,
};

/// Handles preparing the map for the gamemode.
pub struct Stage {
    location: Arc<ProgramLocationProvider>,
    game: Arc<GameStateProvider>,
    /// Targeted stage's config.
    config: Arc<ConfigurationProvider>,
    /// Table of loot to pull from.
    loot: Arc<LootGenerator>,
    /// Have we applied the event flags for this match?
    applied_flags: bool,
    /// Have we placed initial loot on map?
    placed_initial_loot: bool,
}

impl Stage {
    pub fn new(
        location: Arc<ProgramLocationProvider>,
        game: Arc<GameStateProvider>,
        config: Arc<ConfigurationProvider>,
        loot: Arc<LootGenerator>,
    ) -> Self {
        Self {
            location,
            game,
            config,
            loot,
            applied_flags: false,
            placed_initial_loot: false,
        }
    }

    pub fn update(&mut self) {
        // Set the event flags once we're in the temporary flag state.
        if self.game.event_flags_are_non_local() && !self.applied_flags {
            self.applied_flags = true;
            self.apply_event_flags();
        }

        // Spawn initial map loot if you're the host
        if self.game.is_host() && self.game.match_in_game() && !self.placed_initial_loot {
            self.placed_initial_loot = true;
            self.place_initial_map_loot();
        }
    }

    pub fn reset(&mut self) {
        self.applied_flags = false;
        self.placed_initial_loot = false;
    }

    fn apply_event_flags(&mut self) {
        let cs_event_flag_man = unsafe { get_instance::<CSEventFlagMan>() }
            .unwrap()
            .unwrap();

        self.config
            .map(&self.game.stage())
            .unwrap()
            .event_flag_overrides
            .iter()
            .for_each(|(flag, state)| {
                cs_event_flag_man
                    .virtual_memory_flag
                    .set_flag(*flag, *state);
            });
    }

    fn place_initial_map_loot(&self) {
        let map = self.config.map(&self.game.stage()).unwrap();
        let map_loot = self.loot.initial_map_loot(&map);

        self.place_loot(map_loot);
    }

    pub fn place_loot(&self, loot: Vec<(LootSpawnPoint, LootTableEntry)>) {
        let location_dropped_item = self.location.get(RVA_SPAWN_DROPPED_ITEM).unwrap();

        std::thread::spawn(move || {
            loot.into_iter().for_each(|(point, loot)| {
                // Throttle to not exhaust fixed-bound packet queue for packet 44.
                std::thread::sleep(Duration::from_millis(50));

                tracing::info!("Spawning loot");
                let MapPosition(x, y, z) = point.position;

                let mut entries = [ItemSpawnRequestEntry::default(); 10];
                // let loot = loot_table.choose(&mut rng).expect("Loot table has no loot");
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
