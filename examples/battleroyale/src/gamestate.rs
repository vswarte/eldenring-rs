use game::cs::{CSEventFlagMan, CSNetMan, CSSessionManager, ChrIns, FieldInsHandle, LobbyState, PlayerIns, WorldChrMan};
use util::singleton::get_instance;

pub trait GameStateProvider {
    /// Get local player handle.
    fn local_player(&self) -> Option<FieldInsHandle>;

    /// Are we currently in a quickmatch lobby?
    fn in_quickmatch(&self) -> bool;

    /// Is the match currently in the map and playing out.
    fn match_active(&self) -> bool;

    /// Is the match currently loading into the map.
    fn match_loading(&self) -> bool;

    /// Is the local player alive? Returns false if the character doesn't exist.
    fn local_player_is_alive(&self) -> bool;

    /// Returns an iterator of all alive players.
    fn alive_players(&self) -> Vec<FieldInsHandle>;

    /// Returns an iterator of all alive players.
    fn player_steam_ids(&self) -> Vec<u64>;

    /// Returns the ChrIns the player was last killed by.
    fn last_killed_by(&self) -> Option<FieldInsHandle>;

    /// Returns the session hosts steam ID.
    fn host_steam_id(&self) -> u64;

    /// Returns whether or not we're hosting the current match.
    fn is_host(&self) -> bool;

    /// Returns whether or not we're hosting the current match.
    fn event_flags_are_non_local(&self) -> bool;
}

#[derive(Default)]
pub struct DefaultGameStateProvider {}

impl GameStateProvider for DefaultGameStateProvider {
    fn in_quickmatch(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| n.quickmatch_manager.quickmatching_ctrl.current_state != 0)
            .unwrap_or_default()
    }

    fn match_active(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| 
                n.quickmatch_manager.quickmatching_ctrl.current_state == 7
                || n.quickmatch_manager.quickmatching_ctrl.current_state == 13
            )
            .unwrap_or_default()
    }

    fn match_loading(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| 
                n.quickmatch_manager.quickmatching_ctrl.current_state == 6
                || n.quickmatch_manager.quickmatching_ctrl.current_state == 12
            )
            .unwrap_or_default()
    }

    fn local_player_is_alive(&self) -> bool {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .map(|n| n.main_player.as_ref())
            .flatten()
            .map(|p| is_chr_ins_alive(&p.as_ref().chr_ins))
            .unwrap_or_default()
    }

    fn alive_players(&self) -> Vec<FieldInsHandle> {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .map(|w| {
                w.player_chr_set
                    .characters()
                    .filter_map(|p| match is_chr_ins_alive(&p.as_ref()) {
                        true => Some(p.chr_ins.field_ins_handle.clone()),
                        false => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn player_steam_ids(&self) -> Vec<u64> {
        unsafe { get_instance::<CSSessionManager>() }
            .unwrap()
            .map(|s| {
                s.players.items().iter().map(|p| p.steam_id).collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    fn local_player(&self) -> Option<FieldInsHandle> {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .map(|w| {
                w.main_player
                    .as_ref()
                    .map(|p| p.chr_ins.field_ins_handle.clone())
            })
            .flatten()
    }

    fn last_killed_by(&self) -> Option<FieldInsHandle> {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .map(|n| {
                n.main_player
                    .as_ref()
                    .map(|p| p.chr_ins.field_ins_handle.clone())
            })
            .flatten()
    }

    fn host_steam_id(&self)  -> u64 {
        unsafe { get_instance::<CSSessionManager>() }
            .unwrap()
            .map(|s| s.host_player.steam_id)
            .unwrap_or_default()
    }

    fn is_host(&self)  -> bool {
        unsafe { get_instance::<CSSessionManager>() }
            .unwrap()
            .map(|s| s.lobby_state == LobbyState::HostingLobby)
            .unwrap_or_default()
    }

    fn event_flags_are_non_local(&self) -> bool {
        let cs_event_flag_man = unsafe { get_instance::<CSEventFlagMan>() }
            .unwrap()
            .unwrap();

        cs_event_flag_man.world_type != 0
    }
}

fn is_chr_ins_alive(chr_ins: &ChrIns) -> bool {
    chr_ins.module_container.data.hp > 0
}
