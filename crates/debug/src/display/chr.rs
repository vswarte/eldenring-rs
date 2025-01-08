use game::cs::{
    CSChrModelParamModifierModule, ChrAsm, ChrAsmEquipEntries, ChrAsmEquipment, ChrIns,
    ChrInsModuleContainer, ChrPhysicsModule, EquipGameData, EquipInventoryData, EquipItemData,
    EquipMagicData, PlayerGameData, PlayerIns,
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for PlayerIns {
    fn render_debug(&self, ui: &&mut Ui) {
        self.chr_ins.render_debug(ui);

        if ui.collapsing_header("ChrAsm", TreeNodeFlags::empty()) {
            ui.indent();
            self.chr_asm.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("PlayerGameData", TreeNodeFlags::empty()) {
            ui.indent();
            self.player_game_data.render_debug(ui);
            ui.unindent();
        }

        ui.text(format!(
            "Steam ID: {:x}",
            unsafe { self.session_manager_player_entry.as_ref() }.steam_id
        ));
        if ui.collapsing_header("Session Player Entry", TreeNodeFlags::empty()) {
            ui.indent();
            ui.unindent();
        }

        ui.text(format!("Locked on enemy: {}", self.locked_on_enemy));
        ui.text(format!("Block position: {}", self.block_position));
        ui.text(format!("Block orientation: {}", self.block_orientation));
    }
}

impl DebugDisplay for ChrAsm {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("ChrAsmEquipment", TreeNodeFlags::empty()) {
            ui.indent();
            self.equipment.render_debug(ui);
            ui.unindent();
        }

        for (i, e) in self.gaitem_handles.iter().enumerate() {
            ui.text(format!("Gaitem handle {}: {}", i, e.to_string()));
        }

        for (i, e) in self.equipment_param_ids.iter().enumerate() {
            ui.text(format!("Equipment param ID {}: {:?}", i, e));
        }
    }
}

impl DebugDisplay for ChrAsmEquipment {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Arm style: {:?}", self.arm_style));
        ui.text(format!(
            "Left-hand weapon slot: {:?}",
            self.selected_slots.left_weapon_slot
        ));
        ui.text(format!(
            "Right-hand weapon slot: {:?}",
            self.selected_slots.right_weapon_slot
        ));
        ui.text(format!(
            "Left-hand arrow slot: {:?}",
            self.selected_slots.left_arrow_slot
        ));
        ui.text(format!(
            "Right-hand arrow slot: {:?}",
            self.selected_slots.right_weapon_slot
        ));
        ui.text(format!(
            "Left-hand bolt slot: {:?}",
            self.selected_slots.left_bolt_slot
        ));
        ui.text(format!(
            "Right-hand bolt slot: {:?}",
            self.selected_slots.right_bolt_slot
        ));
    }
}

impl DebugDisplay for ChrAsmEquipEntries {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "Primary Left weapon: {:?}",
            self.weapon_primary_left
        ));
        ui.text(format!(
            "Primary Right weapon: {:?}",
            self.weapon_primary_right
        ));
        ui.text(format!(
            "Secondary Left weapon: {:?}",
            self.weapon_secondary_left
        ));
        ui.text(format!(
            "Secondary Right weapon: {:?}",
            self.weapon_secondary_right
        ));
        ui.text(format!(
            "Tertiary Left weapon: {:?}",
            self.weapon_tertiary_left
        ));
        ui.text(format!(
            "Tertiary Right weapon: {:?}",
            self.weapon_tertiary_right
        ));

        ui.text(format!("Primary Left arrow: {:?}", self.arrow_primary));
        ui.text(format!("Primary Left bolt: {:?}", self.bolt_primary));
        ui.text(format!("Secondary Left arrow: {:?}", self.arrow_secondary));
        ui.text(format!("Secondary Left bolt: {:?}", self.bolt_secondary));
        ui.text(format!("Tertiary Left arrow: {:?}", self.arrow_tertiary));
        ui.text(format!("Tertiary Left bolt: {:?}", self.bolt_tertiary));

        ui.text(format!("Protector Head: {:?}", self.protector_head));
        ui.text(format!("Protector Chest: {:?}", self.protector_chest));
        ui.text(format!("Protector Hands: {:?}", self.protector_hands));
        ui.text(format!("Protector Legs: {:?}", self.protector_legs));

        ui.text(format!("Hair???: {:?}", self.hair));

        if ui.collapsing_header("Accessories", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "chr-asm-equip-entries-accessories",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Item ID"),
                ],
            ) {
                self.accessories
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());
                        ui.table_next_column();
                        ui.text(item.to_string());
                    });
            }
        }

        ui.text(format!("Covenant: {:?}", self.covenant));

        if ui.collapsing_header("Quick Items", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "chr-asm-equip-entries-quick-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Item ID"),
                ],
            ) {
                self.quick_tems
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());
                        ui.table_next_column();
                        ui.text(item.to_string());
                    });
            }
        }

        if ui.collapsing_header("Pouch", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "chr-asm-equip-entries-pouch",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Item ID"),
                ],
            ) {
                self.pouch.iter().enumerate().for_each(|(index, item)| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.to_string());
                });
            }
        }
    }
}

impl DebugDisplay for PlayerGameData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "Furlcalling Finger Active: {:?}",
            self.furlcalling_finger_remedy_active
        ));
        ui.text(format!("Rune Arc Active: {:?}", self.rune_arc_active));
        ui.text(format!("White Ring Active: {:?}", self.white_ring_active));
        ui.text(format!("Blue Ring Active: {:?}", self.blue_ring_active));

        ui.text(format!("Character Type: {:?}", self.character_type));
        ui.text(format!("Team Type: {:?}", self.team_type));

        if ui.collapsing_header("EquipGameData", TreeNodeFlags::empty()) {
            self.equipment.render_debug(ui);
        }

        if ui.collapsing_header("Storage Box EquipInventoryData", TreeNodeFlags::empty()) {
            self.storage.render_debug(ui);
        }
    }
}

impl DebugDisplay for EquipGameData {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("EquipInventoryData", TreeNodeFlags::empty()) {
            self.equip_inventory_data.render_debug(ui);
        }
        if ui.collapsing_header("EquipMagicData", TreeNodeFlags::empty()) {
            self.equip_magic_data.render_debug(ui);
        }
        if ui.collapsing_header("EquipItemData", TreeNodeFlags::empty()) {
            self.equip_item_data.render_debug(ui);
        }

        if ui.collapsing_header("QuickMatch Item Backup Vector", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-game-data-qm-item-backup-vector",
                [
                    TableColumnSetup::new("ItemId"),
                    TableColumnSetup::new("Count"),
                ],
            ) {
                self.qm_item_backup_vector
                    .items()
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());

                        ui.table_next_column();
                        ui.text(item.item_id.to_string());
                    });
            }
        }

        if ui.collapsing_header("Equipment Entries", TreeNodeFlags::empty()) {
            self.equipment_entries.render_debug(ui);
        }
    }
}

impl DebugDisplay for EquipMagicData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Selected slot: {}", self.selected_slot));

        if ui.collapsing_header("EquipDataItem", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-magic-data-entries",
                [
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Index"),
                ],
            ) {
                self.entries.iter().enumerate().for_each(|(index, item)| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.gaitem_handle.to_string());
                });
            }
        }
    }
}

impl DebugDisplay for EquipItemData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Selected quick slot: {}", self.selected_quick_slot));

        if ui.collapsing_header("Quick slots", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-item-data-quick-slots",
                [
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Index"),
                ],
            ) {
                self.quick_slots
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());

                        ui.table_next_column();
                        ui.text(item.gaitem_handle.to_string());
                    });
            }
        }

        if ui.collapsing_header("Pouch slots", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-item-data-pouch-slots",
                [
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Index"),
                ],
            ) {
                self.pouch_slots
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());

                        ui.table_next_column();
                        ui.text(item.gaitem_handle.to_string());
                    });
            }
        }

        ui.text(format!(
            "Greatrune: {}, index: {}",
            self.great_rune.gaitem_handle, self.great_rune.index
        ));

        if ui.collapsing_header("Equipment Entries", TreeNodeFlags::empty()) {
            self.equip_entries.render_debug(ui);
        }

        ui.text(format!("Selected Quick Slot: {}", self.selected_quick_slot));
    }
}

impl DebugDisplay for EquipInventoryData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "Total item entry count: {}",
            self.total_item_entry_count
        ));

        let label = format!(
            "Normal Items ({}/{})",
            self.items_data.normal_item_count, self.items_data.normal_item_capacity
        );
        if ui.collapsing_header(label.as_str(), TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-inventory-data-normal-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                ],
            ) {
                self.items_data
                    .normal_items()
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());

                        ui.table_next_column();
                        ui.text(item.gaitem_handle.to_string());

                        ui.table_next_column();
                        ui.text(item.item_id.to_string());

                        ui.table_next_column();
                        ui.text(item.quantity.to_string());

                        ui.table_next_column();
                        ui.text(item.display_id.to_string());
                    });
            }
        }

        let label = format!(
            "Key Items ({}/{})",
            self.items_data.key_item_count, self.items_data.key_item_capacity
        );
        if ui.collapsing_header(label.as_str(), TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-inventory-data-key-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                ],
            ) {
                self.items_data
                    .key_items()
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());

                        ui.table_next_column();
                        ui.text(item.gaitem_handle.to_string());

                        ui.table_next_column();
                        ui.text(item.item_id.to_string());

                        ui.table_next_column();
                        ui.text(item.quantity.to_string());

                        ui.table_next_column();
                        ui.text(item.display_id.to_string());
                    });
            }
        }

        let label = format!(
            "Secondary Key Items ({}/{})",
            self.items_data.secondary_key_item_count, self.items_data.secondary_key_item_capacity
        );
        if ui.collapsing_header(label.as_str(), TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-inventory-data-secondary-key-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                ],
            ) {
                self.items_data
                    .secondary_key_items()
                    .iter()
                    .enumerate()
                    .for_each(|(index, item)| {
                        ui.table_next_column();
                        ui.text(index.to_string());

                        ui.table_next_column();
                        ui.text(item.gaitem_handle.to_string());

                        ui.table_next_column();
                        ui.text(item.item_id.to_string());

                        ui.table_next_column();
                        ui.text(item.quantity.to_string());

                        ui.table_next_column();
                        ui.text(item.display_id.to_string());
                    });
            }
        }
    }
}

impl DebugDisplay for ChrIns {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Team: {}", self.team_type));

        ui.text(format!("Map ID: {}", self.map_id_1));
        // ui.text(format!("Team Type: {}", self.team_type));
        ui.text(format!("Last killed by: {}", self.last_killed_by));
        ui.text(format!("Last used item: {}", self.last_used_item));

        ui.text(format!(
            "Block center origin 1: {}",
            self.block_origin_override
        ));
        ui.text(format!("Block center origin 2: {}", self.block_origin));

        if ui.collapsing_header("Special Effect", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "chr-ins-special-effects",
                [
                    TableColumnSetup::new("ID"),
                    TableColumnSetup::new("Timer"),
                    TableColumnSetup::new("Duration"),
                    TableColumnSetup::new("Duration2"),
                    TableColumnSetup::new("Interval Timer"),
                ],
            ) {
                self.special_effect.entries().for_each(|entry| {
                    ui.table_next_column();
                    ui.text(format!("{}", entry.param_id));

                    ui.table_next_column();
                    ui.text(format!("{}", entry.interval_timer));

                    ui.table_next_column();
                    ui.text(format!("{}", entry.duration));

                    ui.table_next_column();
                    ui.text(format!("{}", entry.duration2));

                    ui.table_next_column();
                    ui.text(format!("{}", entry.interval_timer));
                });
            }
        }

        if ui.collapsing_header("Modules", TreeNodeFlags::empty()) {
            self.module_container.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrInsModuleContainer {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Physics", TreeNodeFlags::empty()) {
            ui.indent();
            self.physics.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Model param modifier", TreeNodeFlags::empty()) {
            ui.indent();
            self.model_param_modifier.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for ChrPhysicsModule {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Position: {}", self.position));
        ui.text(format!("Orientation: {}", self.orientation));
    }
}

impl DebugDisplay for CSChrModelParamModifierModule {
    fn render_debug(&self, ui: &&mut Ui) {
        if let Some(_t) = ui.begin_table_header(
            "chr-ins-model-param-modifier",
            [TableColumnSetup::new("Name")],
        ) {
            self.modifiers.items().iter().for_each(|modifier| {
                ui.table_next_column();
                ui.text(unsafe { modifier.name.to_string() }.unwrap());
            });
        }
    }
}
