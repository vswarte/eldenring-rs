use std::pin::Pin;

use game::cs::{CSEzTask, CSEzTaskVMT, CSTaskGroupIndex};

pub struct FD4TaskHandle {
    vftable: Pin<Box<CSEzTaskVMT>>,
    task: Pin<Box<CSEzTask>>,
}

impl Drop for FD4TaskHandle {
    fn drop(&mut self) {
        todo!("Caught task dropping its vftable. Unregistering a task is currently not implemented.")
    }
}

pub fn run_task(execute_fn: fn(), task_group: CSTaskGroupIndex) -> FD4TaskHandle {
    let vftable = Box::pin(CSEzTaskVMT {
        get_runtime_class: || tracing::error!("TEST_TASK::get_runtime_class called!"),
        execute: |_| tracing::error!("TEST_TASK::execute called!"),
        eztask_execute: execute_fn,
        register_task: || tracing::error!("TEST_TASK::register_task called"),
        free_task: || tracing::error!("TEST_TASK::free_task called"),
        get_task_group: || tracing::error!("TEST_TASK::get_task_group called"),
    });

    let task = Box::pin(CSEzTask {
        vftable: vftable.as_ref().get_ref() as *const CSEzTaskVMT,
        task_proxy: 0,
        unk8: 0,
        _padc: 0,
    });

    tracing::debug!("Registering task to task group. task_group = {task_group:?}");
    let register_task: extern "C" fn(&CSEzTask, CSTaskGroupIndex) =
        unsafe { std::mem::transmute(0x140eb1650_usize) };

    register_task(&task, task_group);

    FD4TaskHandle { vftable, task }
}
