use game::cs::{CSCam, CSCamera};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSCamera {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Pers cam 1", TreeNodeFlags::empty()) {
            self.pers_cam_1.render_debug(ui);
        }

        if ui.collapsing_header("Pers cam 2", TreeNodeFlags::empty()) {
            self.pers_cam_2.render_debug(ui);
        }

        if ui.collapsing_header("Pers cam 3", TreeNodeFlags::empty()) {
            self.pers_cam_3.render_debug(ui);
        }

        if ui.collapsing_header("Pers cam 4", TreeNodeFlags::empty()) {
            self.pers_cam_4.render_debug(ui);
        }

        ui.text(format!("Camera mask: {}", self.camera_mask));
    }
}

impl DebugDisplay for CSCam {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Fov: {}", self.fov));
        ui.text(format!("Aspect ratio: {}", self.aspect_ratio));
        ui.text(format!("Far plane: {}", self.far_plane));
        ui.text(format!("Near plane: {}", self.near_plane));

        if ui.collapsing_header("Matrix", TreeNodeFlags::empty()) {
            self.matrix.render_debug(ui);
        }
    }
}
