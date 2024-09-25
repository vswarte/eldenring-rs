use hudhook::imgui::{TreeNodeFlags, Ui};
use game::DLRFLocatable;

pub(crate) mod shared;
pub(crate) mod cs_session_manager;
pub(crate) mod world_area_time;
pub(crate) mod cs_world_geom_man;
pub(crate) mod cs_camera;
pub(crate) mod cs_fade;
pub(crate) mod chr_ins;
pub(crate) mod world_chr_man;
pub(crate) mod cs_net_man;
pub(crate) mod cs_task;
pub(crate) mod param;

pub trait DebugDisplay {
    fn render_debug(&self, ui: &&mut Ui);
}

pub fn render_debug_singleton<T: DLRFLocatable + DebugDisplay + 'static>(ui: &&mut Ui) {
    let singleton = util::singleton::get_instance::<T>()
        .unwrap_or_else(|_| panic!("Could not get reflection data for {}", T::DLRF_NAME));

    match singleton {
        Some(instance) => if ui.collapsing_header(T::DLRF_NAME, TreeNodeFlags::empty()) {
            let pointer = instance as *const T;
            let mut pointer_string = format!("{:#x?}", pointer);
            let label = format!("{} instance", T::DLRF_NAME);
            ui.input_text(label.as_str(), &mut pointer_string).read_only(true).build();

            instance.render_debug(ui);
            ui.separator();
        },
        None => ui.text(format!("No instance of {} found", T::DLRF_NAME)),
    }
}
