use std::{
    ptr::NonNull,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use game::cs::{FieldInsHandle, WorldChrMan, WorldChrManDbg};
use util::singleton::get_instance;

use crate::gamestate::{is_chr_ins_alive, GameStateProvider};

pub const SPECTATOR_CAMERA_ACTIVATION_TIMEOUT: Duration = Duration::from_secs(4);

pub struct SpectatorCamera {
    game: Arc<GameStateProvider>,

    /// Player we're currently spectating.
    currently_spectating: Option<FieldInsHandle>,

    /// Death spectator timeout
    died_at: Option<Instant>,
}

impl SpectatorCamera {
    pub fn new(game: Arc<GameStateProvider>) -> Self {
        Self {
            game,
            currently_spectating: None,
            died_at: None,
        }
    }

    pub fn update(&mut self) {
        let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap() else {
            return;
        };

        let Some(main_player) = world_chr_man.main_player.as_mut() else {
            return;
        };

        match self.currently_spectating.clone() {
            Some(spectated) => {
                let Some(spectated_player) =
                    world_chr_man.player_chr_set.chr_ins_by_handle(&spectated)
                else {
                    return;
                };

                // Cycle to the next player if the one we're currently spectating has died.
                if !is_chr_ins_alive(spectated_player) {
                    tracing::info!("Switching spectator camera...");
                    self.spectate_next();
                    return;
                }

                // Update our pos to that of the player we're spectating so that the map keeps
                // loading around us.
                main_player.chr_ins.module_container.physics.position =
                    spectated_player.module_container.physics.position;
            }
            None => {
                // Enqueue spectator camera
                if !self.game.local_player_is_alive() && self.died_at.is_none() {
                    self.died_at = Some(Instant::now());
                }

                // Put character into spectator camera once timeout has expired
                if self
                    .died_at
                    .is_some_and(|da| da < Instant::now() - SPECTATOR_CAMERA_ACTIVATION_TIMEOUT)
                {
                    self.spectate(self.initial_spectating_player());
                }
            }
        }
    }

    /// Determines the player to spectate after dying.
    fn initial_spectating_player(&self) -> Option<FieldInsHandle> {
        self.game
            .killed_by()
            .or(self.game.alive_players().first().cloned())
    }

    fn spectate_next(&mut self) {
        self.spectate(self.game.alive_players().first().cloned());
    }

    pub fn reset(&mut self) {
        if let Some(world_chr_man_dbg) = unsafe { get_instance::<WorldChrManDbg>() }.unwrap() {
            world_chr_man_dbg.cam_override_chr_ins = None;
        }

        self.currently_spectating = None;
        self.died_at = None;
    }

    /// Spectate a particular player by its FieldInsHandle.
    fn spectate(&mut self, handle: Option<FieldInsHandle>) {
        let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap() else {
            return;
        };

        let Some(world_chr_man_dbg) = unsafe { get_instance::<WorldChrManDbg>() }.unwrap() else {
            return;
        };

        let chr_ins = handle
            .and_then(|h| world_chr_man.player_chr_set.chr_ins_by_handle(&h))
            .and_then(|c| NonNull::new(c));

        world_chr_man_dbg.cam_override_chr_ins = chr_ins;
        self.currently_spectating = chr_ins.map(|c| unsafe { c.as_ref().field_ins_handle.clone() });
    }
}
