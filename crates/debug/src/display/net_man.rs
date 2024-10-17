use game::cs::{CSNetBloodMessageDb, CSNetBloodMessageDbItem, CSNetMan};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl<'a> DebugDisplay for CSNetMan<'a> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Blood Messages", TreeNodeFlags::empty()) {
            self.blood_message_db.render_debug(ui);
        }
    }
}

impl<'a> DebugDisplay for CSNetBloodMessageDb<'a> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Entries", TreeNodeFlags::empty()) {
            render_message_table(unsafe { self.entries.iter() }.map(|f| *f), ui);
        }

        ui.text(format!("Unk20: {}", self.unk20));

        if ui.collapsing_header("Created message data", TreeNodeFlags::empty()) {
            unsafe { self.created_data.iter() }.for_each(|f| ui.text(format!("{f} {f:x}")))
        }

        ui.text(format!("Unk58: {}", self.unk58));

        if ui.collapsing_header("Discovered messages", TreeNodeFlags::empty()) {
            render_message_table(unsafe { self.discovered_messages.iter() }.map(|f| **f), ui);
        }
    }
}

fn render_message_table<'a>(
    messages: impl Iterator<Item = &'a CSNetBloodMessageDbItem>,
    ui: &&mut Ui,
) {
    if let Some(_t) = ui.begin_table_header(
        "cs-net-man-blood-messages-entries",
        [
            TableColumnSetup::new("Message ID"),
            TableColumnSetup::new("Map ID"),
            TableColumnSetup::new("Placement (x, y, z, angle)"),
            TableColumnSetup::new("Template 1"),
            TableColumnSetup::new("Part 1"),
            TableColumnSetup::new("Infix"),
            TableColumnSetup::new("Template 2"),
            TableColumnSetup::new("Part 2"),
            TableColumnSetup::new("Gesture"),
        ],
    ) {
        messages.for_each(|message| {
            ui.table_next_column();
            ui.text(format!("{:x}", message.message_id));

            ui.table_next_column();
            ui.text(message.map_id.to_string());

            ui.table_next_column();
            ui.text(format!(
                "{}, {}, {}, {}",
                message.position_x, message.position_y, message.position_z, message.angle,
            ));

            ui.table_next_column();
            ui.text(message.template1.to_string());

            ui.table_next_column();
            ui.text(message.part1.to_string());

            ui.table_next_column();
            ui.text(message.infix.to_string());

            ui.table_next_column();
            ui.text(message.template2.to_string());

            ui.table_next_column();
            ui.text(message.part2.to_string());

            ui.table_next_column();
            ui.text(message.gesture_param.to_string());
        });
    }
}
