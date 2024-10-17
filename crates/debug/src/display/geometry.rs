use game::cs::{CSWorldGeomMan, CSWorldGeomManMapData};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSWorldGeomMan<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Map data", TreeNodeFlags::empty()) {
            ui.text(format!("Map data count: {}", unsafe { self.map_geometry.len() }));
        }

        if ui.collapsing_header("Current Unk Map", TreeNodeFlags::empty()) {
            self.curent_99_map_data.render_debug(ui);
        }
    }
}

impl DebugDisplay for CSWorldGeomManMapData<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Map ID: {}", self.map_id));
        ui.text(format!("World block info: {:x}", self.world_block_info));
        ui.text(format!("Next GeomIns FieldIns index: {}", self.next_geom_ins_field_ins_index));
        ui.text(format!("Reached GeomIns vector capacity: {}", self.reached_geom_ins_vector_capacity));

        if ui.collapsing_header("Geometry", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                format!("geometry-ins-{}-table", self.map_id),
                [
                    TableColumnSetup::new("Map ID"),
                    TableColumnSetup::new("Instance ID"),
                ],
            ) {
                for geometry_ins in unsafe { self.geom_ins_vector.iter() } {
                    ui.table_next_column();
                    ui.text(format!("{}", geometry_ins.field_ins_handle.map_id));

                    ui.table_next_column();
                    ui.text(format!("{}", geometry_ins.field_ins_handle.instance_id));
                }
            }
        }
    }
}
