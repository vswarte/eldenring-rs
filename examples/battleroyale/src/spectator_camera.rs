use std::{
    ptr::NonNull,
    sync::{Arc, RwLock},
};

use game::cs::{FieldInsHandle, WorldChrMan, WorldChrManDbg};
use util::singleton::get_instance;

use crate::gamestate::{is_chr_ins_alive, GameStateProvider};

pub struct SpectatorCamera {
    game_state: Arc<GameStateProvider>,

    /// Player we're currently spectating.
    currently_spectating: Option<FieldInsHandle>,
}

impl SpectatorCamera {
    pub fn new(game_state: Arc<GameStateProvider>) -> Self {
        Self {
            game_state,
            currently_spectating: None,
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
                    let next_player = self.game_state.alive_players().first().cloned();
                    self.spectate(next_player);
                }

                // Update our pos to that of the player we're spectating so that the map keeps
                // loading around us.
                main_player.chr_ins.module_container.physics.position =
                    spectated_player.module_container.physics.position;
            }
            None => {
                if !self.game_state.local_player_is_alive()
                    && self.game_state.local_player_in_death_anim_loop()
                {
                    self.spectate(Some(main_player.chr_ins.last_killed_by.clone()));
                }
            }
        }
    }

    pub fn reset(&mut self) {
        if let Some(world_chr_man_dbg) = unsafe { get_instance::<WorldChrManDbg>() }.unwrap() {
            world_chr_man_dbg.cam_override_chr_ins = None;
        }

        self.currently_spectating = None;
    }

    /// Spectate a particular player by its FieldInsHandle.
    fn spectate(&mut self, handle: Option<FieldInsHandle>) {
        tracing::info!("SpectatorCamera::spectate({handle:?})");

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
