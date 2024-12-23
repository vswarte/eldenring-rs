use std::sync::RwLock;

use crate::config::MapPoint;

#[derive(Default)]
pub struct GameModeContext {
    inner: RwLock<GameModeContextInner>,
}

#[derive(Default)]
pub struct GameModeContextInner {
    stage: u32,
    spawn_point: Option<MapPoint>,
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
    pub fn spawn_point(&self) -> Option<MapPoint> {
        self.inner.read().unwrap().spawn_point.clone()
    }

    pub fn set_spawn_point(&self, point: MapPoint) {
        tracing::info!("Setting spawn point {point:#?}");
        self.inner.write().unwrap().spawn_point = Some(point);
    }
}
