use shared::{FSMatrix4x4, FSVector4};
use hudhook::imgui::Ui;

use super::DebugDisplay;

impl DebugDisplay for FSMatrix4x4 {
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

impl DebugDisplay for FSVector4 {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("x: {}", self.0));
        ui.text(format!("y: {}", self.1));
        ui.text(format!("z: {}", self.2));
        ui.text(format!("w: {}", self.3));
    }
}
