use std::sync::{Arc, RwLock};

use game::{
    cs::{
        ChrAsmEquipEntries, ChrAsmSlot, EquipInventoryData, EquipInventoryDataListEntry, ItemId,
        QMItemBackupVectorItem, WorldChrMan,
    },
    Vector,
};
use util::singleton::get_instance;

use crate::{
    rva::{RVA_EQUIP_INVENTORY_DATA_REMOVE_ITEM, RVA_QM_BACKUP_ITEM, RVA_UNEQUIP_ITEM},
    ProgramLocationProvider,
};

/// Levels applied to the player when in the battle.
pub const PLAYER_LEVELS_IN_BATTLE: PlayerLevels = PlayerLevels {
    level: 215,
    vigor: 60,
    mind: 15,
    endurance: 35,
    strength: 40,
    dexterity: 48,
    intelligence: 38,
    faith: 30,
    arcane: 28,
};

pub struct Player {
    location: Arc<ProgramLocationProvider>,

    /// Holds the original levels for the player.
    pub levels_snapshot: RwLock<Option<PlayerLevels>>,
    /// Holds the original equipment for the player.
    pub equipment_snapshot: RwLock<Option<ChrAsmEquipEntries>>,
}

impl Player {
    pub fn new(location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            location,
            levels_snapshot: Default::default(),
            equipment_snapshot: Default::default(),
        }
    }

    pub fn setup_for_match(&self) {
        tracing::info!("Setting up player for match.");

        self.snapshot_equipment();
        self.store_items_in_backup();
        self.clear_equipment();
        self.clean_inventory();

        self.snapshot_levels();
        self.apply_levels_to_player(&PLAYER_LEVELS_IN_BATTLE);
    }

    pub fn restore_original_levels(&self) {
        tracing::info!("Restoring levels after match");
        let original = self
            .levels_snapshot
            .write()
            .unwrap()
            .take()
            .expect("No levels to restore");
        self.apply_levels_to_player(&original);
    }

    /// Copies current player level into memory for later reapplication.
    fn snapshot_levels(&self) {
        let player_game_data = &unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .expect("Could not get WorldChrMan")
            .main_player
            .as_ref()
            .expect("Could not get main player")
            .player_game_data;

        *self.levels_snapshot.write().unwrap() = Some(PlayerLevels {
            level: player_game_data.level,
            vigor: player_game_data.vigor,
            mind: player_game_data.mind,
            endurance: player_game_data.endurance,
            strength: player_game_data.strength,
            dexterity: player_game_data.dexterity,
            intelligence: player_game_data.intelligence,
            faith: player_game_data.faith,
            arcane: player_game_data.arcane,
        });
    }

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
    pub fn clean_inventory(&self) {
        tracing::info!("Storing player items");

        let equipment = &mut unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .expect("Could not get WorldChrMan")
            .main_player
            .as_mut()
            .expect("Could not get main player")
            .player_game_data
            .equipment;

        let remove_item: fn(&EquipInventoryData, u32, u32) -> bool = unsafe {
            std::mem::transmute(
                self.location
                    .get(RVA_EQUIP_INVENTORY_DATA_REMOVE_ITEM)
                    .unwrap(),
            )
        };

        (equipment.equip_inventory_data.items_data.key_item_capacity
            ..equipment.equip_inventory_data.items_data.normal_item_count)
            .for_each(|i| {
                remove_item(&equipment.equip_inventory_data, i, 1);
            });
    }

    /// Store the players item in quick match item backup vector.
    pub fn store_items_in_backup(&self) {
        tracing::info!("Storing player items in backup");

        let equipment = &mut unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .expect("Could not get WorldChrMan")
            .main_player
            .as_mut()
            .expect("Could not get main player")
            .player_game_data
            .equipment;
        let backup_item_for_qm: fn(&Vector<QMItemBackupVectorItem>, &ItemId, u32) -> bool =
            unsafe { std::mem::transmute(self.location.get(RVA_QM_BACKUP_ITEM).unwrap()) };

        // Clean vector so no duplicates are stored
        let qmv = equipment.qm_item_backup_vector.as_mut();
        qmv.end = qmv.begin;

        (equipment.equip_inventory_data.items_data.key_item_capacity
            ..equipment.equip_inventory_data.items_data.normal_item_count)
            .for_each(|i| {
                if let Some(item_entry) = equipment
                    .equip_inventory_data
                    .items_data
                    .normal_items()
                    .get(i as usize)
                {
                    backup_item_for_qm(
                        &equipment.qm_item_backup_vector,
                        &item_entry.item_id,
                        item_entry.quantity,
                    );
                }
            });
    }

    pub fn snapshot_equipment(&self) {
        let equipment_entries = &unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .expect("Could not get WorldChrMan")
            .main_player
            .as_ref()
            .expect("Could not get main player")
            .player_game_data
            .equipment
            .equipment_entries;

        *self.equipment_snapshot.write().unwrap() = Some(equipment_entries.clone());
    }

    /// Clear out players equipment
    pub fn clear_equipment(&self) {
        tracing::info!("Clearing player equipment");

        let location_unequip_item = self.location.get(RVA_UNEQUIP_ITEM).unwrap();
        let unequip_item: extern "C" fn(ChrAsmSlot, bool) =
            unsafe { std::mem::transmute(location_unequip_item) };

        unequip_item(ChrAsmSlot::WeaponLeft1, true);
        unequip_item(ChrAsmSlot::WeaponRight1, true);

        unequip_item(ChrAsmSlot::WeaponRight1, true);
        unequip_item(ChrAsmSlot::WeaponLeft2, true);
        unequip_item(ChrAsmSlot::WeaponRight2, true);
        unequip_item(ChrAsmSlot::WeaponLeft3, true);
        unequip_item(ChrAsmSlot::WeaponRight3, true);

        unequip_item(ChrAsmSlot::Arrow1, true);
        unequip_item(ChrAsmSlot::Bolt1, true);
        unequip_item(ChrAsmSlot::Arrow2, true);
        unequip_item(ChrAsmSlot::Bolt2, true);

        unequip_item(ChrAsmSlot::ProtectorHead, true);
        unequip_item(ChrAsmSlot::ProtectorChest, true);
        unequip_item(ChrAsmSlot::ProtectorHands, true);
        unequip_item(ChrAsmSlot::ProtectorLegs, true);

        unequip_item(ChrAsmSlot::Accessory1, true);
        unequip_item(ChrAsmSlot::Accessory2, true);
        unequip_item(ChrAsmSlot::Accessory3, true);
        unequip_item(ChrAsmSlot::Accessory4, true);

        unequip_item(ChrAsmSlot::AccessoryCovenant, true);

        unequip_item(ChrAsmSlot::QuickItem1, true);
        unequip_item(ChrAsmSlot::QuickItem2, true);
        unequip_item(ChrAsmSlot::QuickItem3, true);
        unequip_item(ChrAsmSlot::QuickItem4, true);
        unequip_item(ChrAsmSlot::QuickItem5, true);
        unequip_item(ChrAsmSlot::QuickItem6, true);
        unequip_item(ChrAsmSlot::QuickItem7, true);
        unequip_item(ChrAsmSlot::QuickItem8, true);
        unequip_item(ChrAsmSlot::QuickItem9, true);
        unequip_item(ChrAsmSlot::QuickItem10, true);

        unequip_item(ChrAsmSlot::Pouch1, true);
        unequip_item(ChrAsmSlot::Pouch2, true);
        unequip_item(ChrAsmSlot::Pouch3, true);
        unequip_item(ChrAsmSlot::Pouch4, true);
        unequip_item(ChrAsmSlot::Pouch5, true);
        unequip_item(ChrAsmSlot::Pouch6, true);
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
