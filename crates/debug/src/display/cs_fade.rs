use game::cs::{CSFD4FadePlate, CSFD4FadePlateColor, CSFade};
use hudhook::imgui::{TreeNodeFlags, Ui};

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

                let fade_plate = unsafe { &mut *((*fade_plate) as *const CSFD4FadePlate as *mut CSFD4FadePlate) };
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
        ui.text(format!(
            "Current color: ({}, {}, {}, {})",
            self.current_color.r,
            self.current_color.g,
            self.current_color.b,
            self.current_color.a,
        ));
        ui.text(format!(
            "Transition color: ({}, {}, {}, {})",
            self.transition_color.r,
            self.transition_color.g,
            self.transition_color.b,
            self.transition_color.a,
        ));
        ui.text(format!(
            "Target color: ({}, {}, {}, {})",
            self.target_color.r,
            self.target_color.g,
            self.target_color.b,
            self.target_color.a,
        ));
        ui.text(format!("Fade timer: {}", self.fade_timer.time));
        ui.text(format!("Fade duration: {}", self.fade_duration.time));
        ui.text(format!("Unk60: {}", self.unk60));
        ui.text(format!("UnkA8: {}", self.unka8.time));
    }
}

