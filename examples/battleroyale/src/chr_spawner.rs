use std::{
    error::Error,
    ptr::NonNull,
    sync::Arc,
    time::{Duration, Instant},
};

use game::{
    cs::{ChrIns, ChrSet, NetChrSync, P2PEntityHandle, WorldChrMan},
    matrix::FSVector4,
    position::HavokPosition,
};
use serde::{Deserialize, Serialize};
use util::singleton::get_instance;

use crate::{
    config::{ConfigurationProvider, MapPoint},
    gamestate::GameStateProvider,
    network::MatchMessaging,
    rva::{RVA_CHR_FLAGS_UNK1, RVA_CHR_FLAGS_UNK2, RVA_NET_CHR_SYNC_SETUP_ENTITY_HANDLE, RVA_SPAWN_CHR},
    ProgramLocationProvider,
};

const CHR_SPAWN_INTERVAL: Duration = Duration::from_secs(60);

pub struct ChrSpawner {
    location: Arc<ProgramLocationProvider>,
    config: Arc<ConfigurationProvider>,
    game: Arc<GameStateProvider>,
    networking: Arc<MatchMessaging>,
    last_spawned_chr: Instant,
}

impl ChrSpawner {
    pub fn new(
        location: Arc<ProgramLocationProvider>,
        config: Arc<ConfigurationProvider>,
        game: Arc<GameStateProvider>,
        networking: Arc<MatchMessaging>,
    ) -> Self {
        Self {
            location,
            config,
            game,
            networking,
            last_spawned_chr: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        if self.game.is_host() && self.game.match_in_game() {
            if self.last_spawned_chr + CHR_SPAWN_INTERVAL < Instant::now() {
                self.last_spawned_chr = Instant::now();
            }
        }
    }

    pub fn spawn_mob(&mut self, model: &str) -> Result<(), Box<dyn Error>> {
        tracing::info!("Spawning character");

        let world_chr_man = unsafe { get_instance::<WorldChrMan>() }.unwrap().unwrap();
        let Some(main_player) = &world_chr_man.main_player else {
            return Ok(());
        };

        // Convert map coordinates to physics pos
        let physics_pos = main_player
            .chr_ins
            .module_container
            .physics
            .position
            .clone();

        let mut request = Box::leak(Box::new(ChrSpawnRequest {
            position: physics_pos,
            orientation: FSVector4(0.0, 3.5, 0.0, 0.0),
            scale: FSVector4(1.0, 1.0, 1.0, 1.0),
            unk30: FSVector4(1.0, 1.0, 1.0, 1.0),
            npc_param: 31000000,
            npc_think_param: 31000000,
            chara_init_param: -1,
            event_entity_id: 0,
            talk_id: 0,
            unk54: -1.828282595,

            unk58: 0x142A425A0,
            asset_name_str_ptr: 0, // Filled in after the fact
            unk68: 5,
            unk6c: 0,
            unk70: 0,
            unk74: 0x00010002,
            asset_name: Default::default(),
            unk98: 0x140BDE74D,
            unka0: 16,
            unka4: 0,
            /// World pos?
            unka8: 29.46097755,
            unkac: 1,
            unkb0: 441.519043,
            unkb4: 1,
            unkb8: 47.78706741,
            unkbc: 1,
            unkc0: 0,
            unkc4: 0,
        }));

        // Set string pointers as god intended
        let model_bytes = model.encode_utf16().collect::<Vec<u16>>();
        request.asset_name[0..5].clone_from_slice(model_bytes.as_slice());
        request.asset_name_str_ptr = request.asset_name.as_ptr() as usize;

        let spawn_chr: extern "C" fn(
            &ChrSet<ChrIns>,
            u8,
            &ChrSpawnRequest,
            u32,
        ) -> Option<NonNull<ChrIns>> =
            unsafe { std::mem::transmute(self.location.get(RVA_SPAWN_CHR).unwrap()) };

        let setup_chrsync: extern "C" fn(&NetChrSync, &P2PEntityHandle) -> Option<NonNull<ChrIns>> = unsafe {
            std::mem::transmute(
                self.location
                    .get(RVA_NET_CHR_SYNC_SETUP_ENTITY_HANDLE)
                    .unwrap(),
            )
        };

        let chr_flags_unk1: extern "C" fn(&ChrIns, bool) = unsafe {
            std::mem::transmute(
                self.location
                    .get(RVA_CHR_FLAGS_UNK1)
                    .unwrap(),
            )
        };

        let chr_flags_unk2: extern "C" fn(&ChrIns) = unsafe {
            std::mem::transmute(
                self.location
                    .get(RVA_CHR_FLAGS_UNK2)
                    .unwrap(),
            )
        };

        let buddy_slot = world_chr_man
            .summon_buddy_manager
            .as_ref()
            .unwrap()
            .next_buddy_slot;

        let mut chr_ins = spawn_chr(&world_chr_man.summon_buddy_chr_set, 0, request, buddy_slot)
            .expect("Could not spawn chr");

        let p2phandle = &unsafe { chr_ins.as_ref() }.p2p_entity_handle;
        setup_chrsync(world_chr_man.net_chr_sync.as_ref(), p2phandle);

        unsafe { chr_ins.as_mut() }.net_chr_sync_flags.set_unk2(true);

        // chr_flags_unk1(unsafe { chr_ins.as_ref() }, true);
        // chr_flags_unk2(unsafe { chr_ins.as_ref() });

        if self.game.is_host() {
            // Notify others of the spawn
            self.networking.send_mob_spawn(model)?;
        }

        Ok(())
    }

    /// Runs cleanup at end of match.
    pub fn reset(&mut self) {}
}

#[repr(C)]
pub struct ChrSpawnRequest {
    pub position: HavokPosition,
    pub orientation: FSVector4,
    pub scale: FSVector4,
    pub unk30: FSVector4,
    pub npc_param: i32,        // 31000000
    pub npc_think_param: i32,  // 31000000
    pub chara_init_param: i32, // -1
    pub event_entity_id: u32,  // 0
    pub talk_id: u32,          // 0
    unk54: f32,                // -1.828282595

    // Cursed ass dlinplace str meme
    unk58: usize,              // 142A425A0
    asset_name_str_ptr: usize, // 13FFF0278
    unk68: u32,                // 5
    unk6c: u32,                // 0
    unk70: u32,                // 0
    unk74: u32,                // 0x00010002
    asset_name: [u16; 0x10],   // c3100
    unk98: usize,              // 140BDE74D
    unka0: u32,                // 16
    unka4: u32,                // 0
    unka8: f32,                // 29.46097755
    unkac: u32,                // 1
    unkb0: f32,                // 441.519043
    unkb4: u32,                // 1
    unkb8: f32,                // 47.78706741
    unkbc: u32,                // 1
    unkc0: u32,                // 0
    unkc4: u32,                // 0
}
