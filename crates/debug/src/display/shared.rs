use game::{
    matrix::{Matrix4X4, Vector4},
    DoublyLinkedList,
};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for Matrix4X4 {
    fn render_debug(&self, ui: &&mut Ui) {
        self.0.render_debug(ui);
        ui.separator();
        self.1.render_debug(ui);
        ui.separator();
        self.2.render_debug(ui);
        ui.separator();
        self.3.render_debug(ui);
        ui.separator();
    }
}

impl DebugDisplay for Vector4 {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("x: {}", self.0));
        ui.text(format!("y: {}", self.1));
        ui.text(format!("z: {}", self.2));
        ui.text(format!("w: {}", self.3));
    }
}
