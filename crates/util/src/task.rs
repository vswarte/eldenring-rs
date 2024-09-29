use std::pin::Pin;

use game::cs::{CSTaskGroupIndex, CSTaskImp, FD4TaskBase, FD4TaskBaseVMT, FD4TaskData};

pub type TaskExecuteFn = fn(*const FD4TaskBase, *const FD4TaskData);

pub trait TaskRuntime {
    fn run_task(&self, execute: TaskExecuteFn, group: CSTaskGroupIndex) -> TaskHandle;
}

impl TaskRuntime for CSTaskImp<'_> {
    fn run_task(&self, execute: TaskExecuteFn, group: CSTaskGroupIndex) -> TaskHandle {
        let vmt = Box::pin(FD4TaskBaseVMT {
            get_runtime_class: |_| unimplemented!(),
            destructor: |_| unimplemented!(),
            execute,
        });

        let task = Box::pin(FD4TaskBase {
            vftable: vmt.as_ref().get_ref() as *const _,
        });

        tracing::info!("Registering task to task group. group = {group:?}");
        let register_task: extern "C" fn(&CSTaskImp, CSTaskGroupIndex, &FD4TaskBase) =
            unsafe { std::mem::transmute(0x140eb1fd0usize) };

        register_task(self, group, task.as_ref().get_ref());

        TaskHandle {
            vmt,
            task,
            group,
        }
    }
}

pub struct TaskHandle {
    vmt: Pin<Box<FD4TaskBaseVMT>>,
    task: Pin<Box<FD4TaskBase>>,
    group: CSTaskGroupIndex,
}

impl Drop for TaskHandle {
    fn drop(&mut self) {
        todo!()
    }
}
