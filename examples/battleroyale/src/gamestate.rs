use game::cs::{CSEventFlagMan, CSNetMan, CSSessionManager, ChrIns, FieldInsHandle, LobbyState, WorldChrMan};
use util::singleton::get_instance;

#[derive(Default)]
pub struct GameStateProvider {}

impl GameStateProvider {
    /// Is anything happening related to quickmatch?
    pub fn match_active(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| n.quickmatch_manager.quickmatching_ctrl.current_state != 0)
            .unwrap_or_default()
    }

    /// Is the match currently playing out on the map?
    pub fn match_running(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| 
                n.quickmatch_manager.quickmatching_ctrl.current_state == 7
                || n.quickmatch_manager.quickmatching_ctrl.current_state == 13
            )
            .unwrap_or_default()
    }

    /// Are players currently loading into the map or awaiting the start packet from the host?
    pub fn match_loading(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| 
                n.quickmatch_manager.quickmatching_ctrl.current_state == 6
                || n.quickmatch_manager.quickmatching_ctrl.current_state == 12
            )
            .unwrap_or_default()
    }

    pub fn local_player_is_alive(&self) -> bool {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|n| n.main_player.as_ref())
            .map(|p| is_chr_ins_alive(&p.as_ref().chr_ins))
            .unwrap_or_default()
    }

    pub fn alive_players(&self) -> Vec<FieldInsHandle> {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .map(|w| {
                w.player_chr_set
                    .characters()
                    .filter_map(|p| match is_chr_ins_alive(p.as_ref()) {
                        true => Some(p.chr_ins.field_ins_handle.clone()),
                        false => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn player_steam_ids(&self) -> Vec<u64> {
        unsafe { get_instance::<CSSessionManager>() }
            .unwrap()
            .map(|s| {
                s.players.items().iter().map(|p| p.steam_id).collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    pub fn local_player(&self) -> Option<FieldInsHandle> {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|w| {
                w.main_player
                    .as_ref()
                    .map(|p| p.chr_ins.field_ins_handle.clone())
            })
    }

    pub fn last_killed_by(&self) -> Option<FieldInsHandle> {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|n| {
                n.main_player
                    .as_ref()
                    .map(|p| p.chr_ins.field_ins_handle.clone())
            })
    }

    pub fn host_steam_id(&self)  -> u64 {
        unsafe { get_instance::<CSSessionManager>() }
            .unwrap()
            .map(|s| s.host_player.steam_id)
            .unwrap_or_default()
    }

    pub fn is_host(&self)  -> bool {
        unsafe { get_instance::<CSSessionManager>() }
            .unwrap()
            .map(|s| s.lobby_state == LobbyState::HostingLobby)
            .unwrap_or_default()
    }

    pub fn event_flags_are_non_local(&self) -> bool {
        let cs_event_flag_man = unsafe { get_instance::<CSEventFlagMan>() }
            .unwrap()
            .unwrap();

        cs_event_flag_man.world_type != 0
    }

    /// Returns the chosen stage from the battleroyale context. Fixed to 0 for now.
    pub fn stage(&self) -> u32 {
        return 0;
    }
}

pub fn is_chr_ins_alive(chr_ins: &ChrIns) -> bool {
    chr_ins.module_container.data.hp > 0
}
