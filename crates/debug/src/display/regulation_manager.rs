use game::cs::{CSRegulationManager};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSRegulationManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("unk_0x84: {}", self.unk_0x84));
    }
}