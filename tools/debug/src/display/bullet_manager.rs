use game::cs::{CSBulletIns, CSBulletManager};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSBulletManager {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("BulletInses", TreeNodeFlags::empty()) {
            ui.indent();
            self.bullets().for_each(|b| {
                if ui.collapsing_header(format!("{}", b.field_ins_handle), TreeNodeFlags::empty()) {
                    ui.indent();
                    b.render_debug(ui);
                    ui.unindent();
                }
            });
            ui.unindent();
        }
    }
}

impl DebugDisplay for CSBulletIns {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Physics", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!("Position: {}", self.physics.position));
            ui.text(format!("Orientation: {}", self.physics.orientation));
            ui.text(format!("Velocity: {:?}", self.physics.velocity));
            ui.unindent();
        }
    }
}
