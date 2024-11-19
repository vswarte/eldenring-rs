use game::cs::MsbRepository;
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for MsbRepository {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        if ui.collapsing_header("Resources", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "msb-repository-rescaps",
                [
                    TableColumnSetup::new("Name"),
                ],
            ) {
                for msb in self.res_rep.res_cap_holder.entries() {
                    ui.table_next_column();
                    ui.text(msb.file_cap.res_cap.name.to_string());
                }
            }
        }
    }
}
