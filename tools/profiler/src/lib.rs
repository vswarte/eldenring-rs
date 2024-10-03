use std::{
    cell::{Cell, RefCell},
    pin::Pin,
    sync::Arc,
};

use game::cs::{CSTaskGroupIndex, CSTaskImp, FD4TaskRequestEntry};
use pelite::pe::*;
use retour::static_detour;
use util::{program::Program, rtti::find_rtti_classes, singleton::get_instance, task::TaskRuntime};

static_detour! {
    static FD4_EXECUTE_TASK_DETOUR: extern "C" fn(usize, *const FD4TaskRequestEntry, u32, u32);
}

#[no_mangle]
pub unsafe extern "C" fn DllMain(_base: usize, reason: u32) -> bool {
    if reason == 1 {
        start_puffin_server();

        FD4_EXECUTE_TASK_DETOUR.initialize(
            std::mem::transmute(0x1426d54d0usize),
            |task_group, request_entry, task_group_index, task_runner_index| {
                puffin::profile_scope!(format!("{task_group_index:x?}"));

                FD4_EXECUTE_TASK_DETOUR.call(
                    task_group,
                    request_entry,
                    task_group_index,
                    task_runner_index,
                );
            },
        ).unwrap();
        FD4_EXECUTE_TASK_DETOUR.enable().unwrap();

        std::thread::spawn(|| {
            // Wait for CSTask to become a thing
            // TODO: write optimized wait for with thread parking?
            std::thread::sleep(std::time::Duration::from_secs(10));

            let task = get_instance::<CSTaskImp>().unwrap().unwrap();
            // TODO: manage the lifetime around the tasks
            // std::mem::forget(task.run_task(
            //     |_, _| puffin::GlobalProfiler::lock().new_frame(),
            //     CSTaskGroupIndex::FrameBegin,
            // ));
        });
    }

    true
}

fn start_puffin_server() {
    puffin::set_scopes_on(true);
    let puffin_server = puffin_http::Server::new("127.0.0.1:8585").unwrap();
    #[allow(clippy::mem_forget)]
    std::mem::forget(puffin_server);
}
