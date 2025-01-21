use std::{collections::HashMap, error::Error};

use game::{
    cs::{FieldInsHandle, WorldChrMan},
    position::BlockPoint,
};
use serde::{Deserialize, Serialize};
use steamworks::{
    networking_types::{NetworkingIdentity, SendFlags},
    SteamId,
};
use util::{singleton::get_instance, steam};

use crate::{
    config::{MapId, MapPosition, PlayerSpawnPoint},
    loadout::PlayerMatchDetails,
};

const STEAM_MESSAGE_CHANNEL: u32 = 51234;
const STEAM_MESSAGE_BATCH_SIZE: usize = 0x10;

#[derive(Default)]
pub struct MatchMessaging {}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    /// Sent to initiate networking session.
    Hello,
    /// Goes from host to clients to set the local players loadout.
    MatchDetails {
        spawn: PlayerSpawnPoint,
        partner: Option<u64>,
    },
    MobSpawn {
        field_ins_handle_map_id: i32,
        field_ins_handle_selector: u32,
        map: i32,
        pos: (f32, f32, f32),
        orientation: f32,
        npc_param: i32,
        think_param: i32,
        chara_init_param: i32,
        model: String,
    },
}

impl MatchMessaging {
    pub fn send_mob_spawn(
        &self,
        field_ins_handle: &FieldInsHandle,
        map: &game::cs::MapId,
        pos: &BlockPoint,
        orientation: &f32,
        npc_param: &i32,
        think_param: &i32,
        chara_init_param: &i32,
        model: &str,
    ) -> Result<(), Box<dyn Error>> {
        let serialized = bincode::serialize(&Message::MobSpawn {
            field_ins_handle_map_id: field_ins_handle.map_id.0,
            field_ins_handle_selector: field_ins_handle.selector.0,
            map: map.0,
            pos: (pos.0 .0, pos.0 .1, pos.0 .2),
            orientation: *orientation,
            npc_param: *npc_param,
            think_param: *think_param,
            chara_init_param: *chara_init_param,
            model: model.to_string(),
        })?;

        self.broadcast(serialized.as_slice())?;

        Ok(())
    }

    pub fn send_match_details(
        &self,
        loadout: &HashMap<u64, PlayerMatchDetails>,
    ) -> Result<(), Box<dyn Error>> {
        loadout.iter().for_each(|(remote, match_details)| {
            let message = Message::MatchDetails {
                spawn: match_details.spawn.clone(),
                partner: match_details.partner,
            };

            let serialized =
                bincode::serialize(&message).expect("Could not serialize spawn point message");

            self.send(remote, serialized.as_slice());
        });

        Ok(())
    }

    pub fn receive_messages(&self) -> Vec<(u64, Message)> {
        steam::client()
            .networking_messages()
            .receive_messages_on_channel(STEAM_MESSAGE_CHANNEL, STEAM_MESSAGE_BATCH_SIZE)
            .iter()
            .filter_map(|m| {
                Some((
                    m.identity_peer().steam_id()?.raw(),
                    bincode::deserialize(m.data()).ok()?,
                ))
            })
            .collect()
    }

    pub fn broadcast_hello(&self) {
        let serialized =
            bincode::serialize(&Message::Hello).expect("Could not serialize hello message");
        self.broadcast(serialized.as_slice());
    }

    /// Send message to all players in world.
    fn broadcast(&self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let world_chr_man = unsafe { get_instance::<WorldChrMan>() }.unwrap().unwrap();
        world_chr_man.player_chr_set.characters().for_each(|p| {
            if world_chr_man
                .main_player
                .as_ref()
                // Prevent us from broadcasting data to ourselves.
                .is_some_and(|m| m.chr_ins.field_ins_handle != p.chr_ins.field_ins_handle)
            {
                self.send(
                    &unsafe { p.session_manager_player_entry.as_ref() }.steam_id,
                    data,
                );
            }
        });

        Ok(())
    }

    /// Send message to a specific player
    fn send(&self, remote: &u64, data: &[u8]) {
        let result = steam::client().networking_messages().send_message_to_user(
            NetworkingIdentity::new_steam_id(Self::make_steam_id(remote.to_owned())),
            SendFlags::RELIABLE,
            data,
            STEAM_MESSAGE_CHANNEL,
        );

        match result {
            Ok(_) => {}
            Err(why) => tracing::error!("Could not send message {why}"),
        }
    }

    fn make_steam_id(raw: u64) -> SteamId {
        unsafe { std::mem::transmute(raw) }
    }
}
