
use display::DebugDisplay;
use hudhook::eject;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::imgui;
use hudhook::imgui::*;
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::Hudhook;
use hudhook::ImguiRenderLoop;

use game::cs::CSCamera;
use game::cs::CSEventFlagMan;
use game::cs::CSFade;
use game::cs::CSNetMan;
use game::cs::CSSessionManager;
use game::cs::CSTaskGroup;
use game::cs::CSTaskImp;
use game::cs::CSWorldGeomMan;
use game::cs::MsbRepository;
use game::cs::WorldAreaTime;
use game::cs::WorldChrMan;
use game::dlio::DLFileDeviceManager;
use game::fd4::FD4ParamRepository;

use display::render_debug_singleton;
use tracing_panic::panic_hook;

mod display;

#[no_mangle]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32) -> bool {
    if reason == 1 {
        std::panic::set_hook(Box::new(panic_hook));

        let appender = tracing_appender::rolling::never("./", "chains-debug.log");
        tracing_subscriber::fmt().with_writer(appender).init();

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
            .size([600., 400.], imgui::Condition::FirstUseEver)
            .build(|| {
                let tabs = ui.tab_bar("main-tabs").unwrap();
                if let Some(item) = ui.tab_item("World") {
                    render_debug_singleton::<CSEventFlagMan>(&ui);
                    render_debug_singleton::<WorldChrMan>(&ui);
                    render_debug_singleton::<CSWorldGeomMan>(&ui);
                    render_debug_singleton::<WorldAreaTime>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Networking") {
                    render_debug_singleton::<CSSessionManager>(&ui);
                    render_debug_singleton::<CSNetMan>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Resource") {
                    if ui.collapsing_header("DLFileDeviceManager", TreeNodeFlags::empty()) {
                        let file_device_manager =
                            unsafe { &*(0x1448464c0usize as *mut DLFileDeviceManager) };

                        file_device_manager.render_debug(&ui);
                    }

                    render_debug_singleton::<CSTaskGroup>(&ui);
                    render_debug_singleton::<CSTaskImp>(&ui);
                    render_debug_singleton::<FD4ParamRepository>(&ui);
                    render_debug_singleton::<MsbRepository>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Render") {
                    render_debug_singleton::<CSCamera>(&ui);
                    render_debug_singleton::<CSFade>(&ui);
                    item.end();
                }
                tabs.end();
            });
    }
}
