use game::world_area_time::WorldAreaTime;
use hudhook::imgui::Ui;

use super::DebugDisplay;

impl DebugDisplay for WorldAreaTime {
    fn render_debug(&self, ui: &&mut Ui) {
        let year = self.clock.year();
        ui.text(format!("Year: {year}"));

        let month = self.clock.month();
        ui.text(format!("Month: {month}"));

        let day_of_week = self.clock.day_of_week();
        ui.text(format!("Day of week: {day_of_week}"));

        let day = self.clock.day();
        ui.text(format!("Day: {day}"));

        let hours = self.clock.hours();
        ui.text(format!("Hours: {hours}"));

        let minutes = self.clock.minutes();
        ui.text(format!("Minutes: {minutes}"));

        let seconds = self.clock.seconds();
        ui.text(format!("Seconds: {seconds}"));
    }
}
