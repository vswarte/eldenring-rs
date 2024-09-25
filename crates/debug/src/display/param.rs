use game::fd4::FD4ParamRepository;
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for FD4ParamRepository {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        ui.text(format!("ResCapHolder map capacity: {:?}", self.res_cap_holder.capacity));
        ui.text(format!("ResCapHolder unk18 unk18: {:?}", self.res_cap_holder.unk18));

        if ui.collapsing_header("Resources", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "fd4-param-repository-rescaps",
                [
                    TableColumnSetup::new("Name"),
                ],
            ) {
                for res_cap in self.res_cap_holder.entries() {
                    ui.table_next_column();
                    ui.text(res_cap.header.name.to_string());
                }
            }
        }
    }
}
