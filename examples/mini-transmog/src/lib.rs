use std::collections::HashMap;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use std::{error::Error, thread::spawn};
use steamworks::networking_types::{NetworkingIdentity, SendFlags};
use steamworks::{LobbyId, SteamId};
use tracing_panic::panic_hook;

use game::cs::{
    CSTaskGroupIndex, CSTaskImp, FD4TaskData, WorldChrMan, CHR_ASM_SLOT_PROTECTOR_HEAD,
    CHR_ASM_SLOT_PROTECTOR_LEGS,
};
use util::steam::{self, networking_messages, register_callback, SteamCallbackImpl};
use util::{singleton::get_instance, task::CSTaskImpExt};

const STEAM_MESSAGE_CHANNEL: u32 = 123;

#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason == 1 {
        std::panic::set_hook(Box::new(panic_hook));

        let appender = tracing_appender::rolling::never("./", "mini-transmog.log");
        tracing_subscriber::fmt().with_writer(appender).init();

        spawn::<_, _>(|| {
            // Give the CRT init a bit of leeway
            sleep(Duration::from_secs(5));

            init().expect("Could not initialize mod");
        });
    }

    true
}

fn init() -> Result<(), Box<dyn Error>> {
    let task = unsafe { get_instance::<CSTaskImp>() }.unwrap().unwrap();
    let protector_mapping = Arc::new(ProtectorOverrideHolder {
        local_overrides: HashMap::from([
            (640000, 900000),
            (640100, 900100),
            (640200, 900200),
            (640300, 900300),
        ]),
        remote_overrides: Default::default(),
    });

    // Patch the ChrAsm for every character after backreads happen.
    let chr_asm_patch_task = {
        let protector_mapping = protector_mapping.clone();

        task.run_task(
            move |_: &FD4TaskData| {
                let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap() else {
                    return;
                };

                // Apply the main players overrides
                if let Some(player) = world_chr_man.main_player.as_mut() {
                    player.chr_asm.equipment_param_ids
                        [CHR_ASM_SLOT_PROTECTOR_HEAD..CHR_ASM_SLOT_PROTECTOR_LEGS]
                        .iter_mut()
                        .filter_map(|equipped| {
                            protector_mapping
                                .local_overrides
                                .get(equipped)
                                .map(|substitute| (equipped, substitute))
                        })
                        .for_each(|(equipped, subtitute)| *equipped = *subtitute);
                }
            },
            CSTaskGroupIndex::WorldChrMan_Update_BackreadRequestPost,
        )
    };
    std::mem::forget(chr_asm_patch_task);

    let model_param_modifier_task = task.run_task(
        |_: &FD4TaskData| {
            let Some(player) = unsafe { get_instance::<WorldChrMan>() }
                .unwrap()
                .and_then(|w| w.main_player.as_ref())
            else {
                return;
            };

            unsafe {
                player
                    .chr_ins
                    .module_container
                    .model_param_modifier
                    .modifiers
                    .iter()
            }
            // .filter(|modifier| {
            //     unsafe { modifier.name.to_string() }
            //         .unwrap()
            //         .contains("Phantom")
            // })
            .for_each(|modifier| {
                modifier.unk28.value1 = 1.0;
                modifier.unk28.value2 = 1.0;
                modifier.unk28.value3 = 1.0;
                modifier.unk28.value4 = 1.0;

                modifier.unk40.value1 = 1.0;
                modifier.unk40.value2 = 1.0;
                modifier.unk40.value3 = 1.0;
                modifier.unk40.value4 = 1.0;

                modifier.unk58.value1 = 1.0;
                modifier.unk58.value2 = 1.0;
                modifier.unk58.value3 = 1.0;
                modifier.unk58.value4 = 1.0;

                modifier.unk88.value1 = 1.0;
                modifier.unk88.value2 = 1.0;
                modifier.unk88.value3 = 1.0;
                modifier.unk88.value4 = 1.0;

                modifier.unka0.value1 = 1.0;
                modifier.unka0.value2 = 1.0;
                modifier.unka0.value3 = 1.0;
                modifier.unka0.value4 = 1.0;
            });
        },
        CSTaskGroupIndex::Draw_Pre,
    );
    std::mem::forget(model_param_modifier_task);

    // WHY
    std::mem::forget(register_callback::<SyncMappingLobbyUpdateCallback>());
    // std::mem::forget(register_callback::<SyncMessageRequestCallback>());

    // Retrieve updates to our character table from the p2p.
    let networking_task = task.run_task(
        |_: &FD4TaskData| {
            for message in steam::client()
                .networking_messages()
                .receive_messages_on_channel(STEAM_MESSAGE_CHANNEL, 0x5)
                .into_iter()
            {
                let mapping = bincode::deserialize::<'_, HashMap<i32, i32>>(message.data());
                tracing::info!("Received mapping: {mapping:#?}");
            }
        },
        CSTaskGroupIndex::SteamThread0,
    );
    std::mem::forget(networking_task);

    Ok(())
}

#[derive(Default)]
struct ProtectorOverrideHolder {
    remote_overrides: HashMap<NetworkingIdentity, HashMap<i32, i32>>,
    local_overrides: HashMap<i32, i32>,
}

pub fn send_mapping(remote: SteamId, mapping: &HashMap<i32, i32>) -> Result<(), Box<dyn Error>> {
    let serialized = bincode::serialize(mapping)?;

    steam::client().networking_messages().send_message_to_user(
        NetworkingIdentity::new_steam_id(remote),
        SendFlags::RELIABLE,
        serialized.as_slice(),
        STEAM_MESSAGE_CHANNEL,
    )?;

    Ok(())
}

struct SyncMappingLobbyUpdateCallback;
impl SteamCallbackImpl for SyncMappingLobbyUpdateCallback {
    type TData = steamworks_sys::LobbyDataUpdate_t;
    const CALLBACK: i32 = steamworks_sys::LobbyDataUpdate_t_k_iCallback as _;

    fn run(data: *const Self::TData) {
        let data = unsafe { data.as_ref() }.unwrap();
        tracing::info!("Got lobby update: {:?}", data);

        let mapping = HashMap::from([
            (640000, 900000),
            (640100, 900100),
            (640200, 900200),
            (640300, 900300),
        ]);

        for member in steam::client()
            .matchmaking()
            .lobby_members(LobbyId::from_raw(data.m_ulSteamIDLobby))
        {
            tracing::info!("Sending mapping to {member:?}");

            let result = send_mapping(member, &mapping);
            tracing::info!("Sent mapping: {:?} -> {:?}", data, result);
        }
    }
}

struct SyncMessageRequestCallback;
impl SteamCallbackImpl for SyncMessageRequestCallback {
    type TData = steamworks_sys::SteamNetworkingMessagesSessionRequest_t;
    const CALLBACK: i32 = steamworks_sys::SteamNetworkingMessagesSessionRequest_t_k_iCallback as _;

    fn run(data: *const Self::TData) {
        let data = unsafe { data.as_ref() }.unwrap();
        if !unsafe {
            steamworks_sys::SteamAPI_ISteamNetworkingMessages_AcceptSessionWithUser(
                networking_messages().unwrap(),
                &data.m_identityRemote as *const _,
            )
        } {
            tracing::error!("Could not accept session");
        }
    }
}
