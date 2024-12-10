use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use steamworks::{
    networking_types::{NetworkingIdentity, SendFlags},
    SteamId,
};
use util::steam;

use crate::mapdata::MapPoint;

const STEAM_MESSAGE_CHANNEL: u32 = 51234;
const STEAM_MESSAGE_BATCH_SIZE: usize = 0x10;

#[derive(Default)]
pub struct MatchMessaging {}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    /// Goes from host to clients to set the local players loadout.
    Loadout {
        map_id: u32,
        position: (f32, f32, f32),
        orientation: f32,
    },
}

impl MatchMessaging {
    pub fn send_loadouts(&self, loadout: &HashMap<u64, MapPoint>) -> Result<(), Box<dyn Error>> {
        loadout.iter().for_each(|(remote, spawn)| {
            let message = Message::Loadout {
                map_id: 0x0,
                position: (
                    spawn.position.0 .0,
                    spawn.position.0 .1,
                    spawn.position.0 .2,
                ),
                orientation: spawn.orientation,
            };

            let serialized =
                bincode::serialize(&message).expect("Could not serialize spawn point message");
            self.send_raw(remote, serialized.as_slice());
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

    fn send_raw(&self, remote: &u64, data: &[u8]) {
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
