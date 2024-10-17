use game::cs::{CSFD4FadePlate, CSFade};
use hudhook::imgui::{TreeNodeFlags, Ui};
use util::fade::CSFD4FadePlateExt;

use super::DebugDisplay;

impl DebugDisplay for CSFade<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text("Fade plates");
        for fade_plate in self.fade_plates.iter() {
            let title = unsafe {
                windows::core::PCWSTR::from_raw(fade_plate.title.as_ptr())
                    .to_string()
                    .unwrap()
            };

            if ui.collapsing_header(title, TreeNodeFlags::empty()) {
                fade_plate.render_debug(ui);

                let fade_plate = unsafe {
                    &mut *((*fade_plate) as *const CSFD4FadePlate as *mut CSFD4FadePlate)
                };

                if ui.button("Fade out") {
                    fade_plate.fade_out(2.0);
                }

                if ui.button("Fade in") {
                    fade_plate.fade_in(2.0);
                }
            }
        }
    }
}

impl DebugDisplay for CSFD4FadePlate {
    fn render_debug(&self, ui: &&mut Ui) {
        let mut current_color: [f32; 4] = (&self.current_color).into();
        if ui.color_edit4("current_color", &mut current_color) {
            // TODO
        }

        let mut start_color: [f32; 4] = (&self.start_color).into();
        if ui.color_edit4("start_color", &mut start_color) {
            // TODO
        }

        let mut end_color: [f32; 4] = (&self.end_color).into();
        if ui.color_edit4("end_color", &mut end_color) {
            // TODO
        }

        ui.text(format!("Fade timer: {}", self.fade_timer.time));
        ui.text(format!("Fade duration: {}", self.fade_duration.time));
        ui.text(format!("Unk60: {}", self.unk60));
        ui.text(format!("UnkA8: {}", self.unka8.time));
    }
}
