use game::cs::CSSessionManager;
use hudhook::imgui::Ui;

use super::DebugDisplay;

impl DebugDisplay for CSSessionManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Lobby state: {:?}", self.lobby_state));
        ui.text(format!("Protocol state: {:?}", self.protocol_state));
    }
}
