#![feature(once_cell_get_mut)]

use std::io::Write;

use game::fd4::FD4ParamRepository;
use hudhook::eject;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::imgui;
use hudhook::imgui::*;
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::Hudhook;
use hudhook::ImguiRenderLoop;

use game::cs::CSTaskImp;
use game::cs::CSTaskGroup;
use game::cs::CSCamera;
use game::cs::CSFade;
use game::cs::CSNetMan;
use game::cs::CSSessionManager;
use game::cs::CSWorldGeomMan;
use game::cs::WorldChrMan;
use game::world_area_time::WorldAreaTime;

use display::render_debug_singleton;
use tracing_panic::panic_hook;
use tracing_subscriber::layer::SubscriberExt;
use util::program::Program;
use util::rtti::find_rtti_classes;

use pelite::pe::Pe;
use util::singleton::get_instance;

mod display;

#[no_mangle]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32) -> bool {
    match reason {
        // DLL_PROCESS_ATTACH
        1 => {
            std::panic::set_hook(Box::new(panic_hook));

            let appender = tracing_appender::rolling::never("./", "chains-bindings.log");
            tracing_subscriber::fmt().with_writer(appender).init();

            let program = unsafe { Program::current() };
            let test = find_rtti_classes(&program)
                .find(|c| c.name.as_str() == "CS::ChrIns");

            for class in find_rtti_classes(&program) {
                let vmt = program.rva_to_va(class.vtable).unwrap();

                tracing::trace!(
                    "Discovered RTTI class. name = {}, vmt = {:x}",
                    &class.name,
                    vmt,
                );
            }

            tracing::info!("Inited tracing");

            std::thread::spawn(move || {
                if let Err(e) = Hudhook::builder()
                    .with::<ImguiDx12Hooks>(EldenRingDebugGui::new())
                    .with_hmodule(hmodule)
                    .build()
                    .apply()
                {
                    tracing::error!("Couldn't apply hooks: {e:?}");
                    eject();
                }
            });
        }
        _ => {},
    }

    true
}

struct EldenRingDebugGui;

impl EldenRingDebugGui {
    fn new() -> Self {
        Self {}
    }
}

impl ImguiRenderLoop for EldenRingDebugGui {
    fn render(&mut self, ui: &mut Ui) {
        ui.window("Elden Ring Rust Bindings Debug")
            .position([0., 0.], imgui::Condition::FirstUseEver)
            .size([800., 600.], imgui::Condition::FirstUseEver)
            .build(|| {
                render_debug_singleton::<FD4ParamRepository>(&ui);
                render_debug_singleton::<WorldChrMan>(&ui);
                render_debug_singleton::<CSWorldGeomMan>(&ui);
                render_debug_singleton::<CSCamera>(&ui);
                render_debug_singleton::<CSFade>(&ui);
                render_debug_singleton::<WorldAreaTime>(&ui);
                render_debug_singleton::<CSSessionManager>(&ui);
                render_debug_singleton::<CSNetMan>(&ui);
                render_debug_singleton::<CSTaskGroup>(&ui);
                render_debug_singleton::<CSTaskImp>(&ui);
            });
    }
}
