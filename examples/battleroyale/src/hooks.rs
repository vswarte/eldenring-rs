use std::marker::PhantomData;
use std::sync::Arc;

use game::cs::ChrIns;
use game::cs::QuickmatchManager;
use game::position::ChunkPosition4;
use retour::static_detour;
use thiserror::Error;
use windows::core::w;
use windows::core::PCWSTR;

use crate::gamemode::GameMode;
use crate::gamestate::GameStateProvider;
use crate::location::*;
use crate::ProgramLocationProvider;

static_detour! {
    static HOOK_MAP_QUICKMATCH_ENUM_TO_MAP_ID: extern "C" fn(*mut u32, u32) -> *mut u32;
    static HOOK_MSB_GET_EVENT_DATA_COUNT: extern "C" fn(usize, u32) -> u32;
    static HOOK_MSB_GET_POINT_DATA_COUNT: extern "C" fn(usize, u32) -> u32;
    static HOOK_MSB_GET_PARTS_DATA_COUNT: extern "C" fn(usize, u32) -> u32;
    static HOOK_CHR_INS_DEAD: extern "C" fn(*mut ChrIns);
    static HOOK_INITIAL_SPAWN_POSITION: extern "C" fn(*mut QuickmatchManager, *mut ChunkPosition4, usize, usize, usize);
    static HOOK_LOOKUP_MENU_TEXT: extern "C" fn(*const usize, u32) -> PCWSTR;
}

#[derive(Debug, Error)]
pub enum HookError {
    #[error("Location resolver error. {0}")]
    Location(#[from] LocationProviderError),
    #[error("Retour error. {0}")]
    Retour(#[from] retour::Error),
}

pub struct GamemodeHooks<S, L>
where
    S: GameStateProvider + Send + Sync + 'static,
    L: ProgramLocationProvider + Send + Sync + 'static,
{
    _game_state: PhantomData<S>,
    _location: PhantomData<L>,
}

// TODO: stop using static detours
impl<S, L> GamemodeHooks<S, L>
where
    S: GameStateProvider + Send + Sync + 'static,
    L: ProgramLocationProvider + Send + Sync + 'static,
{
    pub unsafe fn place(location: Arc<L>, gamemode: Arc<GameMode<S, L>>) -> Result<Self, HookError>
    where
        L: ProgramLocationProvider,
    {
        // Take control over the players death so we can apply the specator cam.
        Self::hook_player_characters(&location, gamemode.clone())?;

        // Take control over the map we're warping into and the spawn position of the player.
        Self::override_map_load(&location, gamemode.clone())?;

        // Stop the overworld MSBs from crashing.
        Self::apply_msb_fixups(&location, gamemode.clone())?;

        // Disable player item drop cap
        Self::patch_item_drop_limit(&location, gamemode.clone())?;

        // Inject custom strings
        Self::hook_text_lookups(&location, gamemode.clone())?;

        Ok(Self {
            _game_state: PhantomData,
            _location: PhantomData,
        })
    }

    unsafe fn patch_item_drop_limit(
        location: &L,
        gamemode: Arc<GameMode<S, L>>,
    ) -> Result<(), HookError> {
        // Neuter dropped item cap check
        let location = location.get(LOCATION_DROPPED_ITEM_CAP_CHECK)?;
        unsafe { *(location as *mut u8) = 0xEB };

        Ok(())
    }

    unsafe fn hook_player_characters(
        location: &L,
        gamemode: Arc<GameMode<S, L>>,
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
                        let chr_ins = chr_ins.as_ref().unwrap();
                        gamemode.handle_death(&chr_ins.field_ins_handle);
                    },
                )?
                .enable()?;
        }

        Ok(())
    }

    unsafe fn override_map_load(
        location: &L,
        gamemode: Arc<GameMode<S, L>>,
    ) -> Result<(), HookError> {
        {
            let gamemode = gamemode.clone();
            // Override map ID on qm map load
            HOOK_MAP_QUICKMATCH_ENUM_TO_MAP_ID
                .initialize(
                    std::mem::transmute(location.get(LOCATION_MAP_QUICKMATCH_ENUM_TO_MAP_ID)?),
                    move |map_id: *mut u32, map: u32| {
                        let result = HOOK_MAP_QUICKMATCH_ENUM_TO_MAP_ID.call(map_id, map);
                        let target_map_id = gamemode.target_map(map);
                        *result = (&gamemode.target_map(map)).into();
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
                        let (x, y, z) = gamemode.player_spawn_point().position.xyz();
                        (*position).0 .0 = x;
                        (*position).0 .1 = y;
                        (*position).0 .2 = z;
                    },
                )?
                .enable()?;
        }
        Ok(())
    }

    unsafe fn apply_msb_fixups(
        location: &L,
        gamemode: Arc<GameMode<S, L>>,
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
        location: &L,
        gamemode: Arc<GameMode<S, L>>,
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
