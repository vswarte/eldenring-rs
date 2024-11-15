use game::cs::{
    CSChrModelParamModifierModule, ChrAsm, ChrIns, ChrInsModuleContainer, ChrPhysicsModule, EquipInventoryData, PlayerGameData, PlayerIns
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for PlayerIns {
    fn render_debug(&self, ui: &&mut Ui) {
        self.chr_ins.render_debug(ui);

        if ui.collapsing_header("ChrAsm", TreeNodeFlags::empty()) {
            self.chr_asm.render_debug(ui);
        }

        if ui.collapsing_header("PlayerGameData", TreeNodeFlags::empty()) {
            self.player_game_data.render_debug(ui);
        }

        ui.text(format!("Locked on enemy: {}", self.locked_on_enemy));
        ui.text(format!("Chunk position: {}", self.chunk_position));
    }
}

impl DebugDisplay for ChrAsm {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Arm style: {:?}", self.arm_style));
        ui.text(format!(
            "Left-hand weapon slot: {:?}",
            self.left_weapon_slot
        ));
        ui.text(format!(
            "Right-hand weapon slot: {:?}",
            self.right_weapon_slot
        ));
        ui.text(format!("Left-hand arrow slot: {:?}", self.left_arrow_slot));
        ui.text(format!(
            "Right-hand arrow slot: {:?}",
            self.right_weapon_slot
        ));
        ui.text(format!("Left-hand bolt slot: {:?}", self.left_bolt_slot));
        ui.text(format!("Right-hand bolt slot: {:?}", self.right_bolt_slot));

        for (i, e) in self.gaitem_handles.iter().enumerate() {
            ui.text(format!("Gaitem {}: {:x?}", i, e));
        }

        for (i, e) in self.equipment_param_ids.iter().enumerate() {
            ui.text(format!("Equipment param ID {}: {:?}", i, e));
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

        if ui.collapsing_header("Storage Box EquipInventoryData", TreeNodeFlags::empty()) {
            self.storage.render_debug(ui);
        }
    }
}

impl DebugDisplay for EquipInventoryData {
    fn render_debug(&self, ui: &&mut Ui) {
        let label = format!("Normal Items ({}/{})", self.normal_item_count, self.normal_item_capacity);
        if ui.collapsing_header(label.as_str(), TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-inventory-data-normal-items",
                [
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Category"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                ],
            ) {
                self.normal_items().iter().for_each(|item| {
                    ui.table_next_column();
                    ui.text(format!("{:x}", item.gaitem_handle));

                    ui.table_next_column();
                    ui.text(item.category.to_string());

                    ui.table_next_column();
                    ui.text(item.item_id.to_string());

                    ui.table_next_column();
                    ui.text(item.quantity.to_string());

                    ui.table_next_column();
                    ui.text(item.display_id.to_string());
                });
            }
        }

        let label = format!("Key Items ({}/{})", self.key_item_count, self.key_item_capacity);
        if ui.collapsing_header(label.as_str(), TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-inventory-data-key-items",
                [
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Category"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                ],
            ) {
                self.key_items().iter().for_each(|item| {
                    ui.table_next_column();
                    ui.text(format!("{:x}", item.gaitem_handle));

                    ui.table_next_column();
                    ui.text(item.category.to_string());

                    ui.table_next_column();
                    ui.text(item.item_id.to_string());

                    ui.table_next_column();
                    ui.text(item.quantity.to_string());

                    ui.table_next_column();
                    ui.text(item.display_id.to_string());
                });
            }
        }

        let label = format!("Secondary Key Items ({}/{})", self.secondary_key_item_count, self.secondary_key_item_capacity);
        if ui.collapsing_header(label.as_str(), TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "equip-inventory-data-secondary-key-items",
                [
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Category"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                ],
            ) {
                self.secondary_key_items().iter().for_each(|item| {
                    ui.table_next_column();
                    ui.text(format!("{:x}", item.gaitem_handle));

                    ui.table_next_column();
                    ui.text(item.category.to_string());

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
        ui.text(format!("Team Type: {}", self.team_type));
        ui.text(format!("Last killed by: {}", self.last_killed_by));
        ui.text(format!("Last used item: {}", self.last_used_item));

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
            self.physics.render_debug(ui);
        }

        if ui.collapsing_header("Model param modifier", TreeNodeFlags::empty()) {
            self.model_param_modifier.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrPhysicsModule {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Position: {}", self.position));
    }
}

impl DebugDisplay for CSChrModelParamModifierModule {
    fn render_debug(&self, ui: &&mut Ui) {
        if let Some(_t) = ui.begin_table_header(
            "chr-ins-model-param-modifier",
            [
                TableColumnSetup::new("Name"),
            ],
        ) {
            self.modifiers.iter().for_each(|modifier| {
                ui.table_next_column();
                ui.text(unsafe { modifier.name.to_string() }.unwrap());
            });
        }
    }
}
