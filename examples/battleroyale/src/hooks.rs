use std::marker::PhantomData;
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
use crate::gamemode::GameMode;
use crate::gamestate::GameStateProvider;
use crate::location::*;
use crate::ProgramLocationProvider;

static_detour! {
    static HOOK_MAP_QUICKMATCH_ENUM_TO_MAP_ID: extern "C" fn(*mut MapId, u32) -> *mut MapId;
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
    ) -> Result<Self, HookError> {
        // Take control over the players death so we can apply the specator cam.
        Self::hook_player_characters(&location, gamemode.clone())?;

        // Take control over the map we're warping into and the spawn position of the player.
        Self::override_map_load(&location, gamemode.clone())?;

        // Stop the overworld MSBs from crashing.
        Self::apply_msb_fixups(&location, gamemode.clone())?;

        // Disable player item drop cap as well as loot visuals.
        Self::patch_loot(&location, gamemode.clone())?;

        // Inject custom strings.
        Self::hook_text_lookups(&location, gamemode.clone())?;

        Ok(Self {})
    }

    unsafe fn patch_loot(
        location: &ProgramLocationProvider,
        gamemode: Arc<GameMode>,
    ) -> Result<(), HookError> {
        // Neuter dropped item cap check
        {
            let location = location.get(LOCATION_DROPPED_ITEM_CAP_CHECK)?;
            unsafe { *(location as *mut u8) = 0xEB };
        }

        // Change the item drop loot visuals
        {
            let gamemode = gamemode.clone();
            HOOK_SPAWN_DROPPED_ITEM_VFX
                .initialize(
                    std::mem::transmute(location.get(LOCATION_SPAWN_DROPPED_ITEM_VFX)?),
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

    unsafe fn hook_player_characters(
        location: &ProgramLocationProvider,
        gamemode: Arc<GameMode>,
    ) -> Result<(), HookError> {
        // Take control over character death so we can enforce spectator mode instead
        {
            let gamemode = gamemode.clone();
            HOOK_CHR_INS_DEAD
                .initialize(
                    std::mem::transmute(location.get(LOCATION_CHR_INS_DEAD)?),
                    move |chr_ins: *mut ChrIns| {
                        if !gamemode.running() {
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
    ) -> Result<(), HookError> {
        {
            let gamemode = gamemode.clone();
            // Override map ID on qm map load
            HOOK_MAP_QUICKMATCH_ENUM_TO_MAP_ID
                .initialize(
                    std::mem::transmute(location.get(LOCATION_MAP_QUICKMATCH_ENUM_TO_MAP_ID)?),
                    move |map_id: *mut MapId, map: u32| {
                        let result = HOOK_MAP_QUICKMATCH_ENUM_TO_MAP_ID.call(map_id, map);
                        let target_map_id = gamemode.target_map(map);
                        *result = target_map_id;
                        result
                    },
                )?
                .enable()?;
        }

        // Override initial spawn pos
        {
            let gamemode = gamemode.clone();
            HOOK_INITIAL_SPAWN_POSITION
                .initialize(
                    std::mem::transmute(location.get(LOCATION_INITIAL_SPAWN_POSITION)?),
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

                        // Here's praying the message was received in-time...
                        tracing::info!("Overriding initial spawn position");
                        let MapPosition(x, y, z) = gamemode.player_spawn_point().position;

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
                    std::mem::transmute(location.get(LOCATION_MSB_GET_EVENT_DATA_COUNT)?),
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
                    std::mem::transmute(location.get(LOCATION_MSB_GET_PARTS_DATA_COUNT)?),
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
                    std::mem::transmute(location.get(LOCATION_MSB_GET_POINT_DATA_COUNT)?),
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
                    std::mem::transmute(location.get(LOCATION_LOOKUP_MENU_TEXT)?),
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
