use std::{
    marker::Sync,
    sync::{Arc, RwLock},
};

use game::cs::{
    EquipInventoryData, WorldChrMan, CHR_ASM_SLOT_PROTECTOR_CHEST, CHR_ASM_SLOT_PROTECTOR_HANDS,
    CHR_ASM_SLOT_PROTECTOR_HEAD, CHR_ASM_SLOT_PROTECTOR_LEGS, CHR_ASM_SLOT_WEAPON_LEFT_1,
    CHR_ASM_SLOT_WEAPON_RIGHT_1,
};
use util::singleton::get_instance;

use crate::{ProgramLocationProvider, LOCATION_TRANSFER_ITEM};

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
        self.snapshot_levels();
        self.apply_levels_to_player(&PLAYER_LEVELS_IN_BATTLE);
    }

    pub fn restore_original_levels(&self) {
        tracing::info!("Restoring levels after match");
        let original = self
            .snapshot
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

        *self.snapshot.write().unwrap() = Some(PlayerLevels {
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
            unsafe { std::mem::transmute(self.location.get(LOCATION_TRANSFER_ITEM).unwrap()) };

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
                    false,
                );
            });
    }

    /// Clear out players equipment
    pub fn clear_equipment(&self) {
        tracing::info!("Clearing player equipment");

        let equipment = &mut unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .expect("Could not get WorldChrMan")
            .main_player
            .as_mut()
            .expect("Could not get main player")
            .player_game_data
            .equipment;

        equipment.chr_asm.arm_style = 0;
        equipment.chr_asm.left_weapon_slot = 0;
        equipment.chr_asm.right_weapon_slot = 0;
        equipment.chr_asm.left_arrow_slot = 0;
        equipment.chr_asm.right_arrow_slot = 0;
        equipment.chr_asm.left_bolt_slot = 0;
        equipment.chr_asm.right_bolt_slot = 0;
        equipment.chr_asm.gaitem_handles = [0; 22];
        equipment.chr_asm.equipment_param_ids = [0; 22];

        equipment.chr_asm.gaitem_handles[CHR_ASM_SLOT_WEAPON_LEFT_1] = 0x808004b3;
        equipment.chr_asm.gaitem_handles[CHR_ASM_SLOT_WEAPON_RIGHT_1] = 0x808004b3;
        equipment.chr_asm.gaitem_handles[CHR_ASM_SLOT_PROTECTOR_HEAD] = 0x908004af;
        equipment.chr_asm.gaitem_handles[CHR_ASM_SLOT_PROTECTOR_CHEST] = 0x908004b0;
        equipment.chr_asm.gaitem_handles[CHR_ASM_SLOT_PROTECTOR_HANDS] = 0x908004b1;
        equipment.chr_asm.gaitem_handles[CHR_ASM_SLOT_PROTECTOR_LEGS] = 0x908004b2;

        equipment.chr_asm.equipment_param_ids[CHR_ASM_SLOT_WEAPON_LEFT_1] = 110000;
        equipment.chr_asm.equipment_param_ids[CHR_ASM_SLOT_WEAPON_RIGHT_1] = 110000;
        equipment.chr_asm.equipment_param_ids[CHR_ASM_SLOT_PROTECTOR_HEAD] = 10000;
        equipment.chr_asm.equipment_param_ids[CHR_ASM_SLOT_PROTECTOR_CHEST] = 10100;
        equipment.chr_asm.equipment_param_ids[CHR_ASM_SLOT_PROTECTOR_HANDS] = 10200;
        equipment.chr_asm.equipment_param_ids[CHR_ASM_SLOT_PROTECTOR_LEGS] = 10300;

        equipment.accessories = [-1; 4];
        equipment.quick_tems = [-1; 10];
        equipment.pouch = [-1; 6];
        equipment.protector_head = 10000;
        equipment.protector_chest = 10100;
        equipment.protector_hands = 10200;
        equipment.protector_legs = 10300;

        equipment.weapon_primary_left = 110000;
        equipment.weapon_primary_right = 110000;
        equipment.weapon_secondary_left = 110000;
        equipment.weapon_secondary_right = 110000;
        equipment.weapon_tertiary_left = 110000;
        equipment.weapon_tertiary_right = 110000;

        equipment.arrow_primary = -1;
        equipment.bolt_primary = -1;
        equipment.arrow_secondary = -1;
        equipment.bolt_secondary = -1;
        equipment.arrow_tertiary = -1;
        equipment.bolt_tertiary = -1;

        equipment.covenant = -1;
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
