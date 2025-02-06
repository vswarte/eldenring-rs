use game::cs::{CSSessionManager, CSSessionManagerPlayerEntry};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSSessionManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Lobby state: {:?}", self.lobby_state));
        ui.text(format!("Protocol state: {:?}", self.protocol_state));

        if ui.collapsing_header("Members", TreeNodeFlags::empty()) {
            ui.indent();
            for player in self.players.items() {
                player.render_debug(ui);
            }
            ui.unindent();
        }

        if self.host_player.steam_id != 0x0 && ui.collapsing_header("Host", TreeNodeFlags::empty())
        {
            ui.indent();
            self.host_player.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for CSSessionManagerPlayerEntry {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.input_text("Steam Name", &mut self.steam_name.to_string())
            .read_only(true)
            .build();
        ui.input_text("Steam ID", &mut self.steam_id.to_string())
            .read_only(true)
            .build();
    }
}
