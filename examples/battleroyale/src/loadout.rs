use std::{collections::HashMap, sync::Arc};

use game::cs::CSSessionManager;
use rand::prelude::*;
use util::singleton::get_instance;

use crate::{
    config::{ConfigurationProvider, MapConfiguration, PlayerSpawnPoint},
    context::GameModeContext,
    gamestate::GameStateProvider,
    network::MatchMessaging,
};

pub struct PlayerMatchDetails {
    pub spawn: PlayerSpawnPoint,
    pub party: Vec<u64>,
}

pub struct PlayerLoadout {
    config: Arc<ConfigurationProvider>,
    game: Arc<GameStateProvider>,
    context: Arc<GameModeContext>,
    messaging: Arc<MatchMessaging>,
    sent_loadout: bool,
    generated: bool,
    spawn_points: Vec<PlayerSpawnPoint>,
    party_size: u32,
    parties: HashMap<u64, Vec<u64>>,
}

impl PlayerLoadout {
    pub fn new(
        config: Arc<ConfigurationProvider>,
        game: Arc<GameStateProvider>,
        context: Arc<GameModeContext>,
        messaging: Arc<MatchMessaging>,
    ) -> Self {
        Self {
            config,
            game,
            context,
            messaging,
            sent_loadout: false,
            generated: false,
            spawn_points: vec![],
            party_size: 2,
            parties: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        if !self.generated {
            self.generated = true;
            tracing::info!("Generating loadout for session");

            // TODO: make it a bit better by spreading players over equal space?
            let map = self.config.map(&self.game.stage()).unwrap();
            let mut rng = rand::thread_rng();
            let spawn_points = {
                let mut points = map.player_spawn_points.clone();
                points.shuffle(&mut rng);
                points
            };

            // For host we'll need to assign it locally
            tracing::info!("Self-assigning spawn point");
            self.context
                .set_spawn_point(spawn_points.last().cloned().unwrap());

            self.spawn_points = spawn_points;

            tracing::info!("Generated player pairs");

            // Generate player parties
            let mut players = self.game.player_steam_ids();
            players.shuffle(&mut rng);

            for chunk in players.chunks(self.party_size as usize) {
                let party: Vec<u64> = chunk.to_vec();
                for &member in chunk {
                    self.parties.insert(member, party.clone());
                }
            }
            tracing::info!("Generated player parties");
            tracing::debug!("Parties: {:?}", self.parties);
        }

        if !self.sent_loadout && self.game.match_loading() {
            self.sent_loadout = true;

            let loadouts = self
                .game
                .player_steam_ids()
                .into_iter()
                .enumerate()
                .map(|(index, remote)| {
                    (
                        remote,
                        PlayerMatchDetails {
                            spawn: self.spawn_point_for_player(index).clone(),
                            party: self.parties.get(&remote).unwrap().clone(),
                        },
                    )
                })
                .collect::<HashMap<_, _>>();

            // Remove host spawn point since we're the host and we need to apply it locally
            let cs_session_manager = unsafe { get_instance::<CSSessionManager>() }
                .unwrap()
                .unwrap();

            self.messaging.send_match_details(&loadouts).unwrap();

            tracing::info!("Sent loadouts");
        }
    }

    pub fn reset(&mut self) {
        self.spawn_points = vec![];
        self.generated = false;
        self.sent_loadout = false;
    }

    /// Retrieves the generated spawn point for a particular player.
    pub fn spawn_point_for_player(&self, player: usize) -> &PlayerSpawnPoint {
        self.spawn_points
            .get(player % self.spawn_points.len())
            .expect("Tried calling spawnpoint getter without having spawn points for a map.")
    }
}
