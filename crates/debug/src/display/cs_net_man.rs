use game::cs::{CSNetBloodMessageDb, CSNetMan};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl<'a> DebugDisplay for CSNetMan<'a> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.blood_message_db.render_debug(ui);
    }
}

impl<'a> DebugDisplay for CSNetBloodMessageDb<'a> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Entries", TreeNodeFlags::empty()) {
            self.entries.render_debug(ui);
        }

        ui.text(format!("Unk20: {}", self.unk20));

        if ui.collapsing_header("Created messages", TreeNodeFlags::empty()) {
            self.created_data.render_debug(ui);
        }

        if ui.collapsing_header("Unk40", TreeNodeFlags::empty()) {
            self.unk40.render_debug(ui);
        }

        ui.text(format!("Unk58: {}", self.unk58));

        if ui.collapsing_header("Discovered messages", TreeNodeFlags::empty()) {
            self.discovered_messages.render_debug(ui);
        }
    }
}
