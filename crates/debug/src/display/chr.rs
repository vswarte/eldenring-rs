use game::cs::{
    CSChrModelParamModifierModule, ChrAsm, ChrIns, ChrInsModuleContainer, ChrPhysicsModule,
    PlayerGameData, PlayerIns,
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for PlayerIns<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.chr_ins.render_debug(ui);

        if ui.collapsing_header("ChrAsm", TreeNodeFlags::empty()) {
            self.chr_asm.render_debug(ui);
        }

        if ui.collapsing_header("PlayerGameData", TreeNodeFlags::empty()) {
            self.player_game_data.render_debug(ui);
        }

        ui.text(format!("Unk position: {}", self.chunk_position));
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
    }
}

impl DebugDisplay for ChrIns<'_> {
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

impl DebugDisplay for ChrInsModuleContainer<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Physics", TreeNodeFlags::empty()) {
            self.physics.render_debug(ui);
        }

        if ui.collapsing_header("Model param modifier", TreeNodeFlags::empty()) {
            self.model_param_modifier.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrPhysicsModule<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Position: {}", self.position));
    }
}

impl DebugDisplay for CSChrModelParamModifierModule<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if let Some(_t) = ui.begin_table_header(
            "chr-ins-model-param-modifier",
            [TableColumnSetup::new("Unk0"), TableColumnSetup::new("Name")],
        ) {
            self.modifiers.iter().for_each(|modifier| {
                ui.table_next_column();
                ui.text(format!("{:x}", modifier.unk0));

                ui.table_next_column();
                ui.text(unsafe { modifier.name.to_string() }.unwrap());
            });
        }
    }
}
