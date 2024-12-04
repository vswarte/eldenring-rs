use std::{
    collections::HashMap,
    ptr::NonNull,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};

use game::{
    cs::{
        CSEventFlagMan, CSNetMan, CSSessionManager, ChrIns, FieldInsHandle, MapId, WorldChrMan,
        WorldChrManDbg,
    },
    fd4::FD4Time,
    position::BlockPoint,
};
use rand::prelude::*;
use util::{input::is_key_pressed, singleton::get_instance};

use crate::{
    gamestate::GameStateProvider,
    loot::LootGenerator,
    message,
    network::{MatchMessaging, Message},
    tool,
};
use crate::{
    loadout::PlayerLoadout,
    mapdata::{SpawnPoint, MAP_CONFIG},
};
use crate::{
    message::MatchResultPresenter, spectator_camera::SpectatorCamera, HardcodedLocationProvider,
};

#[derive(Clone, Debug, PartialEq)]
pub enum PlayerState {
    Participating,
    Spectating(FieldInsHandle),
}

/// Fornite emote allotment.
pub const END_DISCONNECT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct GameMode<S>
where
    S: GameStateProvider,
{
    /// Is gamemode active at this time?
    _running: AtomicBool,
    /// Is gamemode active still but the match finished?
    _finished: AtomicBool,
    /// As a host, did send the loadout to the participants?
    sent_loadout: AtomicBool,
    /// Applied flag overrides for this match?
    applied_flag_overrides: AtomicBool,
    /// Handles player spectation.
    spectator_camera: SpectatorCamera<S>,
    /// Handles loot generation
    loot_generator: LootGenerator<HardcodedLocationProvider>,
    /// Provides info about the games state like if we're in a match, score, alive players.
    game_state: Arc<S>,
    /// Used to generate and keep track of player spawns.
    player_loadout: RwLock<Option<PlayerLoadout>>,
    /// Player spawn point
    spawn_point: RwLock<Option<SpawnPoint>>,
    /// Facilitates networking for the match.
    messaging: MatchMessaging,
    /// Timer to keep track of when a match end was requested.
    end_requested_at: RwLock<Option<Instant>>,
    /// Presents match scores at the end.
    match_result_presenter: MatchResultPresenter<HardcodedLocationProvider>,
}

impl<S> GameMode<S>
where
    S: GameStateProvider,
{
    /// Initializes the gamemodes
    pub fn init(
        game_state: S,
        match_result_presenter: MatchResultPresenter<HardcodedLocationProvider>,
    ) -> Self {
        let game_state = Arc::new(game_state);

        Self {
            _running: Default::default(),
            _finished: Default::default(),
            sent_loadout: Default::default(),
            applied_flag_overrides: Default::default(),
            spectator_camera: SpectatorCamera::new(game_state.clone()),
            loot_generator: LootGenerator::new(HardcodedLocationProvider::new()),
            game_state: game_state.clone(),
            player_loadout: Default::default(),
            spawn_point: Default::default(),
            messaging: Default::default(),
            end_requested_at: Default::default(),
            match_result_presenter,
        }
    }

    /// Updates the gamemode state, spectator camera, etc...
    pub fn update(&self) {
        let game_state = self.game_state.clone();

        if is_key_pressed(0x60) {
            tool::sample_spawn_point();
        }

        // Update gamemode state
        {
            let in_quickmatch = game_state.in_quickmatch();
            let gamemode_running = self.running();
            if in_quickmatch && !gamemode_running {
                self.start();
            } else if !in_quickmatch && gamemode_running {
                self.stop();
            }
        }

        // Pull all networking messages and handle them
        for (remote, message) in self.messaging.receive_messages().iter() {
            // Ignore messages not coming from the host
            if *remote != game_state.host_steam_id() {
                tracing::warn!("Received non-host message");
                continue;
            }

            match message {
                Message::Loadout {
                    map_id,
                    position,
                    orientation,
                } => {
                    *self.spawn_point.write().unwrap() = Some(SpawnPoint {
                        map: MapId::from_parts(20, 0, 0, 0),
                        position: BlockPoint::from_xyz(position.0, position.1, position.2),
                        orientation: orientation.clone(),
                    })
                }
            }
        }

        // Send loadout if we're the host and in a loading state.
        if game_state.match_loading()
            && game_state.is_host()
            && !self.sent_loadout.swap(true, Ordering::Relaxed)
        {
            tracing::info!("Sending loadout to other players..");

            // Remove host steam ID so we dont send the loadout for that.
            let guard = self.player_loadout.read().unwrap();
            let loadout = guard.as_ref().unwrap();

            // Create mapping between steam ID and loadout properties.
            let mut loadouts = game_state
                .player_steam_ids()
                .into_iter()
                .enumerate()
                .map(|(index, remote)| (remote, loadout.spawn_point_for_player(index).clone()))
                .collect::<HashMap<_, _>>();

            // Remove host spawn point since we're the host and we need to apply it locally
            let cs_session_manager = unsafe { get_instance::<CSSessionManager>() }
                .unwrap()
                .unwrap();

            let host = cs_session_manager.host_player.steam_id;
            let host_spawn = loadouts
                .remove(&host)
                .expect("Could not place host character (you)");
            *self.spawn_point.write().unwrap() = Some(host_spawn);

            tracing::info!("Loadouts: {loadouts:?}");

            self.messaging
                .send_loadouts(&loadouts)
                .expect("Could not send player loadouts");
        }

        if !game_state.match_active() {
            return;
        }

        // Apply event flags if we're in the networked world state with our event flag man.
        if game_state.event_flags_are_non_local()
            && !self.applied_flag_overrides.swap(true, Ordering::Relaxed)
        {
            tracing::info!("Applying world flag overrides to temp flag blocks");

            // TODO: refactor to general structure that changes world + map state.
            let cs_event_flag_man = unsafe { get_instance::<CSEventFlagMan>() }
                .unwrap()
                .unwrap();

            let map = &MAP_CONFIG[0];
            map.event_flag_overrides.iter().for_each(|(flag, state)| {
                cs_event_flag_man
                    .virtual_memory_flag
                    .set_flag(*flag, *state);
            });
        }

        if self.should_request_end_match() {
            tracing::info!("Requesting match end");
            self.request_end_match();
        }

        if self.should_end_match() {
            tracing::info!("Ending match");
            self.end_match();
        }

        if self.game_state.is_host() {
            self.loot_generator.update();
        }

        self.spectator_camera.update();
    }

    /// Returns whether or not the custom gamemode is running.
    pub fn running(&self) -> bool {
        self._running.load(Ordering::Relaxed)
    }

    /// Puts the player into spectate mode.
    pub fn spectate(&self) {
        tracing::info!("Entering spectate mode");
    }

    /// Returns if the match was finished.
    fn finished(&self) -> bool {
        self._finished.load(Ordering::Relaxed)
    }

    /// Should request the session to end.
    fn should_request_end_match(&self) -> bool {
        return false;
        match self.end_requested_at.read().unwrap().as_ref() {
            Some(_) => false,
            None => self.game_state.alive_players().len() == 1,
        }
    }

    /// Request that a match is ended.
    fn request_end_match(&self) {
        /// Display the results
        let message = if self.game_state.local_player_is_alive() {
            message::Message::Victory
        } else {
            message::Message::Defeat
        };

        self.match_result_presenter.present(message);

        *self.end_requested_at.write().unwrap() = Some(Instant::now());
    }

    /// Should terminate the session.
    fn should_end_match(&self) -> bool {
        match self.end_requested_at.read().unwrap().as_ref() {
            Some(e) => *e + END_DISCONNECT_TIMEOUT < Instant::now(),
            None => false,
        }
    }

    /// Finishes the match and closes it.
    fn end_match(&self) {
        // Disconnect the ugly way for now
        let cs_net_man = unsafe { get_instance::<CSNetMan>() }.unwrap().unwrap();
        cs_net_man
            .quickmatch_manager
            .battle_royal_context
            .quickmatch_context
            .error_state = 13;

        self._finished.swap(true, Ordering::Relaxed);
    }

    /// Starts the gamemode.
    fn start(&self) {
        tracing::info!("Starting gamemode");

        self._running.swap(true, Ordering::Relaxed);
        self._finished.swap(false, Ordering::Relaxed);
        self.sent_loadout.swap(false, Ordering::Relaxed);
    }

    /// Stops the gamemode.
    fn stop(&self) {
        tracing::info!("Stopping gamemode");
        let _ = self.end_requested_at.write().unwrap().take();
        self.spectator_camera.stop();
        self.loot_generator.reset();
        self.applied_flag_overrides.store(false, Ordering::Relaxed);
        self._running.store(false, Ordering::Relaxed);
    }

    /// Sets up the gamemode for a particular map.
    pub fn target_map(&self, map: u32) -> MapId {
        tracing::info!("Requested target map ID for {map}");

        // TODO: match config against incoming map enum
        let targeted_map = &MAP_CONFIG[0];

        // Generate loadout on every end.
        let loadout = PlayerLoadout::generate(targeted_map);
        tracing::info!("Generated loadout: {loadout:#?}");
        *self.player_loadout.write().unwrap() = Some(loadout);

        // TODO: this needs reeavaluation if we ever want to spawn players across multiple blocks.
        targeted_map
            .player_spawn_points
            .get(0)
            .expect("Map has no spawn points...")
            .map
            .clone()
    }

    /// Get local players assigned spawn-point for the match.
    pub fn player_spawn_point(&self) -> SpawnPoint {
        // Place player at default location if no spawn point was networked by now...
        let default = MAP_CONFIG
            .first()
            .unwrap()
            .player_spawn_points
            .first()
            .unwrap()
            .clone();
        self.spawn_point
            .read()
            .unwrap()
            .as_ref()
            .unwrap_or(&default)
            .clone()
    }

    /// Processes a characters death.
    pub fn handle_death(&self, handle: &FieldInsHandle) {
        tracing::info!("ChrIns died: {}", handle);

        // Local player has died
        if self.game_state.local_player().is_some_and(|h| &h == handle) {
            tracing::info!("Local player died, putting in spectate mode");
            // Turn on the spectator camera
            // TODO: define behavior when killed-by player is no longer available (very
            // unlikely).
            self.spectator_camera
                .spectate(self.game_state.last_killed_by())
        }
    }
}
