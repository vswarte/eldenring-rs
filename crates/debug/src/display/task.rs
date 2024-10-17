use game::cs::{CSTaskGroup, CSTaskImp};
use hudhook::imgui::*;

use super::DebugDisplay;

impl DebugDisplay for CSTaskGroup<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        for task_group in self.task_groups.iter() {
            ui.text(task_group.base.name.string.to_string());
        }
    }
}

impl DebugDisplay for CSTaskImp<'_> {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Task Groups", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "task-group-table",
                [
                    TableColumnSetup::new("ID"),
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("Active"),
                ],
            ) {
                for task_group in unsafe { self.inner.task_base.task_groups.iter() } {
                    ui.table_next_column();
                    ui.text(format!("{:x}", task_group.index));

                    let name_bytes = task_group.name.iter()
                        .take_while(|c| **c != 0x0)
                        .cloned()
                        .collect::<Vec<_>>();
                    let name = String::from_utf16(name_bytes.as_slice()).unwrap();

                    ui.table_next_column();
                    ui.text(name);

                    ui.table_next_column();
                    ui.text(format!("{}", task_group.active));
                }
            }
        }

        if ui.collapsing_header("Runners", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header(
                "task-runner-table",
                [
                    TableColumnSetup::new("Queue"),
                    TableColumnSetup::new("Unk string"),
                ],
            ) {
                for runner in self.inner.task_runners.iter() {
                    ui.table_next_column();
                    ui.text(format!("{:x}", runner.task_queue));

                    ui.table_next_column();
                    ui.text(unsafe { runner.unk_string.to_string().unwrap() });
                }
            }
        }
    }
}
