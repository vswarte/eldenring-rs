use game::cs::{CSCam, CSCamera};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSCamera<'_> {
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
        ui.text(format!("Unk2c: {}", self.unk2c));
        ui.text(format!("Unk30: {}", self.unk30));
    }
}

impl DebugDisplay for CSCam {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("unk8: {}", self.unk8));
        ui.text(format!("unkc: {}", self.unkc));
        ui.text(format!("Fov: {}", self.fov));
        ui.text(format!("Aspect ratio: {}", self.aspect_ratio));
        ui.text(format!("Far plane: {}", self.far_plane));
        ui.text(format!("Near plane: {}", self.near_plane));

        if ui.collapsing_header("Matrix", TreeNodeFlags::empty()) {
            self.matrix.render_debug(ui);
        }
    }
}
