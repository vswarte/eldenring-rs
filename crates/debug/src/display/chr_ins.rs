use game::cs::{ChrIns, ChrInsModuleContainer, ChrPhysicsModule, FieldInsHandle, PlayerIns};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;


impl DebugDisplay for PlayerIns<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.chr_ins.render_debug(ui);

        if ui.collapsing_header("Map relative position", TreeNodeFlags::empty()) {
            self.map_relative_position.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrIns<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.field_ins_handle.render_debug(ui);

        ui.text(format!("Map ID 1: {:?}", self.map_id_1));
        ui.text(format!("Map ID origin 1: {}", self.map_id_origin_1));
        ui.text(format!("Map ID 2: {:?}", self.map_id_2));
        ui.text(format!("Map ID origin 2: {}", self.map_id_origin_2));
        ui.text(format!("Last used item?: {}", self.last_used_item));
        ui.text(format!("Character ID?: {}", self.character_id));

        if ui.collapsing_header("Modules", TreeNodeFlags::empty()) {
            self.module_container.render_debug(ui);
        }
    }
}

impl DebugDisplay for FieldInsHandle {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Field Ins ID: {}", self.instance_id));
        ui.text(format!("Field Ins map ID: {:?}", self.map_id));
    }
}

impl DebugDisplay for ChrInsModuleContainer<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Physics", TreeNodeFlags::empty()) {
            self.physics.render_debug(ui);
        }
    }
}

impl DebugDisplay for ChrPhysicsModule<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Unk70 position", TreeNodeFlags::empty()) {
            self.unk70_position.render_debug(ui);
        }

        if ui.collapsing_header("Unk80 position", TreeNodeFlags::empty()) {
            self.unk80_position.render_debug(ui);
        }
    }
}
