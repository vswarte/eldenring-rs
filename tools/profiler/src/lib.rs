use std::{collections::HashMap, mem::transmute, sync::{Arc, LazyLock, Mutex, RwLock}};

use game::{cs::{CSEzTaskProxy, CSTaskGroupIndex, CSTaskImp}, fd4::{FD4TaskBase, FD4TaskData, FD4TaskRequestEntry}};
use retour::static_detour;
use tracy_client::span;
use util::{
    program::Program, rtti::vftable_classname, singleton::get_instance, task::CSTaskImpExt,
};

static_detour! {
    static FD4_EXECUTE_TASK_DETOUR: extern "C" fn(usize, *const FD4TaskRequestEntry, u32, u32);
}

#[no_mangle]
pub unsafe extern "C" fn DllMain(_base: usize, reason: u32) -> bool {
    if reason == 1 {
        let tracy = tracy_client::Client::start();
        {
            FD4_EXECUTE_TASK_DETOUR
                .initialize(
                    std::mem::transmute(0x1426d54a0usize),
                    move |task_group, request_entry, task_group_index, task_runner_index| {
                        let task = unsafe { request_entry.as_ref() }
                            .map(|r| r.task.as_ref())
                            .map(label_task)
                            .flatten()
                            .unwrap_or(String::from("Unknown Task Type"));

                        let task_group_label: CSTaskGroupIndex = unsafe { transmute(task_group_index - 0x90000000) };
                        let span_label = format!("{task_group_label:?} {task}");
                        let _span = tracy_client::Client::running().map(|c| {
                            c.span_alloc(
                                Some(span_label.as_str()),
                                "FD4TaskExecute",
                                "profiler.rs",
                                0,
                                0,
                            )
                        });

                        FD4_EXECUTE_TASK_DETOUR.call(
                            task_group,
                            request_entry,
                            task_group_index,
                            task_runner_index,
                        );
                    },
                )
                .unwrap()
                .enable()
                .unwrap();
        }

        std::thread::spawn(move || {
            // Wait for CSTask to become a thing
            // TODO: write optimized wait for with thread parking?
            std::thread::sleep(std::time::Duration::from_secs(5));

            let task = get_instance::<CSTaskImp>().unwrap().unwrap();
            std::mem::forget(task.run_recurring(
                move |_: &FD4TaskData| tracy.frame_mark(),
                CSTaskGroupIndex::FrameEnd,
            ));
        });
    }

    true
}

/// Determines the label for a given FD4TaskBase instance
fn label_task(task: &FD4TaskBase) -> Option<String> {
    Some(String::from("Unknown"))
    // let mut name = lookup_rtti_classname(&*task.vftable as usize)?;
    // if name.as_str() == "CS::CSEzTaskProxy" {
    //     let proxied_task_vftable = unsafe {
    //         (task as *const FD4TaskBase as *const CSEzTaskProxy)
    //             .as_ref()
    //             .map(|p| p.task.as_ref().vftable as usize)?
    //     };
    //
    //     name = lookup_rtti_classname(proxied_task_vftable)?;
    // }
    //
    // Some(name)
}

const VFTABLES: LazyLock<RwLock<HashMap<usize, Option<String>>>> = LazyLock::new(Default::default);

fn lookup_rtti_classname(vftable: usize) -> Option<String> {
    let vftables = VFTABLES;
    let read = vftables.read().unwrap();
    if let Some(cached) = read.get(&vftable) {
        cached.clone()
    } else {
        drop(read);

        let program = unsafe { Program::current() };
        let mut write = vftables.write().unwrap();
        let name = vftable_classname(&program, vftable);
        write.insert(vftable, name.clone());
        name
    }
}
