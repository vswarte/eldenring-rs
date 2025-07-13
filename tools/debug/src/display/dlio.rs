use eldenring::dlio::DLFileDeviceManager;
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for DLFileDeviceManager {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        ui.input_text("File Device Count", &mut self.devices.len().to_string())
            .read_only(true)
            .build();

        if ui.collapsing_header("Virtual Roots", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header(
                "dl-file-device-manager-virtual-roots",
                [
                    TableColumnSetup::new("Root"),
                    TableColumnSetup::new("Mount"),
                ],
            ) {
                self.virtual_roots.iter().for_each(|vr| {
                    ui.table_next_column();
                    ui.text(vr[0].to_string());
                    ui.table_next_column();
                    ui.text(vr[1].to_string());
                });
            }
            ui.unindent();
        }

        if ui.collapsing_header("BND4 Files", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header(
                "dl-file-device-manager-bnd4-files",
                [
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("File Size"),
                ],
            ) {
                self.bnd4_files.iter().for_each(|file| {
                    ui.table_next_column();
                    ui.text(file.name.to_string());

                    ui.table_next_column();
                    ui.text(file.file_size.to_string());
                });
            }
            ui.unindent();
        }
    }
}
