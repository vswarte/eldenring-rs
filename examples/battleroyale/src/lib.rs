use chr_spawner::ChrSpawner;
use config::{Configuration, ConfigurationProvider, MapId};
use context::GameModeContext;
use crash_handler::{make_crash_event, CrashContext, CrashEventResult, CrashHandler};
use gamestate::GameStateProvider;
use hooks::Hooks;
use loadout::PlayerLoadout;
use location::*;
use loot::LootGenerator;
use message::NotificationPresenter;
use network::{MatchMessaging, Message};
use pain::{PainRing, SfxSpawnLocation};
use player::Player;
use spectator_camera::SpectatorCamera;
use stage::Stage;
use std::{
    cell::RefCell, collections::HashMap, error::Error, f32::consts::PI, sync::Arc, time::Duration,
};
use steamworks_sys::{
    PersonaStateChange_t, SetPersonaNameResponse_t__bindgen_ty_1,
    SetPersonaNameResponse_t_k_iCallback, SteamNetworkingMessagesSessionFailed_t,
    SteamNetworkingMessagesSessionRequest_t, SteamNetworkingMessagesSessionRequest_t_k_iCallback,
};
use ui::Ui;

/// Implements a battle-royale gamemode on top of quickmatches.
use game::{
    cs::{
        CSHavokMan, CSNetMan, CSPhysWorld, CSTaskGroupIndex, CSTaskImp, ChrIns, ChrSet,
        FieldInsHandle, FieldInsSelector, PlayerIns, WorldChrMan,
    },
    fd4::FD4TaskData,
    matrix::FSVector4,
    position::{BlockPoint, HavokPosition},
};

use gamemode::GameMode;
use tracing_panic::panic_hook;
use util::{
    arxan,
    input::is_key_pressed,
    program::Program,
    singleton::get_instance,
    steam::{self, SteamCallback},
    task::CSTaskImpExt,
};

mod chr_spawner;
mod config;
mod context;
mod gamemode;
mod gamestate;
mod hooks;
mod loadout;
mod location;
mod loot;
mod message;
mod network;
mod pain;
mod player;
mod rva;
mod spectator_camera;
mod stage;
mod tool;
mod ui;

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

            steam::register_callback(1251, |request: &SteamNetworkingMessagesSessionRequest_t| {
                tracing::info!("Message sesson request from");
            });

            steam::register_callback(1252, |info: &SteamNetworkingMessagesSessionFailed_t| {
                tracing::error!("Message session failed");
            });

            init().expect("Could not initialize gamemode");
        });
    }

    true
}

fn init() -> Result<(), Box<dyn Error>> {
    // Stop arxan from undoing our hooks
    let program = unsafe { Program::current() };
    unsafe { arxan::disable_code_restoration(&program)? };

    let mut config = Arc::new(ConfigurationProvider::load()?);
    // config.export()?;

    let context = Arc::new(GameModeContext::default());
    let game = Arc::new(GameStateProvider::default());
    let location = Arc::new(ProgramLocationProvider::new());

    let notification = NotificationPresenter::new(location.clone());
    let player = Player::new(location.clone());

    let gamemode = Arc::new(GameMode::init(
        game.clone(),
        config.clone(),
        notification,
        player,
    ));

    let hooks = unsafe {
        Hooks::place(
            location.clone(),
            gamemode.clone(),
            context.clone(),
            game.clone(),
        )?
    };

    // Enqueue task that does it all :tm:
    let cs_task = unsafe { get_instance::<CSTaskImp>() }?.unwrap();
    let task_handle = {
        let gamemode = gamemode.clone();

        let messaging = Arc::new(MatchMessaging::default());
        let mut loadout = PlayerLoadout::new(
            config.clone(),
            game.clone(),
            context.clone(),
            messaging.clone(),
        );

        let mut loot_generator = Arc::new(LootGenerator::new(config.clone()));

        let mut chr_spawner = ChrSpawner::new(
            location.clone(),
            config.clone(),
            game.clone(),
            messaging.clone(),
        );

        let mut stage = Stage::new(
            location.clone(),
            game.clone(),
            config.clone(),
            loot_generator.clone(),
        );
        let mut pain_ring = PainRing::new(location.clone(), config.clone());
        let mut spectator_camera = SpectatorCamera::new(game.clone());
        let mut ui = Ui::new(game.clone(), location.clone());

        let mut patched_utility_effects = false;
        let mut active = false;
        let mut running = false;
        let mut sent_hellos = false;

        cs_task.run_recurring(
            move |data: &FD4TaskData| {
                // Always pull messages but only conditionally handle them
                for (remote, message) in messaging.receive_messages().iter() {
                    // if !game.match_active() {
                    //     continue;
                    // }

                    // Ignore messages not coming from the host
                    if *remote != game.host_steam_id() {
                        tracing::warn!("Received non-host message");
                        continue;
                    }

                    match message {
                        Message::Hello => {
                            tracing::info!("Received Hello");
                        }
                        Message::MatchDetails { spawn } => {
                            tracing::info!("Received match details");
                            context.set_spawn_point(spawn.clone());
                        }
                        Message::MobSpawn {
                            map,
                            pos,
                            model,
                            orientation,
                            npc_param,
                            think_param,
                            chara_init_param,
                            field_ins_handle_map_id,
                            field_ins_handle_selector,
                        } => {
                            chr_spawner.spawn_mob(
                                &FieldInsHandle {
                                    selector: FieldInsSelector(*field_ins_handle_selector),
                                    map_id: game::cs::MapId(*field_ins_handle_map_id),
                                },
                                &game::cs::MapId(*map),
                                &BlockPoint::from_xyz(pos.0, pos.1, pos.2),
                                orientation,
                                npc_param,
                                think_param,
                                chara_init_param,
                                model.as_str(),
                            );
                        }
                    }
                }

                // Unclog the steam messaging session requests
                if game.match_loading() && !sent_hellos {
                    sent_hellos = true;
                    messaging.broadcast_hello();
                }

                // Trigger logic that needs to run when the player goes into a qm lobby.
                if game.match_active() && !active {
                    tracing::info!("Starting battleroyale");
                    active = true;
                } else if !game.match_active() && active {
                    tracing::info!("Stopping battleroyale");
                    ui.reset();
                    loadout.reset();
                    pain_ring.reset();
                    spectator_camera.reset();
                    stage.reset();
                    context.reset();
                    chr_spawner.reset();
                    active = false;
                    sent_hellos = false;
                }

                // Trigger logic that needs to run when player has spawned in map.
                if game.match_in_game() && !running {
                    tracing::info!("Match started");
                    running = true;
                } else if !game.match_in_game() && running {
                    tracing::info!("Match stopped");
                    running = false;
                }

                // Gamemode creation tooling
                if is_key_pressed(0x60) {
                    tool::sample_spawn_point();
                } else if is_key_pressed(0x62) {
                    config.reload().unwrap();
                } else if is_key_pressed(0x63) {
                    let main_player = unsafe { get_instance::<WorldChrMan>() }
                        .unwrap()
                        .map(|w| w.main_player.as_ref())
                        .flatten()
                        .unwrap();

                    pain_ring.spawn_center_marker(&config::RingCenterPoint {
                        map: (&main_player.chr_ins.block_origin_override).into(),
                        position: config::MapPosition(
                            main_player.block_position.0.0,
                            main_player.block_position.0.1,
                            main_player.block_position.0.2,
                        ),
                    })
                }

                gamemode.update(data.delta_time.time);

                if game.match_active() && game.is_host() {
                    loadout.update();
                }

                if game.match_in_game() {
                    if game.is_host() {
                        chr_spawner.update();
                    }

                    // Remove utility effects like the crystal above the player.
                    if !patched_utility_effects {
                        patched_utility_effects = true;
                        let cs_net_man = unsafe { get_instance::<CSNetMan>() }.unwrap().unwrap();
                        cs_net_man.quickmatch_manager.utility_sp_effects = [0; 10];
                    }

                    ui.update();
                    pain_ring.update();
                    spectator_camera.update();
                    stage.update();
                }
            },
            CSTaskGroupIndex::GameMan,
        )
    };

    std::mem::forget(task_handle);

    Ok(())
}
