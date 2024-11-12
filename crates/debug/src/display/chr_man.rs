use game::cs::{
    ChrIns, ChrSet, EquipInventoryData, FieldInsHandle, FieldInsSelector, MapId, OpenFieldChrSet,
    PlayerGameData, PlayerIns, SummonBuddyManager, SummonBuddyManagerWarp, WorldChrMan,
};
use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};
use util::singleton::get_instance;

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
            self.player_chr_set.render_debug(ui);
        }

        if ui.collapsing_header("Ghost ChrSet", TreeNodeFlags::empty()) {
            self.ghost_chr_set.render_debug(ui);
        }

        if ui.collapsing_header("SummonBuddy ChrSet", TreeNodeFlags::empty()) {
            self.summon_buddy_chr_set.render_debug(ui);
        }

        if ui.collapsing_header("Debug ChrSet", TreeNodeFlags::empty()) {
            self.debug_chr_set.render_debug(ui);
        }

        if ui.collapsing_header("Open Field ChrSet", TreeNodeFlags::empty()) {
            self.open_field_chr_set.render_debug(ui);
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
    }
}

impl DebugDisplay for ChrSet<ChrIns> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        let mut current_entry = self.entries;
        let end = unsafe { current_entry.add(self.capacity as usize) };
        while current_entry < end {
            let entry = unsafe { &*current_entry };

            if let Some(chr_ins) = unsafe { entry.chr_ins.as_ref() } {
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
            }

            unsafe {
                current_entry = current_entry.add(1);
            }
        }
    }
}

impl DebugDisplay for ChrSet<PlayerIns> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        let test = unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .map(|w| w.player_chr_set.chr_ins_by_field_ins_handle(
                FieldInsHandle {
                    selector: FieldInsSelector(384827392),
                    map_id: MapId::global(),
                }
            ))
            .flatten()
            .map(|c| format!("{}", c.field_ins_handle));

        ui.text(format!("Main player handle from VMT {:?}", test));

        let mut current_entry = self.entries;
        let end = unsafe { current_entry.add(self.capacity as usize) };
        while current_entry < end {
            let entry = unsafe { &*current_entry };

            if let Some(player_ins) = unsafe { entry.chr_ins.as_ref() } {
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
