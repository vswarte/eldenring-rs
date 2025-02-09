use std::marker::PhantomData;
use std::mem::transmute;
use std::sync::Arc;

use game::cs::ChrIns;
use game::cs::MapId;
use game::cs::QuickmatchManager;
use game::position::ChunkPosition4;
use retour::static_detour;
use thiserror::Error;
use windows::core::w;
use windows::core::PCWSTR;

use crate::config::MapPosition;
use crate::context::GameModeContext;
use crate::gamemode::GameMode;
use crate::gamestate::GameStateProvider;
use crate::location::*;
use crate::rva::RVA_CHR_INS_DEAD;
use crate::rva::RVA_DROPPED_ITEM_CAP_CHECK;
use crate::rva::RVA_GET_TARGET_MAP_ID;
use crate::rva::RVA_INITIAL_SPAWN_POSITION;
use crate::rva::RVA_LOOKUP_MENU_TEXT;
use crate::rva::RVA_MSB_GET_EVENT_DATA_COUNT;
use crate::rva::RVA_MSB_GET_PARTS_DATA_COUNT;
use crate::rva::RVA_MSB_GET_POINT_DATA_COUNT;
use crate::rva::RVA_SPAWN_DROPPED_ITEM_VFX;
use crate::rva::RVA_SUMMON_BUDDY_CHRSET_ALLOC_SIZE;
use crate::rva::RVA_SUMMON_BUDDY_CHRSET_CAPACITY;
use crate::rva::RVA_SUMMON_BUDDY_CHRSET_MEMSET_SIZE;
use crate::ProgramLocationProvider;

static_detour! {
    static HOOK_GET_TARGET_MAP_ID: extern "C" fn(*mut MapId) -> *mut MapId;
    static HOOK_MSB_GET_EVENT_DATA_COUNT: extern "C" fn(usize, u32) -> u32;
    static HOOK_MSB_GET_POINT_DATA_COUNT: extern "C" fn(usize, u32) -> u32;
    static HOOK_MSB_GET_PARTS_DATA_COUNT: extern "C" fn(usize, u32) -> u32;
    static HOOK_CHR_INS_DEAD: extern "C" fn(*mut ChrIns);
    static HOOK_INITIAL_SPAWN_POSITION: extern "C" fn(*mut QuickmatchManager, *mut ChunkPosition4, usize, usize, usize);
    static HOOK_LOOKUP_MENU_TEXT: extern "C" fn(*const usize, u32) -> PCWSTR;
    static HOOK_SPAWN_DROPPED_ITEM_VFX: extern "C" fn(usize, *mut u32, usize);
}

#[derive(Debug, Error)]
pub enum HookError {
    #[error("Location resolver error. {0}")]
    Location(#[from] LocationProviderError),
    #[error("Retour error. {0}")]
    Retour(#[from] retour::Error),
}

pub struct Hooks {}

// TODO: stop using static detours
impl Hooks {
    pub unsafe fn place(
        location: Arc<ProgramLocationProvider>,
        gamemode: Arc<GameMode>,
        context: Arc<GameModeContext>,
        game: Arc<GameStateProvider>,
    ) -> Result<Self, HookError> {
        Self::patch_chr_sets(&location)?;

        // Take control over the players death so we can apply the specator cam.
        Self::hook_player_character(&location, game.clone())?;

        // Take control over the map we're warping into and the spawn position of the player.
        Self::override_map_load(&location, gamemode.clone(), context.clone())?;

        // Stop the overworld MSBs from crashing.
        Self::apply_msb_fixups(&location, gamemode.clone())?;

        // Disable player item drop cap as well as loot visuals.
        Self::patch_loot(&location, gamemode.clone())?;

        // Inject custom strings.
        Self::hook_text_lookups(&location, gamemode.clone())?;

        Ok(Self {})
    }

    /// Patch summonbuddy chrset to allow for 160 entries
    unsafe fn patch_chr_sets(location: &ProgramLocationProvider) -> Result<(), HookError> {
        std::ptr::write_unaligned(
            location.get(RVA_SUMMON_BUDDY_CHRSET_ALLOC_SIZE)? as _,
            0xA00i32,
        );

        std::ptr::write_unaligned(
            location.get(RVA_SUMMON_BUDDY_CHRSET_MEMSET_SIZE)? as _,
            0xA00i32,
        );

        std::ptr::write_unaligned(
            location.get(RVA_SUMMON_BUDDY_CHRSET_CAPACITY)? as _,
            0xA0i32,
        );
        Ok(())
    }

    unsafe fn patch_loot(
        location: &ProgramLocationProvider,
        gamemode: Arc<GameMode>,
    ) -> Result<(), HookError> {
        // Neuter dropped item cap check
        {
            let location = location.get(RVA_DROPPED_ITEM_CAP_CHECK)?;
            unsafe { *(location as *mut u8) = 0xEB };
        }

        // Change the item drop loot visuals
        {
            let gamemode = gamemode.clone();
            HOOK_SPAWN_DROPPED_ITEM_VFX
                .initialize(
                    transmute(location.get(RVA_SPAWN_DROPPED_ITEM_VFX)?),
                    move |param_1: usize, param_2: *mut u32, param_3: usize| {
                        if gamemode.running() {
                            *param_2 = 6109;
                        }

                        HOOK_SPAWN_DROPPED_ITEM_VFX.call(param_1, param_2, param_3)
                    },
                )?
                .enable()?;
        }

        Ok(())
    }

    unsafe fn hook_player_character(
        location: &ProgramLocationProvider,
        game: Arc<GameStateProvider>,
    ) -> Result<(), HookError> {
        // Take control over character death so we can enforce spectator mode instead
        {
            HOOK_CHR_INS_DEAD
                .initialize(
                    transmute(location.get(RVA_CHR_INS_DEAD)?),
                    move |chr_ins: *mut ChrIns| {
                        let is_main_player = game
                            .local_player()
                            .is_some_and(|h| &h == unsafe { &(*chr_ins).field_ins_handle });

                        if !game.match_in_game() || !is_main_player {
                            return HOOK_CHR_INS_DEAD.call(chr_ins);
                        }

                        // Disable character collision
                        chr_ins.as_mut().unwrap().chr_ctrl.flags |= 2;
                        tracing::info!("Caught ChrIns death");
                    },
                )?
                .enable()?;
        }

        Ok(())
    }

    unsafe fn override_map_load(
        location: &ProgramLocationProvider,
        gamemode: Arc<GameMode>,
        context: Arc<GameModeContext>,
    ) -> Result<(), HookError> {
        {
            let context = context.clone();
            let gamemode = gamemode.clone();
            // Override map ID on qm map load
            HOOK_GET_TARGET_MAP_ID
                .initialize(
                    transmute(location.get(RVA_GET_TARGET_MAP_ID)?),
                    move |map: *mut MapId| {
                        let result = HOOK_GET_TARGET_MAP_ID.call(map);

                        if let Some(point) = context.spawn_point() {
                            // if (*map).0 == 0x2D000000 {
                            *map = MapId(point.map.0);
                            tracing::info!("Patched target map ID {}", *map);
                            // }
                        }

                        map
                    },
                )?
                .enable()?;
        }

        // Override initial spawn pos
        {
            let gamemode = gamemode.clone();
            HOOK_INITIAL_SPAWN_POSITION
                .initialize(
                    transmute(location.get(RVA_INITIAL_SPAWN_POSITION)?),
                    move |quickmatch_manager: *mut QuickmatchManager,
                          position: *mut ChunkPosition4,
                          orientation: usize,
                          msb_res_cap: usize,
                          param_5: usize| {
                        if !gamemode.running() {
                            return HOOK_INITIAL_SPAWN_POSITION.call(
                                quickmatch_manager,
                                position,
                                orientation,
                                msb_res_cap,
                                param_5,
                            );
                        }

                        HOOK_INITIAL_SPAWN_POSITION.call(
                            quickmatch_manager,
                            position,
                            orientation,
                            msb_res_cap,
                            param_5,
                        );

                        // Here's praying the message was received in time...
                        let point = context.spawn_point().unwrap();
                        let MapPosition(x, y, z) = context.spawn_point().unwrap().position;
                        tracing::info!(
                            "Overriding initial spawn position. {:x}, ({x}, {y}, {z}) - {}",
                            point.map.0,
                            point.orientation
                        );

                        (*position).0 .0 = x;
                        (*position).0 .1 = y;
                        (*position).0 .2 = z;

                        // TODO: set orientation
                    },
                )?
                .enable()?;
        }
        Ok(())
    }

    unsafe fn apply_msb_fixups(
        location: &ProgramLocationProvider,
        gamemode: Arc<GameMode>,
    ) -> Result<(), HookError> {
        // Disable loading of certain MSB event entries
        {
            let gamemode = gamemode.clone();
            HOOK_MSB_GET_EVENT_DATA_COUNT
                .initialize(
                    transmute(location.get(RVA_MSB_GET_EVENT_DATA_COUNT)?),
                    move |msb_res_cap: usize, event_type: u32| {
                        if !gamemode.running() {
                            return HOOK_MSB_GET_EVENT_DATA_COUNT.call(msb_res_cap, event_type);
                        }

                        match event_type {
                            // Disable treasure
                            4 => 0,
                            // Disable NPC invasions
                            12 => 0,
                            // Disable sign pools
                            23 => 0,
                            // Disable retry points
                            24 => 0,

                            _ => HOOK_MSB_GET_EVENT_DATA_COUNT.call(msb_res_cap, event_type),
                        }
                    },
                )?
                .enable()?;
        }

        // Disable loading of certain MSB point entries
        {
            let gamemode = gamemode.clone();
            HOOK_MSB_GET_PARTS_DATA_COUNT
                .initialize(
                    transmute(location.get(RVA_MSB_GET_PARTS_DATA_COUNT)?),
                    move |msb_res_cap: usize, parts_type: u32| {
                        if !gamemode.running() {
                            return HOOK_MSB_GET_PARTS_DATA_COUNT.call(msb_res_cap, parts_type);
                        }

                        match parts_type {
                            // Disable enemies
                            2 => 0,

                            // Disable dummy enemies
                            9 => 0,

                            _ => HOOK_MSB_GET_PARTS_DATA_COUNT.call(msb_res_cap, parts_type),
                        }
                    },
                )?
                .enable()?;
        }

        // Disable loading of certain MSB parts entries
        {
            let gamemode = gamemode.clone();
            HOOK_MSB_GET_POINT_DATA_COUNT
                .initialize(
                    transmute(location.get(RVA_MSB_GET_POINT_DATA_COUNT)?),
                    move |msb_res_cap: usize, point_type: u32| {
                        if !gamemode.running() {
                            return HOOK_MSB_GET_POINT_DATA_COUNT.call(msb_res_cap, point_type);
                        }

                        match point_type {
                            // Disable triggers shapes
                            0 => 0,

                            // Disable invasion points
                            1 => 0,

                            _ => HOOK_MSB_GET_POINT_DATA_COUNT.call(msb_res_cap, point_type),
                        }
                    },
                )?
                .enable()?;
        }

        Ok(())
    }

    unsafe fn hook_text_lookups(
        location: &ProgramLocationProvider,
        gamemode: Arc<GameMode>,
    ) -> Result<(), HookError> {
        {
            let gamemode = gamemode.clone();
            HOOK_LOOKUP_MENU_TEXT
                .initialize(
                    transmute(location.get(RVA_LOOKUP_MENU_TEXT)?),
                    move |msg_repository_imp: *const usize, entry: u32| {
                        let original = HOOK_LOOKUP_MENU_TEXT.call(msg_repository_imp, entry);
                        // tracing::info!("MenuText lookup {entry} -> {}", original.to_string().unwrap());

                        if entry == 506201 {
                            w!("Battle Royale")
                        } else {
                            original
                        }
                    },
                )?
                .enable()?;
        }

        Ok(())
    }
}
