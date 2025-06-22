use eldenring::cs::CSGaitemImp;
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for CSGaitemImp {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        if ui.collapsing_header("Gaitem Inses", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "cs-gaitem-imp-gaiteminses",
                [
                    TableColumnSetup::new("Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Category"),
                    TableColumnSetup::new("Additional"),
                ],
            ) {
                ui.indent();
                for gaitem in self.gaitems.iter().filter_map(|f| f.as_ref()) {
                    let gaitem = gaitem.as_ref();

                    ui.table_next_column();
                    ui.text(format!("{:x?}", gaitem.gaitem_handle));

                    ui.table_next_column();
                    ui.text(format!("{:?}", gaitem.item_id));

                    ui.table_next_column();
                    ui.text(format!("{:?}", gaitem.gaitem_handle.category()));

                    ui.table_next_column();
                    if let Some(wep) = gaitem.as_wep() {
                        ui.text(format!(
                            "AoW handle: {:x?}",
                            wep.gem_slot_table.gem_slots[0].gaitem_handle
                        ));
                    } else if let Some(gem) = gaitem.as_gem() {
                        ui.text(format!("Gem item ID: {:?}", gem.item_id));
                    } else {
                        ui.text("");
                    }
                }
                ui.unindent();
            }
        }
    }
}
