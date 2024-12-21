use std::{collections::{HashMap, VecDeque}, sync::Arc};

use game::cs::CSSessionManager;
use rand::prelude::*;
use util::singleton::get_instance;

use crate::{
    config::{ConfigurationProvider, MapConfiguration, MapPoint},
    context::GameModeContext,
    gamestate::GameStateProvider, network::MatchMessaging,
};

pub struct PlayerLoadout {
    config: Arc<ConfigurationProvider>,
    game: Arc<GameStateProvider>,
    context: Arc<GameModeContext>,
    messaging: Arc<MatchMessaging>,
    sent_loadout: bool,
    generated: bool,
    spawn_points: Vec<MapPoint>,
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
        }

        if !self.sent_loadout && self.game.match_loading() {
            self.sent_loadout = true;

            let mut loadouts = self.game.player_steam_ids()
                .into_iter()
                .enumerate()
                .map(|(index, remote)| (remote, self.spawn_point_for_player(index).clone()))
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
    pub fn spawn_point_for_player(&self, player: usize) -> &MapPoint {
        self.spawn_points
            .get(player % self.spawn_points.len())
            .expect("Tried calling spawnpoint getter without having spawn points for a map.")
    }
}
