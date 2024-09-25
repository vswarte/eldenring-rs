#![feature(once_cell_get_mut)]

use std::io::Write;
use std::marker::PhantomData;
use std::pin::Pin;
use std::time::Duration;

use broadsword::dll;

use crash_handler::CrashEventResult;
use game::cs::CSTaskGroup;
use game::cs::CSTaskGroupIndex;
use game::cs::CSTaskImp;
use game::fd4::FD4Time;
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
use util::task::run_task;

use pelite::pe::Pe;

mod display;

#[dll::entrypoint]
pub fn entry(hmodule: usize) -> bool {
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
        crash_handler::make_crash_event(move |crash_context: &crash_handler::CrashContext| {
            tracing::info!("Handling crash event");

            let mut file = std::fs::File::create("crash.txt").unwrap();
            write!(file, "Crash event");
            write!(file, "Process ID: {}", crash_context.process_id);
            write!(file, "Thread ID: {}", crash_context.thread_id);

            CrashEventResult::Handled(false)
        })
    }).expect("failed to attach crash handler");

    std::thread::spawn(move || {
        if let Err(e) = Hudhook::builder()
            .with::<ImguiDx12Hooks>(EldenRingDebugGui::new())
            .with_hmodule(HINSTANCE(hmodule as isize))
            .build()
            .apply()
        {
            tracing::error!("Couldn't apply hooks: {e:?}");
            eject();
        }
    });

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
