use game::cs::{CSNetBloodMessageDb, CSNetBloodMessageDbItem, CSNetMan, QuickmatchManager};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSNetMan {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Blood Messages", TreeNodeFlags::empty()) {
            ui.indent();
            self.blood_message_db.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Quickmatch", TreeNodeFlags::empty()) {
            ui.indent();
            self.quickmatch_manager.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for CSNetBloodMessageDb {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Entries", TreeNodeFlags::empty()) {
            ui.indent();
            render_message_table(self.entries.iter().map(|f| f.as_ref()), ui);
            ui.unindent();
        }

        if ui.collapsing_header("Created message data", TreeNodeFlags::empty()) {
            ui.indent();
            self.created_data
                .iter()
                .for_each(|f| ui.text(format!("{f} {f:x}")));
            ui.unindent();
        }

        if ui.collapsing_header("Discovered messages", TreeNodeFlags::empty()) {
            ui.indent();
            render_message_table(
                self.discovered_messages.iter().map(|f| f.as_ref().as_ref()),
                ui,
            );
            ui.unindent();
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

impl DebugDisplay for QuickmatchManager {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("CSBattleRoyalContext", TreeNodeFlags::empty()) {
            ui.indent();

            ui.input_text(
                "Match settings",
                &mut self.battle_royal_context.quickmatch_context.match_settings.to_string(),
            )
            .read_only(true)
            .build();

            ui.input_text(
                "Match map (map ID)",
                &mut self.battle_royal_context.quickmatch_context.match_map.to_string(),
            )
            .read_only(true)
            .build();

            ui.input_text(
                "Match Player Count",
                &mut self.battle_royal_context.match_player_count.to_string(),
            )
            .read_only(true)
            .build();

            ui.input_text(
                "Match Map (enum)",
                &mut self.battle_royal_context.match_player_count.to_string(),
            )
            .read_only(true)
            .build();

            ui.input_text(
                "Password",
                &mut self.battle_royal_context.password.to_string(),
            )
            .read_only(true)
            .build();

            ui.input_text(
                "Participant count",
                &mut self.battle_royal_context.quickmatch_context.participants.len().to_string(),
            )
            .read_only(true)
            .build();


            ui.unindent();
        }


    }
}
