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
use ui::Ui;
use std::{
    cell::RefCell, collections::HashMap, error::Error, f32::consts::PI, sync::Arc, time::Duration,
};
use steamworks_sys::{
    PersonaStateChange_t, SetPersonaNameResponse_t__bindgen_ty_1,
    SetPersonaNameResponse_t_k_iCallback,
};

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

// 523357 - Fia's Mist
// 523573 - Darkness clouds
// 523887 - Freezing Mist

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

            steam::register_callback(0x15b, |update: &PersonaStateChange_t| {
                tracing::info!("Persona state change {}", update.m_ulSteamID);
            });

            let friends = steamworks_sys::SteamAPI_SteamFriends_v017();
            steamworks_sys::SteamAPI_ISteamFriends_RequestUserInformation(
                friends,
                76561197997653528,
                false,
            );

            // let cb = SteamCallback::<
            //     0x15b,
            //     PersonaStateChange_t,
            // >::from(|update: &PersonaStateChange_t| {
            //     tracing::info!("Persona state change {}", update.m_ulSteamID);
            // });

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
                }

                // if is_key_pressed(0x60) {
                //     let world_chr_man = unsafe { get_instance::<WorldChrMan>() }.unwrap().unwrap();
                //     if let Some(main_player) = &world_chr_man.main_player {
                //         let physics_pos = main_player
                //             .chr_ins
                //             .module_container
                //             .physics
                //             .position
                //             .clone();
                //
                //         let cast_ray: extern "C" fn(
                //             *const CSPhysWorld,
                //             u32,
                //             *const FSVector4,
                //             *const FSVector4,
                //             *const FSVector4,
                //             *const PlayerIns,
                //         ) -> bool = unsafe {
                //             std::mem::transmute(location.get(LOCATION_PHYS_WORLD_CAST_RAY).unwrap())
                //         };
                //
                //         let phys_world = unsafe { get_instance::<CSHavokMan>() }
                //             .unwrap()
                //             .unwrap()
                //             .phys_world
                //             .as_ptr();
                //
                //         let player = main_player.as_ptr();
                //
                //         let radius = 50.0;
                //         let count = 128;
                //         for i in 0..count {
                //             let current = ((PI * 2.0) / count as f32) * i as f32;
                //             let point_x = f32::sin(current) * radius;
                //             let point_z = f32::cos(current) * radius;
                //
                //             let (ox, oy, oz) = physics_pos.xyz();
                //             let origin = FSVector4(ox + point_x, oy + 100.0, oz + point_z, 0.0);
                //             let direction = FSVector4(0.0, -200.0, 0.0, 0.0);
                //             let mut collision = FSVector4(0.0, 0.0, 0.0, 0.0);
                //
                //             tracing::info!("Phys World: {phys_world:#x?}");
                //             tracing::info!("Player: {player:#x?}");
                //             tracing::info!("Origin: {origin:#?}");
                //             tracing::info!("Direction: {direction:#?}");
                //
                //             if cast_ray(
                //                 phys_world,
                //                 0x2000058, // Broadphase filter
                //                 &origin, // Where we shoot the cast from
                //                 &direction, // Direction to shoot cast into
                //                 &mut collision, // Output
                //                 player, // Owner of the ray
                //             ) {
                //                 tracing::info!("Collision: {collision:#?}");
                //
                //                 // Angle the sfx we're about to spawn
                //                 let angle = (
                //                     FSVector4(0.7882865667, -0.007318737917, 0.6165360808, 0.0),
                //                     FSVector4(0.06933222711, 0.9946286082, -0.07685082406, 0.0),
                //                     FSVector4(-0.6126625538, 0.1033189669, 0.784560442, 0.0),
                //                 );
                //
                //                 let spawn_sfx: fn(&u32, &SfxSpawnLocation) -> bool =
                //                     unsafe { std::mem::transmute(location.get(LOCATION_SFX_SPAWN).unwrap()) };
                //
                //                 // Place sfx at collision
                //                 let (x, y, z) = (
                //                     collision.0,
                //                     collision.1,
                //                     collision.2,
                //                 );
                //                 let spawn_location = SfxSpawnLocation {
                //                     angle,
                //                     position: HavokPosition::from_xyz(x, y, z),
                //                 };
                //
                //                 spawn_sfx(&523887, &spawn_location);
                //             }
                //         }
                //     }
                // }

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
