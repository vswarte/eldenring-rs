use super::DebugDisplay;
use game::cs::CSGaitemImp;
use hudhook::imgui::{TableBgTarget, TableColumnSetup, TableFlags, TreeNodeFlags};
use std::cell::RefCell;
use std::time::Instant;

thread_local! {
    static JUMP_TARGET_HANDLE_INDEX: RefCell<i32> = const { RefCell::new(-1) };
    static CURRENT_HIGHLIGHT_INDEX: RefCell<i32> = const { RefCell::new(-1) };
    static JUMP_TARGET_HIGHLIGHT_TIMER: RefCell<Instant> = RefCell::new(Instant::now());
}

const HIGHLIGHT_DURATION_SECS: f32 = 1.5;
const HIGHLIGHT_BASE_COLOR: [f32; 4] = [0.26, 0.59, 0.98, 1.0];

impl DebugDisplay for CSGaitemImp {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        if ui.collapsing_header("Gaitem Inses", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header_with_flags(
                "cs-gaitem-imp-gaiteminses",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Category"),
                    TableColumnSetup::new("Additional"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                for gaitem in self.gaitems.iter().filter_map(|f| f.as_ref()) {
                    let gaitem = gaitem.as_ref();
                    let index = gaitem.gaitem_handle.index() as i32;

                    let target_index = JUMP_TARGET_HANDLE_INDEX.with(|h| *h.borrow());
                    if target_index != -1 && target_index == index {
                        ui.set_scroll_here_y_with_ratio(0.5);
                        JUMP_TARGET_HANDLE_INDEX.with(|h| *h.borrow_mut() = -1);
                        JUMP_TARGET_HIGHLIGHT_TIMER.with(|t| *t.borrow_mut() = Instant::now());
                        CURRENT_HIGHLIGHT_INDEX.with(|h| *h.borrow_mut() = index);
                    }

                    ui.table_next_column();
                    ui.text(format!("{index:?}"));

                    ui.table_next_column();
                    ui.text(gaitem.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(gaitem.item_id.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", gaitem.gaitem_handle.category()));

                    ui.table_next_column();
                    if let Some(wep) = gaitem.as_wep() {
                        let gem_handle = wep.gem_slot_table.gem_slots[0].gaitem_handle;
                        if gem_handle.0 != 0 && ui.button(format!("Gem: {:?}", gem_handle.index()))
                        {
                            JUMP_TARGET_HANDLE_INDEX.with(|h| {
                                // We can safely cast this to i32 because we know max index is 5120
                                *h.borrow_mut() = gem_handle.index() as i32;
                            });
                        }
                    } else if let Some(gem) = gaitem.as_gem() {
                        if gem.weapon_handle.0 != 0
                            && ui.button(format!("Weapon: {:?}", gem.weapon_handle.index()))
                        {
                            JUMP_TARGET_HANDLE_INDEX.with(|h| {
                                *h.borrow_mut() = gem.weapon_handle.index() as i32;
                            });
                        }
                    }

                    let highlight_index = CURRENT_HIGHLIGHT_INDEX.with(|h| *h.borrow());
                    if highlight_index == index {
                        let elapsed_secs = JUMP_TARGET_HIGHLIGHT_TIMER
                            .with(|t| t.borrow().elapsed())
                            .as_secs_f32();

                        if elapsed_secs < HIGHLIGHT_DURATION_SECS {
                            let alpha = (1.0 - (elapsed_secs / HIGHLIGHT_DURATION_SECS)).max(0.0);
                            let highlight_color = [
                                HIGHLIGHT_BASE_COLOR[0],
                                HIGHLIGHT_BASE_COLOR[1],
                                HIGHLIGHT_BASE_COLOR[2],
                                alpha * 0.6,
                            ];
                            ui.table_set_bg_color(TableBgTarget::ROW_BG1, highlight_color);
                        } else {
                            CURRENT_HIGHLIGHT_INDEX.with(|h| {
                                if *h.borrow() == index {
                                    *h.borrow_mut() = -1;
                                }
                            });
                        }
                    }
                }
            }
            ui.unindent();
        }
    }
}
