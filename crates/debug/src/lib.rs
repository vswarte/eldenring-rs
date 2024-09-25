#![feature(once_cell_get_mut)]

use std::io::Write;

use crash_handler::CrashEventResult;
use crash_handler::ExceptionCode;
use game::cs::CSTaskGroup;
use game::cs::CSTaskImp;
use game::fd4::FD4ParamRepository;
use hudhook::eject;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::imgui;
use hudhook::imgui::*;
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::Hudhook;
use hudhook::ImguiRenderLoop;

use game::cs::CSCamera;
use game::cs::CSFade;
use game::cs::CSNetMan;
use game::cs::CSSessionManager;
use game::cs::CSWorldGeomMan;
use game::cs::WorldChrMan;
use game::world_area_time::WorldAreaTime;

use display::render_debug_singleton;
use tracing_subscriber::layer::SubscriberExt;
use util::program::Program;
use util::rtti::find_rtti_classes;

use pelite::pe::Pe;

mod display;

#[no_mangle]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32) -> bool {
    match reason {
        // DLL_PROCESS_ATTACH
        1 => {
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
            let crash_handler = crash_handler::CrashHandler::attach(unsafe {
                crash_handler::make_crash_event(move |crash_context: &crash_handler::CrashContext| unsafe {
                    tracing::error!("Caught crash event");
                    tracing::error!("Process ID: {}", crash_context.process_id);
                    tracing::error!("Thread ID: {}", crash_context.thread_id);

                    let pointers = crash_context.exception_pointers;
                    let exception_record = &*(*pointers).ExceptionRecord;
                    tracing::error!("Exception Record ExceptionAddress: {:x}", exception_record.ExceptionAddress as usize);
                    tracing::error!("Exception Record ExceptionCode: {:x}", exception_record.ExceptionCode);
                    tracing::error!("Exception Record NumberParameters: {:x}", exception_record.NumberParameters);

                    for (index, entry) in exception_record.ExceptionInformation.iter().enumerate() {
                        tracing::error!("Exception Record ExceptionInformation[{}]: {:x}", index, entry);
                    }

                    CrashEventResult::Handled(false)
                })
            }).expect("failed to attach crash handler");

            // Leak it for now...
            Box::leak(Box::new(crash_handler));

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
