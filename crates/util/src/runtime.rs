#[repr(C)]
struct CSEzTaskVftable {
    pub get_runtime_class: fn(),
    pub execute: fn(),
    pub eztask_execute: fn(),
    pub register_task: fn(),
    pub free_task: fn(),
    pub get_task_group: fn(),
}

#[repr(C)]
struct CSEzTask<'a> {
    pub vftable: &'a CSEzTaskVftable,
    pub unk8: u32,
    _padc: u32,
    pub task_proxy: usize,
}
