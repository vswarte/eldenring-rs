use std::sync::{Arc, RwLock};

use game::cs::{ChrAsmSlot, EquipInventoryData, WorldChrMan};
use util::singleton::get_instance;

use crate::{
    rva::{RVA_TRANSFER_ITEM, RVA_UNEQUIP_ITEM},
    ProgramLocationProvider,
};

/// Levels applied to the player when in the battle.
pub const PLAYER_LEVELS_IN_BATTLE: PlayerLevels = PlayerLevels {
    level: 401,
    vigor: 60,
    mind: 60,
    endurance: 60,
    strength: 60,
    dexterity: 60,
    intelligence: 60,
    faith: 60,
    arcane: 60,
};

pub struct Player {
    location: Arc<ProgramLocationProvider>,

    /// Holds the original levels for the player.
    pub snapshot: RwLock<Option<PlayerLevels>>,
}

impl Player {
    pub fn new(location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            location,
            snapshot: Default::default(),
        }
    }

    pub fn setup_for_match(&self) {
        tracing::info!("Setting up player for match.");

        self.clear_equipment();
        self.store_items();
        // self.snapshot_levels();
        self.apply_levels_to_player(&PLAYER_LEVELS_IN_BATTLE);
    }

    // pub fn restore_original_levels(&self) {
    //     tracing::info!("Restoring levels after match");
    //     let original = self
    //         .snapshot
    //         .write()
    //         .unwrap()
    //         .take()
    //         .expect("No levels to restore");
    //     self.apply_levels_to_player(&original);
    // }
    //
    // /// Copies current player level into memory for later reapplication.
    // fn snapshot_levels(&self) {
    //     let player_game_data = &unsafe { get_instance::<WorldChrMan>() }
    //         .unwrap()
    //         .expect("Could not get WorldChrMan")
    //         .main_player
    //         .as_ref()
    //         .expect("Could not get main player")
    //         .player_game_data;
    //
    //     *self.snapshot.write().unwrap() = Some(PlayerLevels {
    //         level: player_game_data.level,
    //         vigor: player_game_data.vigor,
    //         mind: player_game_data.mind,
    //         endurance: player_game_data.endurance,
    //         strength: player_game_data.strength,
    //         dexterity: player_game_data.dexterity,
    //         intelligence: player_game_data.intelligence,
    //         faith: player_game_data.faith,
    //         arcane: player_game_data.arcane,
    //     });
    // }

    /// Applies a set of levels to the player
    fn apply_levels_to_player(&self, levels: &PlayerLevels) {
        let player_game_data = &mut unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .expect("Could not get WorldChrMan")
            .main_player
            .as_mut()
            .expect("Could not get main player")
            .player_game_data;

        player_game_data.level = levels.level;
        player_game_data.vigor = levels.vigor;
        player_game_data.mind = levels.mind;
        player_game_data.endurance = levels.endurance;
        player_game_data.strength = levels.strength;
        player_game_data.dexterity = levels.dexterity;
        player_game_data.intelligence = levels.intelligence;
        player_game_data.faith = levels.faith;
        player_game_data.arcane = levels.arcane;
    }

    /// Store the players item in the storage box.
    pub fn store_items(&self) {
        tracing::info!("Storing player items");

        let player_game_data = &unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .expect("Could not get WorldChrMan")
            .main_player
            .as_ref()
            .expect("Could not get main player")
            .player_game_data;

        let transfer_item: fn(u32, &EquipInventoryData, &EquipInventoryData, u32, bool) -> bool =
            unsafe { std::mem::transmute(self.location.get(RVA_TRANSFER_ITEM).unwrap()) };

        (0..player_game_data
            .equipment
            .equip_inventory_data
            .total_item_entry_count)
            .for_each(|i| {
                transfer_item(
                    i,
                    &player_game_data.equipment.equip_inventory_data,
                    &player_game_data.storage,
                    99,
                    true,
                );
            });
    }

    /// Clear out players equipment
    pub fn clear_equipment(&self) {
        tracing::info!("Clearing player equipment");

        let location_unequip_item = self.location.get(RVA_UNEQUIP_ITEM).unwrap();
        let unequip_item: extern "C" fn(ChrAsmSlot, bool) =
            unsafe { std::mem::transmute(location_unequip_item) };

        unequip_item(ChrAsmSlot::WeaponLeft1, false);
        unequip_item(ChrAsmSlot::WeaponRight1, false);

        unequip_item(ChrAsmSlot::WeaponRight1, false);
        unequip_item(ChrAsmSlot::WeaponLeft2, false);
        unequip_item(ChrAsmSlot::WeaponRight2, false);
        unequip_item(ChrAsmSlot::WeaponLeft3, false);
        unequip_item(ChrAsmSlot::WeaponRight3, false);

        unequip_item(ChrAsmSlot::Arrow1, false);
        unequip_item(ChrAsmSlot::Bolt1, false);
        unequip_item(ChrAsmSlot::Arrow2, false);
        unequip_item(ChrAsmSlot::Bolt2, false);

        unequip_item(ChrAsmSlot::ProtectorHead, false);
        unequip_item(ChrAsmSlot::ProtectorChest, false);
        unequip_item(ChrAsmSlot::ProtectorHands, false);
        unequip_item(ChrAsmSlot::ProtectorLegs, false);

        unequip_item(ChrAsmSlot::Accessory1, false);
        unequip_item(ChrAsmSlot::Accessory2, false);
        unequip_item(ChrAsmSlot::Accessory3, false);
        unequip_item(ChrAsmSlot::Accessory4, false);

        unequip_item(ChrAsmSlot::AccessoryCovenant, false);

        unequip_item(ChrAsmSlot::QuickSlot1, false);
        unequip_item(ChrAsmSlot::QuickSlot2, false);
        unequip_item(ChrAsmSlot::QuickSlot3, false);
        unequip_item(ChrAsmSlot::QuickSlot4, false);
        unequip_item(ChrAsmSlot::QuickSlot5, false);
        unequip_item(ChrAsmSlot::QuickSlot6, false);
        unequip_item(ChrAsmSlot::QuickSlot7, false);
        unequip_item(ChrAsmSlot::QuickSlot8, false);
        unequip_item(ChrAsmSlot::QuickSlot9, false);
        unequip_item(ChrAsmSlot::QuickSlot10, false);

        unequip_item(ChrAsmSlot::Pouch1, false);
        unequip_item(ChrAsmSlot::Pouch2, false);
        unequip_item(ChrAsmSlot::Pouch3, false);
        unequip_item(ChrAsmSlot::Pouch4, false);
        unequip_item(ChrAsmSlot::Pouch5, false);
        unequip_item(ChrAsmSlot::Pouch6, false);
    }
}

pub struct PlayerLevels {
    pub level: u32,
    pub vigor: u32,
    pub mind: u32,
    pub endurance: u32,
    pub strength: u32,
    pub dexterity: u32,
    pub intelligence: u32,
    pub faith: u32,
    pub arcane: u32,
}
