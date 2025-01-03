use std::{
    error::Error,
    ptr::NonNull,
    sync::Arc,
    time::{Duration, Instant},
};

use game::{
    cs::{
        ChrIns, ChrSet, FieldArea, FieldInsHandle, FieldInsSelector, MapId, NetChrSync,
        P2PEntityHandle, WorldChrMan,
    },
    matrix::FSVector4,
    position::{BlockPoint, HavokPosition},
    rva::RVA_GLOBAL_FIELD_AREA,
};
use pelite::pe::Pe;
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};
use serde::{Deserialize, Serialize};
use util::{program::Program, singleton::get_instance, team_relation::TEAM_TYPE_ENEMY};

use crate::{
    config::{ConfigurationProvider, MonsterType}, gamestate::GameStateProvider, network::MatchMessaging, rva::{
        RVA_CHR_FLAGS_UNK1, RVA_CHR_FLAGS_UNK2, RVA_NET_CHR_SYNC_SETUP_ENTITY_1,
        RVA_NET_CHR_SYNC_SETUP_ENTITY_2, RVA_NET_CHR_SYNC_SETUP_ENTITY_3, RVA_SPAWN_CHR,
    }, team::ENEMY_TEAM_TYPE, ProgramLocationProvider
};

const CHR_SPAWN_INDEX_START: u32 = 20;
const CHR_SPAWN_INTERVAL: Duration = Duration::from_secs(60);

pub struct ChrSpawner {
    location: Arc<ProgramLocationProvider>,
    config: Arc<ConfigurationProvider>,
    game: Arc<GameStateProvider>,
    networking: Arc<MatchMessaging>,
    last_spawned_chr: Instant,

    spawned_initial_monsters: bool,

    /// Keeps track of next chr selector field for FielsInsHandle generation.
    next_chr_index: u32,
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
            next_chr_index: CHR_SPAWN_INDEX_START,
            spawned_initial_monsters: false,
        }
    }

    pub fn update(&mut self) {
        if self.game.is_host() && self.game.match_in_game() && !self.spawned_initial_monsters {
            self.spawned_initial_monsters = true;
            self.spawn_initial_monsters();
        }
    }

    fn spawn_initial_monsters(&mut self) {
        tracing::info!("Spawning initial monsters");
        let map = self.config.map(&self.game.stage()).unwrap();

        map.monster_spawn_points.iter().for_each(|s| {
            let field_ins_handle = self.generate_field_ins_handle();
            let m = self.pick_mob(s.pool);

            self.spawn_mob(
                &field_ins_handle,
                &game::cs::MapId(s.map.0),
                &BlockPoint::from_xyz(s.position.0, s.position.1, s.position.2),
                &s.orientation,
                &m.npc_id,
                &m.think_id,
                &-1,
                m.asset.as_str(),
            );
        });

        // map.bespoke_monster_spawns.iter().for_each(|m| {
        //     let field_ins_handle = self.generate_field_ins_handle();
        //     self.spawn_mob(
        //         &field_ins_handle,
        //         &game::cs::MapId(m.map.0),
        //         &BlockPoint::from_xyz(m.position.0, m.position.1, m.position.2),
        //         &m.orientation,
        //         &m.npc_id,
        //         &m.think_id,
        //         &-1,
        //         m.asset.as_str(),
        //     );
        // });
    }

    /// Generate field ins handle for to-be spawned character.
    pub fn generate_field_ins_handle(&mut self) -> FieldInsHandle {
        let field_ins_handle = FieldInsHandle {
            selector: FieldInsSelector::from_parts(1, 113, self.next_chr_index),
            map_id: MapId::none(),
        };
        self.next_chr_index += 1;
        field_ins_handle
    }

    /// Select random mob from pool for spawning.
    pub fn pick_mob(&self, pool: u32) -> MonsterType {
        let map = self.config.map(&self.game.stage()).unwrap();
        let monster_pool = map
            .monster_types
            .iter()
            .filter(|l| l.pool == pool)
            .collect::<Vec<_>>();
        let weights = monster_pool.iter().map(|f| f.weight).collect::<Vec<u32>>();
        let distribution = WeightedIndex::new(weights.as_slice()).unwrap();
        let mut rng = thread_rng();
        monster_pool[distribution.sample(&mut rng)].clone()
    }

    pub fn spawn_mob(
        &mut self,
        field_ins_handle: &FieldInsHandle,
        map: &MapId,
        pos: &BlockPoint,
        orientation: &f32,
        npc_param: &i32,
        think_param: &i32,
        chara_init_param: &i32,
        model: &str,
    ) -> Result<(), Box<dyn Error>> {
        let world_chr_man = unsafe { get_instance::<WorldChrMan>() }.unwrap().unwrap();

        if world_chr_man.main_player.is_none() {
            return Ok(());
        }

        let program = unsafe { Program::current() };
        let field_area = unsafe {
            (*(program.rva_to_va(RVA_GLOBAL_FIELD_AREA).unwrap() as *const *const FieldArea))
                .as_ref()
        }
        .unwrap();

        let Some(center) = field_area
            .world_info_owner
            .world_res
            .world_info
            .world_block_info_by_map(map)
            .map(|b| b.physics_center)
        else {
            tracing::error!("Could not find WorldBlockInfo for map ID {map}");
            return Ok(());
        };

        let spawn_physics_pos = HavokPosition::from_xyz(
            pos.0 .0 + center.0 .0,
            pos.0 .1 + center.0 .1,
            pos.0 .2 + center.0 .2,
        );

        let mut request = Box::leak(Box::new(ChrSpawnRequest {
            position: spawn_physics_pos,
            orientation: FSVector4(0.0, *orientation, 0.0, 0.0),
            scale: FSVector4(1.0, 1.0, 1.0, 1.0),
            unk30: FSVector4(1.0, 1.0, 1.0, 1.0),
            npc_param: *npc_param,
            npc_think_param: *think_param,
            chara_init_param: *chara_init_param,
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
            unka8: 0x141EBB015,
            unkb0: 0x143DCC270,
            unkb8: 0x1423F25F5,
            unkc0: 0,
            unkc4: 0,
        }));

        // Set string pointers as god intended
        let model_bytes = model.encode_utf16().collect::<Vec<u16>>();
        request.asset_name[0..5].clone_from_slice(model_bytes.as_slice());
        request.asset_name_str_ptr = request.asset_name.as_ptr() as usize;

        let spawn_chr: extern "C" fn(
            &ChrSet<ChrIns>,
            &ChrSpawnRequest,
            FieldInsHandle,
        ) -> Option<NonNull<ChrIns>> =
            unsafe { std::mem::transmute(self.location.get(RVA_SPAWN_CHR).unwrap()) };

        let setup_chrsync_1: extern "C" fn(
            &NetChrSync,
            &P2PEntityHandle,
        ) -> Option<NonNull<ChrIns>> = unsafe {
            std::mem::transmute(self.location.get(RVA_NET_CHR_SYNC_SETUP_ENTITY_1).unwrap())
        };

        let setup_chrsync_2: extern "C" fn(
            &NetChrSync,
            &P2PEntityHandle,
            bool,
        ) -> Option<NonNull<ChrIns>> = unsafe {
            std::mem::transmute(self.location.get(RVA_NET_CHR_SYNC_SETUP_ENTITY_2).unwrap())
        };

        let setup_chrsync_3: extern "C" fn(
            &NetChrSync,
            &P2PEntityHandle,
            u32,
        ) -> Option<NonNull<ChrIns>> = unsafe {
            std::mem::transmute(self.location.get(RVA_NET_CHR_SYNC_SETUP_ENTITY_3).unwrap())
        };

        let mut chr_ins = spawn_chr(
            &world_chr_man.summon_buddy_chr_set,
            request,
            field_ins_handle.clone(),
        )
        .expect("Could not spawn chr");

        // set team type for all the enemies
        unsafe { chr_ins.as_mut() }.team_type = ENEMY_TEAM_TYPE;

        let p2phandle = &unsafe { chr_ins.as_ref() }.p2p_entity_handle;
        setup_chrsync_1(world_chr_man.net_chr_sync.as_ref(), p2phandle);
        unsafe { chr_ins.as_mut() }
            .net_chr_sync_flags
            .set_unk2(true);

        setup_chrsync_2(world_chr_man.net_chr_sync.as_ref(), p2phandle, true);
        setup_chrsync_3(world_chr_man.net_chr_sync.as_ref(), p2phandle, 0xfff);

        // chr_flags_unk1(unsafe { chr_ins.as_ref() }, true);
        // chr_flags_unk2(unsafe { chr_ins.as_ref() });

        if self.game.is_host() {
            // Notify others of the spawn
            self.networking.send_mob_spawn(
                field_ins_handle,
                map,
                pos,
                orientation,
                npc_param,
                think_param,
                chara_init_param,
                model,
            )?;
        }

        Ok(())
    }

    /// Runs cleanup at end of match.
    pub fn reset(&mut self) {
        self.next_chr_index = CHR_SPAWN_INDEX_START;
        self.spawned_initial_monsters = false;
    }
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
    unka8: u64,                // 0000000141EBB015
    unkb0: u64,                // 0000000143DCC270
    unkb8: u64,                // 00000001423F25F5
    unkc0: u32,                // 0
    unkc4: u32,                // 0
}
