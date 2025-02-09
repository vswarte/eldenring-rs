use std::sync::RwLock;

use crate::config::PlayerSpawnPoint;

#[derive(Default)]
pub struct GameModeContext {
    inner: RwLock<GameModeContextInner>,
}

#[derive(Default)]
pub struct GameModeContextInner {
    stage: u32,
    spawn_point: Option<PlayerSpawnPoint>,
}

impl GameModeContext {
    pub fn reset(&self) {
        let mut guard = self.inner.write().unwrap();
        guard.stage = 0;
        guard.spawn_point = None;
    }

    pub fn set_stage(&self, stage: u32) {
        tracing::info!("Setting stage {stage}");
        self.inner.write().unwrap().stage = stage;
    }

    /// Local player's spawn point
    pub fn spawn_point(&self) -> Option<PlayerSpawnPoint> {
        self.inner.read().unwrap().spawn_point.clone()
    }

    pub fn set_spawn_point(&self, point: PlayerSpawnPoint) {
        self.inner.write().unwrap().spawn_point = Some(point);
    }
}
