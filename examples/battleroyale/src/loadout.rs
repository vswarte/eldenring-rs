use rand::prelude::*;

use crate::config::{MapConfiguration, MapPoint};

#[derive(Debug)]
pub struct PlayerLoadout {
    spawn_points: Vec<MapPoint>,
}

impl PlayerLoadout {
    /// Generate a loadout for a particular match configuration.
    pub fn generate(map: &MapConfiguration) -> Self {
        let mut rng = rand::thread_rng();

        // TODO: make it a bit better by spreading players over equal space?
        // Shuffle the spawn points and assign one to each player slot.
        let spawn_points = {
            let mut points = map.player_spawn_points.clone();
            points.shuffle(&mut rng);
            points
        };

        Self { spawn_points }
    }

    /// Retrieves the generated spawn point for a particular player.
    pub fn spawn_point_for_player(&self, player: usize) -> &MapPoint {
        self.spawn_points
            .get(player % self.spawn_points.len())
            .expect("Tried calling spawnpoint getter without having spawn points for a map.")
    }
}
