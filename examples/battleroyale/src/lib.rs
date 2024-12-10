use pain::PainRing;
use crash_handler::{make_crash_event, CrashContext, CrashEvent, CrashEventResult, CrashHandler};
use gamestate::DefaultGameStateProvider;
use hooks::GamemodeHooks;
use location::*;
use loot::LootGenerator;
use message::NotificationPresenter;
use pelite::pe::{Pe, Rva, Va};
use player::Player;
use spectator_camera::SpectatorCamera;
use std::{
    error::Error,
    sync::{Arc, RwLock},
    time::Duration,
};

/// Implements a battle-royale gamemode on top of quickmatches.
use game::{
    cs::{
        CSNetMan, CSTaskGroupIndex, CSTaskImp, ChrIns, MapId, PlayerIns, QuickmatchManager,
        WorldChrMan,
    },
    fd4::FD4TaskData,
};

use gamemode::GameMode;
use retour::static_detour;
use tracing::level_filters::LevelFilter;
use tracing_panic::panic_hook;
use util::{
    arxan,
    program::Program,
    singleton::get_instance,
    task::{CSTaskImpExt, RecurringTaskHandle},
};

mod gamemode;
mod hooks;
mod location;
mod mapdata;
mod spectator_camera;
mod gamestate;
mod loadout;
mod network;
mod message;
mod loot;
mod tool;
mod pain;
mod player;

#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    // Check if we're being attached anew
    if reason == 1 {
        // Set up some logging so we can catch crashes and such
        let appender = tracing_appender::rolling::never("./", "battleroyale.log");
        tracing_subscriber::fmt().with_writer(appender).init();
        std::panic::set_hook(Box::new(panic_hook));

        let handler = CrashHandler::attach(unsafe {
            make_crash_event(move |context: &CrashContext| {
                tracing::error!(
                    "Exception: {:x} at {:x}",
                    context.exception_code,
                    (*(*context.exception_pointers).ExceptionRecord).ExceptionAddress as usize
                );

                CrashEventResult::Handled(false)
            })
        })
        .unwrap();
        std::mem::forget(handler);

        std::thread::spawn(|| {
            // Give the CRT init a bit of leeway
            std::thread::sleep(Duration::from_secs(5));
            init().expect("Could not initialize gamemode");
        });
    }

    true
}

fn init() -> Result<(), Box<dyn Error>> {
    // Stop arxan from undoing our hooks
    let program = unsafe { Program::current() };
    unsafe { arxan::disable_code_restoration(&program)? };

    let game_state = Arc::new(DefaultGameStateProvider::default());
    let location = Arc::new(HardcodedLocationProvider::new());

    let spectator_camera = SpectatorCamera::new(game_state.clone());
    let loot_generator = LootGenerator::new(location.clone());
    let notification = NotificationPresenter::new(location.clone());
    let player = Player::new(location.clone());
    let pain_ring = PainRing::new(location.clone());

    let gamemode = Arc::new(GameMode::init(
        game_state,
        location.clone(),
        notification,
        spectator_camera,
        loot_generator,
        player,
        pain_ring,
    ));

    let hooks = unsafe {
        GamemodeHooks::<DefaultGameStateProvider, _>::place(
            location,
            gamemode.clone(),
        )?
    };

    // Enqueue task that updates gamemode
    let cs_task = unsafe { get_instance::<CSTaskImp>() }?.unwrap();
    let task_handle = {
        let gamemode = gamemode.clone();

        cs_task.run_recurring(
            move |_: &FD4TaskData| gamemode.update(),
            CSTaskGroupIndex::GameMan,
        )
    };

    std::mem::forget(task_handle);

    Ok(())
}
