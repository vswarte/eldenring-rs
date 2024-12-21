use std::sync::Arc;

use game::cs::CSEventFlagMan;
use util::singleton::get_instance;

use crate::{config::{ConfigurationProvider, MapConfiguration}, context::GameModeContext, gamestate::GameStateProvider, ProgramLocationProvider};

/// Handles preparing the map for the gamemode.
pub struct StagePrepare {
    location: Arc<ProgramLocationProvider>,
    game: Arc<GameStateProvider>,
    /// Targeted stage's config.
    config: Arc<ConfigurationProvider>,
    /// Have we applied the event flags for this match?
    applied_flags: bool,
}

impl StagePrepare {
    pub fn new(
        location: Arc<ProgramLocationProvider>,
        game: Arc<GameStateProvider>,
        config: Arc<ConfigurationProvider>,
    ) -> Self {
        Self {
            location,
            game,
            config,
            applied_flags: false,
        }
    }

    pub fn update(&mut self) {
        // Set the event flags once we're in the temporary flag state.
        if self.game.event_flags_are_non_local() && !self.applied_flags {
            self.applied_flags = true;
            self.apply_event_flags();
        }
    }

    pub fn reset(&mut self) {
        self.applied_flags = false;
    }

    fn apply_event_flags(&mut self) {
        let cs_event_flag_man = unsafe { get_instance::<CSEventFlagMan>() }
            .unwrap()
            .unwrap();

        self.config.map(&self.game.stage())
            .unwrap()
            .event_flag_overrides
            .iter()
            .for_each(|(flag, state)| {
                cs_event_flag_man
                    .virtual_memory_flag
                    .set_flag(*flag, *state);
            });
    }
}
