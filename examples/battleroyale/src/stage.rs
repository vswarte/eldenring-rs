use std::sync::Arc;

use crate::{config::MapConfiguration, ProgramLocationProvider};

/// Handles preparing the map for the gamemode.
pub struct StagePrepare {
    location: Arc<ProgramLocationProvider>,
    /// Targeted stage's config.
    config: Option<Arc<MapConfiguration>>,
}

impl StagePrepare {
    pub fn new(location: Arc<ProgramLocationProvider>) -> Self {
        Self { location, config: None }
    }

    /// Target a particular stage for prepping the map.
    pub fn target_stage(&mut self, config: Arc<MapConfiguration>) {
        self.config = Some(config);
    }

    pub fn reset(&mut self) {
        self.config = None;
    }

    pub fn apply_event_flags(&self) {
        todo!();
    }
}
