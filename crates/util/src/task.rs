use std::{
    cell::UnsafeCell,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::program::Program;
use game::cs::{CSTaskGroupIndex, CSTaskImp, FD4TaskData};
use pelite::pe::Pe;
use pelite::{pattern, pattern::Atom};
use std::sync::LazyLock;

const REGISTER_TASK_PATTERN: &[Atom] =
    pattern!("e8 ? ? ? ? 48 8b 0d ? ? ? ? 4c 8b c7 8b d3 e8 $ { ' }");

const REGISTER_TASK_VA: LazyLock<u64> = LazyLock::new(|| {
    let program = unsafe { Program::current() };
    let mut matches = [0u32; 2];

    if !program
        .scanner()
        .finds_code(REGISTER_TASK_PATTERN, &mut matches)
    {
        panic!("Could not find REGISTER_TASK_PATTERN or found duplicates.");
    }

    program
        .rva_to_va(matches[1])
        .expect("Call target for REGISTER_TASK_PATTERN was not in exe")
});

pub trait CSTaskImpExt {
    fn run_task<T: Into<FD4Task>>(&self, execute: T, group: CSTaskGroupIndex) -> TaskHandle;
}

impl CSTaskImpExt for CSTaskImp {
    fn run_task<T: Into<FD4Task>>(&self, task: T, group: CSTaskGroupIndex) -> TaskHandle {
        tracing::debug!("Registering task to task group. group = {group:?}");
        let register_task: extern "C" fn(&CSTaskImp, CSTaskGroupIndex, &FD4Task) =
            unsafe { std::mem::transmute(*REGISTER_TASK_VA) };

        let task: Arc<FD4Task> = Arc::new(task.into());
        // SAFETY: we hold a unique reference to the contents of `arc`
        unsafe {
            *task.self_ref.get() = Some(task.clone());
        }

        register_task(self, group, task.as_ref());

        TaskHandle { _task: task }
    }
}

pub struct TaskHandle {
    _task: Arc<FD4Task>,
}

impl Drop for TaskHandle {
    fn drop(&mut self) {
        todo!("Call the actual unregister fn from the game");
        self._task.unregister()
    }
}

// 'static required to generate the vmt
// Sized required to make sure &T is not fat
trait FD4TaskVMT: 'static + Sized {
    // TODO: Generate this using a macro_rules taking the virtual functions
    fn vmt() -> *const extern "C" fn() -> () {
        // This must be used as opposed to an untyped tuple to guarantee the ordering
        // Could be generated outside for use in other code too
        #[repr(C)]
        struct Layout<T>(
            extern "C" fn(&T) -> *mut (),
            extern "C" fn(&mut T),
            extern "C" fn(&mut T, &FD4TaskData),
        );

        // Using &'static with type inference without a `static` or `const` keyword, because
        // otherwise `Self` would have to be mentionned which is not allowed.
        // Checked with gotbolt to be sure and it does generate it statically at all opt levels.
        let vmt: &'static _ = &Layout(Self::get_runtime_class, Self::destructor, Self::execute);
        vmt as *const _ as *const extern "C" fn() -> ()
    }

    extern "C" fn get_runtime_class(&self) -> *mut () {
        unimplemented!()
    }
    extern "C" fn destructor(&mut self) {
        unimplemented!()
    }

    extern "C" fn execute(&mut self, data: &FD4TaskData);
}

#[repr(C)]
pub struct FD4Task {
    vftable: *const extern "C" fn() -> (),
    unk8: usize,
    closure: Box<dyn FnMut(&FD4TaskData)>,
    unregister_requested: AtomicBool,
    self_ref: UnsafeCell<Option<Arc<Self>>>,
}

impl FD4TaskVMT for FD4Task {
    extern "C" fn execute(&mut self, data: &FD4TaskData) {
        // Should we stop before run?
        if !self.unregister_requested.load(Ordering::Relaxed) {
            (self.closure)(data);
        }

        // Drop if we got cancelled during run.
        if self.unregister_requested.load(Ordering::Relaxed) {
            self.self_ref.get_mut().take();
        }
    }
}

impl FD4Task {
    pub fn new<F: FnMut(&FD4TaskData) + 'static + Send>(closure: F) -> Self {
        Self {
            vftable: Self::vmt(),
            unk8: 0,
            closure: Box::new(closure),
            unregister_requested: AtomicBool::new(false),
            self_ref: UnsafeCell::new(None),
        }
    }

    fn unregister(&self) {
        self.unregister_requested.store(true, Ordering::Relaxed);
    }
}

impl<F: FnMut(&FD4TaskData) + 'static + Send> From<F> for FD4Task {
    fn from(value: F) -> Self {
        Self::new(value)
    }
}

impl Drop for FD4Task {
    fn drop(&mut self) {
        self.unregister_requested.store(true, Ordering::Relaxed);
    }
}
