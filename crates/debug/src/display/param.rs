use game::fd4::FD4ParamRepository;
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags};
use util::resource::FD4ResCapHolderExt;

use super::DebugDisplay;

impl DebugDisplay for FD4ParamRepository {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        ui.text(format!("ResCapHolder map bucket count: {:?}", self.res_cap_holder.bucket_count));
        ui.text(format!("ResCapHolder bucket base: {:x?}", self.res_cap_holder.buckets));
        ui.text(format!("ResCapHolder unk18: {:?}", self.res_cap_holder.unk18));

        if ui.collapsing_header("Resources", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "fd4-param-repository-rescaps",
                [
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("File Size"),
                    TableColumnSetup::new("Row Count"),
                    TableColumnSetup::new("Paramdef Version"),
                    TableColumnSetup::new("Bytes"),
                ],
            ) {
                for res_cap in unsafe { self.res_cap_holder.entries() } {
                    ui.table_next_column();
                    ui.text(res_cap.header.name.to_string());

                    ui.table_next_column();
                    ui.text(format!("{} bytes", res_cap.data.file_size.to_string()));

                    ui.table_next_column();
                    let row_count = unsafe { res_cap.data.header.as_ref() }.map(|p| p.row_count);
                    ui.text(format!("{:?}", row_count));

                    ui.table_next_column();
                    let paramdef_version = unsafe { res_cap.data.header.as_ref() }.map(|p| p.paramdef_version);
                    ui.text(format!("{:?}", paramdef_version));

                    ui.table_next_column();
                    let bytes_ptr = res_cap.data.header as usize;
                    ui.text(format!("{:x}", bytes_ptr as usize));
                }
            }
        }
    }
}
