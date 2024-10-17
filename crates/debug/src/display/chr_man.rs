use std::cell::RefCell;

use game::cs::{PlayerIns, ChrIns, ChrSet, OpenFieldChrSet, SummonBuddyManager, SummonBuddyManagerWarp, WorldChrMan};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for WorldChrMan<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        let mut character_properties = util::character_type_properties::CHARACTER_TYPE_PROPERTIES
            .write()
            .unwrap();

        // character_properties
        //     .table
        //     .entries
        //     .iter_mut()
        //     .for_each(|e| e.can_use_rune_arcs = 0);

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

        match unsafe { self.main_player.as_ref() } {
            Some(p) => {
                if ui.collapsing_header("Main player", TreeNodeFlags::empty()) {
                    p.render_debug(ui)
                }
            }
            None => ui.text("No Main player instance"),
        }

        match unsafe { self.summon_buddy_manager.as_ref() } {
            Some(s) => {
                if ui.collapsing_header("SummonBuddyManager", TreeNodeFlags::empty()) {
                    s.render_debug(ui)
                }
            }
            None => ui.text("No SummonBuddyManager instance"),
        }
    }
}

impl DebugDisplay for ChrSet<'_, ChrIns<'_>> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        // END ME
        let mut current_entry = self.entries;
        let end = unsafe { current_entry.add(self.capacity as usize) };
        while current_entry < end {
            let entry = unsafe { &*current_entry };

            if let Some(chr_ins) = unsafe { entry.chr_ins.as_ref() } {
                if ui.collapsing_header(
                    format!("{:?}", chr_ins.character_id),
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

impl DebugDisplay for ChrSet<'_, PlayerIns<'_>> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        // END ME
        let mut current_entry = self.entries;
        let end = unsafe { current_entry.add(self.capacity as usize) };
        while current_entry < end {
            let entry = unsafe { &*current_entry };

            if let Some(player_ins) = unsafe { entry.chr_ins.as_ref() } {
                if ui.collapsing_header(
                    format!("{:?}", player_ins.chr_ins.character_id),
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

impl DebugDisplay for OpenFieldChrSet<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        self.base.render_debug(ui)
    }
}

impl DebugDisplay for SummonBuddyManager<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "To spawn buddy param: {}",
            self.to_spawn_buddy_param
        ));
        ui.text(format!("Spawned buddy param: {}", self.spawned_buddy_param));

        match unsafe { self.warp.as_ref() } {
            Some(s) => s.render_debug(ui),
            None => ui.text("No SummonBuddyManagerWarp instance"),
        }
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
