use game::cs::{
    ChrIns, ChrSet, EquipInventoryData, FieldInsHandle, FieldInsSelector, MapId, OpenFieldChrSet,
    PlayerGameData, PlayerIns, SummonBuddyManager, SummonBuddyManagerWarp, WorldChrMan,
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};
use util::{singleton::get_instance, world_chr_man::{ChrDebugSpawnRequest, WorldChrManExt}};

use super::DebugDisplay;

impl DebugDisplay for WorldChrMan {
    fn render_debug(&self, ui: &&mut Ui) {
        let world_area_chr_list_count = self.world_area_chr_list_count;
        ui.text(format!(
            "World Area Chr List Count: {world_area_chr_list_count}"
        ));

        let world_block_chr_list_count = self.world_block_chr_list_count;
        ui.text(format!(
            "World Block Chr List Count: {world_block_chr_list_count}"
        ));

        let world_grid_area_chr_list_count = self.world_grid_area_chr_list_count;
        ui.text(format!(
            "World Grid Area Chr List Count: {world_grid_area_chr_list_count}"
        ));

        let world_area_list_count = self.world_area_list_count;
        ui.text(format!("World Area List Count: {world_area_list_count}"));

        if ui.collapsing_header("Player ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.player_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Ghost ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.ghost_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("SummonBuddy ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.summon_buddy_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Debug ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.debug_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Open Field ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.open_field_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("All ChrSets", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, entry) in self.chr_sets.iter().enumerate() {
                let Some(chr_set) = entry else {
                    continue;
                };

                if ui.collapsing_header(format!("ChrSet {i}"), TreeNodeFlags::empty()) {
                    chr_set.render_debug(ui);
                }
            }
            ui.unindent();
        }

        match self.main_player.as_ref() {
            Some(p) => {
                if ui.collapsing_header("Main player", TreeNodeFlags::empty()) {
                    p.render_debug(ui)
                }
            }
            None => ui.text("No Main player instance"),
        }

        match self.summon_buddy_manager.as_ref() {
            Some(s) => {
                if ui.collapsing_header("SummonBuddyManager", TreeNodeFlags::empty()) {
                    s.render_debug(ui)
                }
            }
            None => ui.text("No SummonBuddyManager instance"),
        }

        if ui.collapsing_header("Debug Character Creator", TreeNodeFlags::empty()) {
            let last_created_characer = self.debug_chr_creator.last_created_chr;
            ui.input_text("Last Created Character", &mut format!("{:x?}", self.debug_chr_creator.last_created_chr))
                .read_only(true)
                .build();

            if ui.button("Spawn Character Creator") {
                let world_chr_man = unsafe { get_instance::<WorldChrMan>() }.unwrap().unwrap();
                if let Some(main_player) = &world_chr_man.main_player {
                    let (x, y, z) = main_player.chr_ins.module_container.physics.position.xyz();

                    world_chr_man.spawn_debug_character(&ChrDebugSpawnRequest {
                        chr_id: 4730,
                        chara_init_param_id: 0,
                        npc_param_id: 47300000,
                        npc_think_param_id: 47300000,
                        event_entity_id: 0,
                        talk_id: 0,
                        pos_x: x,
                        pos_y: y,
                        pos_z: z,
                    });
                }
            }
        }
    }
}

impl DebugDisplay for ChrSet<ChrIns> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        if ui.collapsing_header("Characters", TreeNodeFlags::empty()) {
            ui.indent();
            self.characters().for_each(|chr_ins| {
                if ui.collapsing_header(
                    format!(
                        "c{:0>4} - {} FieldInsSelector({}, {})",
                        chr_ins.character_id,
                        chr_ins.field_ins_handle.map_id,
                        chr_ins.field_ins_handle.selector.container(),
                        chr_ins.field_ins_handle.selector.index()
                    ),
                    TreeNodeFlags::empty(),
                ) {
                    chr_ins.render_debug(ui)
                }
            });
            ui.unindent();
        }

        if ui.collapsing_header("Character event ID mapping", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Event ID"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
            ) {
                self.entity_id_mapping.iter().for_each(|e| {
                    ui.table_next_column();
                    ui.text(e.entity_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", chr_ins.field_ins_handle));
                });
            }
            ui.unindent();
        }

        if ui.collapsing_header("Group mapping", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Group"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
            ) {
                self.group_id_mapping.iter().for_each(|e| {
                    ui.table_next_column();
                    ui.text(e.group_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", chr_ins.field_ins_handle));
                });
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for ChrSet<PlayerIns> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        let mut current_entry = self.entries;
        let end = unsafe { current_entry.add(self.capacity as usize) };
        while current_entry < end {
            let entry = unsafe { current_entry.as_ref() };

            let player_ins = unsafe { entry.chr_ins.as_ref() };

            if ui.collapsing_header(
                format!(
                    "c{:0>4} - {} FieldInsSelector({}, {})",
                    player_ins.chr_ins.character_id,
                    player_ins.chr_ins.field_ins_handle.map_id,
                    player_ins.chr_ins.field_ins_handle.selector.container(),
                    player_ins.chr_ins.field_ins_handle.selector.index()
                ),
                TreeNodeFlags::empty(),
            ) {
                player_ins.render_debug(ui)
            }

            unsafe {
                current_entry = current_entry.add(1);
            }
        }
    }
}

impl DebugDisplay for OpenFieldChrSet {
    fn render_debug(&self, ui: &&mut Ui) {
        self.base.render_debug(ui)
    }
}

impl DebugDisplay for SummonBuddyManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "To spawn buddy param: {}",
            self.to_spawn_buddy_param
        ));
        ui.text(format!("Spawned buddy param: {}", self.spawned_buddy_param));

        self.warp.render_debug(ui);
    }
}

impl DebugDisplay for SummonBuddyManagerWarp {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "Trigger time ray block: {}",
            self.trigger_time_ray_block
        ));
        ui.text(format!(
            "Trigger dist to player: {}",
            self.trigger_dist_to_player
        ));
        ui.text(format!(
            "Trigger threshold time path stacked: {}",
            self.trigger_threshold_time_path_stacked
        ));
        ui.text(format!(
            "Trigger treshhold range path stacked: {}",
            self.trigger_threshold_range_path_stacked
        ));
    }
}
