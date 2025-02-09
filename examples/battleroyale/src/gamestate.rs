use game::cs::{
    CSEventFlagMan, CSNetMan, CSQuickMatchingCtrlState, CSSessionManager, ChrIns, FieldInsHandle,
    LobbyState, MapId, WorldChrMan,
};
use util::singleton::get_instance;

#[derive(Default)]
pub struct GameStateProvider {}

const ONE_30TH_SECOND: f32 = 1.0 / 30.0;
const ONE_15TH_SECOND: f32 = 1.0 / 15.0;

impl GameStateProvider {
    /// Is quickmatch happening in any capacity?
    pub fn match_active(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| {
                n.quickmatch_manager.quickmatching_ctrl.current_state
                    != CSQuickMatchingCtrlState::None
            })
            .unwrap_or_default()
    }

    /// Host is accepting new people and clients are waiting for lobby to fill up.
    pub fn match_onboarding(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| {
                n.quickmatch_manager.quickmatching_ctrl.current_state
                    == CSQuickMatchingCtrlState::HostInvite
                    || n.quickmatch_manager.quickmatching_ctrl.current_state
                        == CSQuickMatchingCtrlState::GuestInviteWait
            })
            .unwrap_or_default()
    }

    /// Is the match currently playing out on the map?
    pub fn match_in_game(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| {
                n.quickmatch_manager.quickmatching_ctrl.current_state
                    == CSQuickMatchingCtrlState::GuestInGame
                    || n.quickmatch_manager.quickmatching_ctrl.current_state
                        == CSQuickMatchingCtrlState::HostInGame
            })
            .unwrap_or_default()
    }

    /// Are players currently loading into the map or awaiting the start packet from the host?
    pub fn match_loading(&self) -> bool {
        unsafe { get_instance::<CSNetMan>() }
            .unwrap()
            .map(|n| {
                n.quickmatch_manager.quickmatching_ctrl.current_state
                    == CSQuickMatchingCtrlState::GuestMoveMap
                    || n.quickmatch_manager.quickmatching_ctrl.current_state
                        == CSQuickMatchingCtrlState::HostMoveMap
            })
            .unwrap_or_default()
    }

    pub fn local_player_is_alive(&self) -> bool {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|n| n.main_player.as_ref())
            .map(|p| is_chr_ins_alive(&p.as_ref().chr_ins))
            .unwrap_or_default()
    }

    pub fn local_player_in_death_anim_loop(&self) -> bool {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|m| m.main_player.as_ref())
            .and_then(|p| {
                p.chr_ins
                    .module_container
                    .as_ref()
                    .time_act
                    .as_ref()
                    .anim_queue
                    .first()
            })
            .map(|a| {
                a.play_time == a.anim_length
                    && (a.play_time == ONE_30TH_SECOND || a.play_time == ONE_15TH_SECOND)
            })
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
                s.players
                    .items()
                    .iter()
                    .map(|p| p.steam_id)
                    .collect::<Vec<_>>()
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

    pub fn killed_by(&self) -> Option<FieldInsHandle> {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|n| {
                n.main_player
                    .as_ref()
                    .map(|p| p.chr_ins.field_ins_handle.clone())
            })
    }

    pub fn host_steam_id(&self) -> u64 {
        unsafe { get_instance::<CSSessionManager>() }
            .unwrap()
            .map(|s| s.host_player.steam_id)
            .unwrap_or_default()
    }

    pub fn is_host(&self) -> bool {
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

    /// Is the match's result definitive?
    pub fn match_concluded(&self) -> bool {
        self.alive_players().len() == 1
    }

    /// Is the local player the winner of the match?
    pub fn is_winner(&self) -> bool {
        self.match_concluded() && self.local_player_is_alive()
    }

    pub fn is_in_roundtable(&self) -> bool {
        unsafe { get_instance::<WorldChrMan>() }
            .unwrap()
            .and_then(|n| {
                n.main_player
                    .as_ref()
                    .map(|p| p.chr_ins.map_id_1 == MapId::from_parts(11, 10, 0, 0))
            })
            .unwrap_or_default()
    }
}

pub fn is_chr_ins_alive(chr_ins: &ChrIns) -> bool {
    chr_ins.module_container.data.hp > 0
}
