use game::cs::{CSWorldGeomIns, CSWorldGeomMan, CSWorldGeomManMapData};
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

        if ui.collapsing_header("Geometry Vector", TreeNodeFlags::empty()) {
            for geometry_ins in unsafe { self.geom_ins_vector.iter() } {
                let name = unsafe { geometry_ins.info.msb_parts_geom.msb_parts.msb_part.name.to_string() }.unwrap();
                if ui.collapsing_header(
                    format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.map_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    TreeNodeFlags::empty(),
                ) {
                    geometry_ins.render_debug(ui)
                }
            }
        }

        if ui.collapsing_header("Sign Geometry Vector", TreeNodeFlags::empty()) {
            for geometry_ins in unsafe { self.sos_sign_geometry.iter() } {
                let name = unsafe { geometry_ins.info.msb_parts_geom.msb_parts.msb_part.name.to_string() }.unwrap();
                if ui.collapsing_header(
                    format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.map_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    TreeNodeFlags::empty(),
                ) {
                    geometry_ins.render_debug(ui)
                }
            }
        }
    }
}

impl DebugDisplay for CSWorldGeomIns<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Unk10: {}", self.info.unk10));
    }
}
