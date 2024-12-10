use std::{
    ptr::NonNull,
    sync::{Arc, RwLock},
};

use game::cs::{FieldInsHandle, WorldChrMan, WorldChrManDbg};
use util::singleton::get_instance;

use crate::gamestate::GameStateProvider;

pub struct SpectatorCamera<S>
where
    S: GameStateProvider,
{
    game_state: Arc<S>,

    /// Player we're currently spectating.
    currently_spectating: RwLock<Option<FieldInsHandle>>,
}

impl<S> SpectatorCamera<S>
where
    S: GameStateProvider,
{
    pub fn new(game_state: Arc<S>) -> Self {
        Self {
            game_state,
            currently_spectating: RwLock::new(None),
        }
    }

    pub fn update(&self) {
        let Some(currently_spectating) = self.currently_spectating.read().unwrap().clone() else {
            return;
        };

        // Check if the player we're spectating has died.
        let alive_players = self.game_state.alive_players();
        if !alive_players.contains(&currently_spectating) {
            tracing::info!("Switching spectator camera...");
            self.spectate(alive_players.first().cloned());
        }

        // Update our pos to that of the player we're spectating so that the map keeps loading
        let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap() else {
            return;
        };

        let Some(main_player) = world_chr_man.main_player.as_mut() else {
            return;
        };

        let Some(spectating_player) = world_chr_man
            .player_chr_set
            .chr_ins_by_handle(&currently_spectating)
        else {
            return;
        };

        main_player.chr_ins.module_container.physics.position = spectating_player.module_container.physics.position;
    }

    /// Stop spectating.
    pub fn stop(&self) {
        if let Some(world_chr_man_dbg) = unsafe { get_instance::<WorldChrManDbg>() }.unwrap() {
            world_chr_man_dbg.cam_override_chr_ins = None;
        }

        let _ = self.currently_spectating.write().unwrap().take();
    }

    /// Spectate a particular player by its FieldInsHandle.
    pub fn spectate(&self, handle: Option<FieldInsHandle>) {
        let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap() else {
            return;
        };

        let Some(world_chr_man_dbg) = unsafe { get_instance::<WorldChrManDbg>() }.unwrap() else {
            return;
        };

        // Lookup character and grab the FieldInsHandle from the discovered character, to ensure
        // the FieldInsHandle is valid.
        let chr_ins = handle
            .and_then(|h| world_chr_man.player_chr_set.chr_ins_by_handle(&h))
            .and_then(|c| NonNull::new(c));

        world_chr_man_dbg.cam_override_chr_ins = chr_ins;
        *self.currently_spectating.write().unwrap() =
            chr_ins.map(|c| unsafe { c.as_ref().field_ins_handle.clone() });
    }
}
