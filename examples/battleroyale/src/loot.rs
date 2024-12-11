use game::cs::{MapId, MapItemMan};
use rand::prelude::*;
use std::{
    marker::Sync, sync::{
        atomic::{AtomicBool, Ordering}, Arc, RwLock
    }, time::{Duration, Instant}
};
use util::singleton::get_instance;

// Spawn some loot around the place
const LOOT_SPAWN_INTERVAL: Duration = Duration::from_secs(10);

use crate::{
    mapdata::{self, MapConfiguration},
    ProgramLocationProvider, LOCATION_SPAWN_DROPPED_ITEM,
};

pub struct LootTableEntry {
    pub weight: u32,
    pub items: Vec<LootTableEntryItem>,
}

pub struct LootTableEntryItem {
    pub item: u32,
    pub quantity: u32,
}

impl LootTableEntryItem {
    const fn new(item: u32, quantity: u32) -> Self {
        Self { item, quantity }
    }
}

/// Generates and spawns random loot over the map
pub struct LootGenerator<L>
where
    L: ProgramLocationProvider + Sync,
{
    /// Did the current map get the initial items spawned already?
    has_provisioned_map: AtomicBool,

    /// When did we last spawn items?
    last_spawn_round: RwLock<Instant>,

    location: Arc<L>,
}

impl<L> LootGenerator<L>
where
    L: ProgramLocationProvider + Sync,
{
    pub fn new(location: Arc<L>) -> Self {
        Self {
            has_provisioned_map: Default::default(),
            last_spawn_round: RwLock::new(Instant::now()),
            location,
        }
    }

    pub fn update(&self) {
        let map = mapdata::get(0).unwrap();

        // First update on the map should provision it
        if !self.has_provisioned_map.load(Ordering::Relaxed) {
            tracing::info!("Provisioning map");
            self.provision_map(&map);
            self.has_provisioned_map.store(true, Ordering::Relaxed);
        }
    }

    /// Place random items on map at start of match.
    pub fn provision_map(&self, map: &MapConfiguration) {
        let points = map.item_spawn_points.clone();
        let location_dropped_item = self.location.get(LOCATION_SPAWN_DROPPED_ITEM).unwrap();

        std::thread::spawn(move || {
            let loot_table = &[
                // 1x Pulley Bow + 15 Shattershard Arrows + Piquebone
                LootTableEntry {
                    weight: 3,
                    items: vec![
                        LootTableEntryItem::new(0x027286A0, 1),
                        LootTableEntryItem::new(0x02FBDAE0, 15),
                        LootTableEntryItem::new(0x03032DE0, 15),
                    ],
                },
                // 20x Bloodbone + Coldbone Arrows
                LootTableEntry {
                    weight: 2,
                    items: vec![
                        LootTableEntryItem::new(0x02FF8460, 20),
                        LootTableEntryItem::new(0x02FFD280, 20),
                    ],
                },
                // 1x Great Katana
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x03F6B5A0, 1)],
                },
                // 1x Stone-Sheathed Sword
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x0026C1E0, 1)],
                },
                // 1x Fire Knight's Shortsword
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x00170A70, 1)],
                },
                // 1x Lizard Greatsword
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x0035B600, 1)],
                },
                // Common Soldier Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104FF4C0, 1),
                        LootTableEntryItem::new(0x104FF524, 1),
                        LootTableEntryItem::new(0x104FF588, 1),
                        LootTableEntryItem::new(0x104FF5EC, 1),
                    ],
                },
                // Dancer's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104D83C0, 1),
                        LootTableEntryItem::new(0x104D8424, 1),
                        LootTableEntryItem::new(0x104D8488, 1),
                        LootTableEntryItem::new(0x104D84EC, 1),
                    ],
                },
                // Grave Bird Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104FCDB0, 1),
                        LootTableEntryItem::new(0x104FCE14, 1),
                        LootTableEntryItem::new(0x104FCE78, 1),
                        LootTableEntryItem::new(0x104FCEDC, 1),
                    ],
                },
                // Shadow Militiaman Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x10509100, 1),
                        LootTableEntryItem::new(0x10509164, 1),
                        LootTableEntryItem::new(0x105091C8, 1),
                        LootTableEntryItem::new(0x1050922C, 1),
                    ],
                },
                // Ansbach's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104DF8F0, 1),
                        LootTableEntryItem::new(0x104DF954, 1),
                        LootTableEntryItem::new(0x104DF9B8, 1),
                        LootTableEntryItem::new(0x104DFA1C, 1),
                    ],
                },
                // Ascetic's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104F7F90, 1),
                        LootTableEntryItem::new(0x104F7FF4, 1),
                        LootTableEntryItem::new(0x104F8058, 1),
                        LootTableEntryItem::new(0x104F80BC, 1),
                    ],
                },
                // Black Knight Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104E9530, 1),
                        LootTableEntryItem::new(0x104E9594, 1),
                        LootTableEntryItem::new(0x104E95F8, 1),
                        LootTableEntryItem::new(0x104E965C, 1),
                    ],
                },
                // Dane's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x102DC6C0, 1),
                        LootTableEntryItem::new(0x102DC724, 1),
                        LootTableEntryItem::new(0x102DC788, 1),
                        LootTableEntryItem::new(0x102DC7EC, 1),
                    ],
                },
                // Death Knight Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104F5880, 1),
                        LootTableEntryItem::new(0x104F58E4, 1),
                        LootTableEntryItem::new(0x104F5948, 1),
                        LootTableEntryItem::new(0x104F59AC, 1),
                    ],
                },
                // Divine Bird Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x10502788, 1),
                        LootTableEntryItem::new(0x105027EC, 1),
                        LootTableEntryItem::new(0x10502850, 1),
                        LootTableEntryItem::new(0x105028B4, 1),
                    ],
                },
                // Fire Knight Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104F0A60, 1),
                        LootTableEntryItem::new(0x104F0AC4, 1),
                        LootTableEntryItem::new(0x104F0B28, 1),
                        LootTableEntryItem::new(0x104F0B8C, 1),
                    ],
                },
                // Freyja's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104E2000, 1),
                        LootTableEntryItem::new(0x104E2064, 1),
                        LootTableEntryItem::new(0x104E20C8, 1),
                        LootTableEntryItem::new(0x104E212C, 1),
                    ],
                },
                // Gaius's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x102DEDD0, 1),
                        LootTableEntryItem::new(0x102DEE34, 1),
                        LootTableEntryItem::new(0x102DEE98, 1),
                        LootTableEntryItem::new(0x102DEEFC, 1),
                    ],
                },
                // High Priest Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104D35A0, 1),
                        LootTableEntryItem::new(0x104D3604, 1),
                        LootTableEntryItem::new(0x104D3668, 1),
                        LootTableEntryItem::new(0x104D36CC, 1),
                    ],
                },
                // Horned Warrior Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x10501BD0, 1),
                        LootTableEntryItem::new(0x10501C34, 1),
                        LootTableEntryItem::new(0x10501C98, 1),
                        LootTableEntryItem::new(0x10501CFC, 1),
                    ],
                },
                // Hornsent Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104D5CB0, 1),
                        LootTableEntryItem::new(0x104D5D14, 1),
                        LootTableEntryItem::new(0x104D5D78, 1),
                        LootTableEntryItem::new(0x104D5DDC, 1),
                    ],
                },
                // Igon's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104DD1E0, 1),
                        LootTableEntryItem::new(0x104DD244, 1),
                        LootTableEntryItem::new(0x104DD2A8, 1),
                        LootTableEntryItem::new(0x104DD30C, 1),
                    ],
                },
                // Iron Rivet Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104C9D48, 1),
                        LootTableEntryItem::new(0x104C99C4, 1),
                        LootTableEntryItem::new(0x104C9A28, 1),
                        LootTableEntryItem::new(0x104C9A8C, 1),
                    ],
                },
                // Highland Warrior Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104F3558, 1),
                        LootTableEntryItem::new(0x104F31D4, 1),
                        LootTableEntryItem::new(0x104F3238, 1),
                        LootTableEntryItem::new(0x104F329C, 1),
                    ],
                },
                // Messmer Soldier Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104E6E20, 1),
                        LootTableEntryItem::new(0x104E6E84, 1),
                        LootTableEntryItem::new(0x104E6EE8, 1),
                        LootTableEntryItem::new(0x104E6F4C, 1),
                    ],
                },
                // Messmer's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104FA6A0, 1),
                        LootTableEntryItem::new(0x104FA704, 1),
                        LootTableEntryItem::new(0x104FA768, 1),
                        LootTableEntryItem::new(0x104FA7CC, 1),
                    ],
                },
                // Night Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104DAAD0, 1),
                        LootTableEntryItem::new(0x104DAB34, 1),
                        LootTableEntryItem::new(0x104DAB98, 1),
                        LootTableEntryItem::new(0x104DABFC, 1),
                    ],
                },
                // Oathseeker Knight Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104C4B40, 1),
                        LootTableEntryItem::new(0x104C5374, 1),
                        LootTableEntryItem::new(0x104C4C08, 1),
                        LootTableEntryItem::new(0x104C4C6C, 1),
                    ],
                },
                // Rakshasa Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104EBC40, 1),
                        LootTableEntryItem::new(0x104EBCA4, 1),
                        LootTableEntryItem::new(0x104EBD08, 1),
                        LootTableEntryItem::new(0x104EBD6C, 1),
                    ],
                },
                // Rellana's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x105042E0, 1),
                        LootTableEntryItem::new(0x10504344, 1),
                        LootTableEntryItem::new(0x105043A8, 1),
                        LootTableEntryItem::new(0x1050440C, 1),
                    ],
                },
                // Solitude Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104E4710, 1),
                        LootTableEntryItem::new(0x104E4774, 1),
                        LootTableEntryItem::new(0x104E47D8, 1),
                        LootTableEntryItem::new(0x104E483C, 1),
                    ],
                },
                // Thiollier's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104CC070, 1),
                        LootTableEntryItem::new(0x104CC0D4, 1),
                        LootTableEntryItem::new(0x104CC138, 1),
                        LootTableEntryItem::new(0x104CC19C, 1),
                    ],
                },
                // Verdigris Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x104C7250, 1),
                        LootTableEntryItem::new(0x104C72B4, 1),
                        LootTableEntryItem::new(0x104C7318, 1),
                        LootTableEntryItem::new(0x104C737C, 1),
                    ],
                },
                // Young Lion's Set
                LootTableEntry {
                    weight: 1,
                    items: vec![
                        LootTableEntryItem::new(0x105069F0, 1),
                        LootTableEntryItem::new(0x10506AB8, 1),
                        LootTableEntryItem::new(0x10506B1C, 1),
                        LootTableEntryItem::new(0x10506B80, 1),
                    ],
                },
                // Dueling Shield
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x03B9ACA0, 1)],
                },
                // Dryleaf Arts
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x039B2820, 1)],
                },
                // Smithscript Dagger
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x03C8EEE0, 1)],
                },
                // Backhand Blade
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x03D83120, 1)],
                },
                // Beast Claw
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x04153A20, 1)],
                },
                // Milady
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x0405F7E0, 1)],
                },
                // Fire Knight's Greatsword
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x0044F840, 1)],
                },
                // Frozen Needle
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x004D0E90, 1)],
                },
                // Sword Lance
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x003567E0, 1)],
                },
                // Horned Warrior's Sword
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x0072E610, 1)],
                },
                // Horned Warrior's Greatsword
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x00820140, 1)],
                },
                // Sword of Night (But no flame)
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x0090F560, 1)],
                },
                // Black Steel Twinblade
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x00A05EB0, 1)],
                },
                // Messmer Soldier's Axe
                LootTableEntry {
                    weight: 3,
                    items: vec![LootTableEntryItem::new(0x00DD8EC0, 1)],
                },
            ];

            std::thread::sleep(Duration::from_secs(3));

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
                let (x, y, z) = point.position.xyz();

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
                    map: point.map,
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
    pub fn reset(&self) {
        tracing::info!("Resetting loot generator");
        self.has_provisioned_map.store(false, Ordering::Relaxed);
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
    pub item: u32,
    pub quantity: u32,
    pub unk8: u32,
    pub unkc: i32,
}
