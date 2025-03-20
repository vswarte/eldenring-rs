use dlrf::DLRFSingleton;
use hudhook::imgui::{TreeNodeFlags, Ui};

pub(crate) mod area_time;
pub(crate) mod camera;
pub(crate) mod chr;
pub(crate) mod dlio;
pub(crate) mod event_flag;
pub(crate) mod fade;
pub(crate) mod field_area;
pub(crate) mod geometry;
pub(crate) mod net_man;
pub(crate) mod param;
pub(crate) mod session_manager;
pub(crate) mod shared;
pub(crate) mod task;
pub(crate) mod world_chr_man;

pub trait DebugDisplay {
    fn render_debug(&self, ui: &&mut Ui);
}

pub fn render_debug_singleton<T: DLRFSingleton + DebugDisplay + 'static>(ui: &&mut Ui) {
    let singleton = unsafe { util::singleton::get_instance::<T>() }
        .unwrap_or_else(|_| panic!("Could not get reflection data for {}", T::DLRF_NAME));

    match singleton {
        Some(instance) => {
            if ui.collapsing_header(T::DLRF_NAME, TreeNodeFlags::empty()) {
                ui.indent();
                let pointer = instance as *const T;
                let mut pointer_string = format!("{:#x?}", pointer);
                let label = format!("{} instance", T::DLRF_NAME);
                ui.input_text(label.as_str(), &mut pointer_string)
                    .read_only(true)
                    .build();

                instance.render_debug(ui);
                ui.unindent();
                ui.separator();
            }
        }
        None => ui.text(format!("No instance of {} found", T::DLRF_NAME)),
    }
}
