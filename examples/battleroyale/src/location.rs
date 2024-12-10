use std::collections::HashMap;

use pelite::pe::{Pe, Rva, Va};
use thiserror::Error;
use util::program::Program;

// Maps quickmatch enum to a map ID used for loading
pub const LOCATION_MAP_QUICKMATCH_ENUM_TO_MAP_ID: &str = "MAP_QUICKMATCH_ENUM_TO_MAP_ID";
// Retrieves the amount of some structure for a given type. Used to remove aspects of some map when
// loading into the map as a quickmatch arena.
pub const LOCATION_MSB_GET_EVENT_DATA_COUNT: &str = "MSB_GET_EVENT_DATA_COUNT";
pub const LOCATION_MSB_GET_PARTS_DATA_COUNT: &str = "MSB_GET_PARTS_DATA_COUNT";
pub const LOCATION_MSB_GET_POINT_DATA_COUNT: &str = "MSB_GET_POINT_DATA_COUNT";
// Gets called when a character dies, including the local player. Used to prevent the player from
// fully dying so we can put the player in spectator mode.
pub const LOCATION_CHR_INS_DEAD: &str = "CHR_INS_DEAD";
// Retrieves the initial spawn position for a given quickmatch arena. Used to place the player on a
// Host-determined spawn point at quickmatch start.
pub const LOCATION_INITIAL_SPAWN_POSITION: &str = "INITIAL_SPAWN_POSITION";
// Function that presents player with MP status message like "YOU DIED"
pub const LOCATION_PRESENT_MP_MESSAGE: &str = "PRESENT_MP_MESSAGE";
// Function that drops an item with networking.
pub const LOCATION_SPAWN_DROPPED_ITEM: &str = "SPAWN_DROPPED_ITEM";
// JL of a check to cap amount of dropped items.
pub const LOCATION_DROPPED_ITEM_CAP_CHECK: &str = "DROPPED_ITEM_CAP_CHECK";
// Function that executes a prepared debug ffx spawn.
pub const LOCATION_SPAWN_DEBUG_FFX: &str = "SPAWN_DEBUG_FFX";
// Message repo lookup fn for ?MenuText?. Contains quickmatch strings.
pub const LOCATION_LOOKUP_MENU_TEXT: &str = "LOOKUP_MENU_TEXT";
// Fn that transfers items between two inventory data instances.
pub const LOCATION_TRANSFER_ITEM: &str = "LOOKUP_TRANSFER_ITEM";
// Applies speffect to chrins
pub const LOCATION_APPLY_SPEFFECT: &str = "APPLY_SPEFFECT";

pub trait ProgramLocationProvider {
    fn get(&self, name: &str) -> Result<Va, LocationProviderError>;
}

#[derive(Debug, Error)]
pub enum LocationProviderError {
    #[error("Could not convert RVA to VA")]
    AddressConversion(#[from] pelite::Error),

    #[error("Could not retrieve IBO for requested location {0}.")]
    LocationNotFound(String),
}

pub struct HardcodedLocationProvider {
    program: Program<'static>,
    offsets: HashMap<&'static str, Rva>,
}

impl HardcodedLocationProvider {
    pub fn new() -> Self {
        Self {
            program: unsafe { Program::current() },
            offsets: HashMap::from([
                (LOCATION_CHR_INS_DEAD, 0x3fcc60),
                (LOCATION_MAP_QUICKMATCH_ENUM_TO_MAP_ID, 0xa3c8a0),
                (LOCATION_INITIAL_SPAWN_POSITION, 0xa4cd70),
                (LOCATION_MSB_GET_EVENT_DATA_COUNT, 0xcf5c10),
                (LOCATION_MSB_GET_PARTS_DATA_COUNT, 0xcf5da0),
                (LOCATION_MSB_GET_POINT_DATA_COUNT, 0xcf6360),
                (LOCATION_PRESENT_MP_MESSAGE, 0x766460),
                (LOCATION_SPAWN_DROPPED_ITEM, 0x561620),
                (LOCATION_DROPPED_ITEM_CAP_CHECK, 0x561fea),
                (LOCATION_SPAWN_DEBUG_FFX, 0xd963c0),
                (LOCATION_LOOKUP_MENU_TEXT, 0xd10a00),
                (LOCATION_TRANSFER_ITEM, 0x24dc40),
                (LOCATION_APPLY_SPEFFECT, 0x3e8cf0),
            ]),
        }
    }
}

impl ProgramLocationProvider for HardcodedLocationProvider {
    /// Gets the virtual runtime address for a given named location.
    fn get(&self, name: &str) -> Result<Va, LocationProviderError> {
        Ok(
            self.offsets
                .get(name)
                .map(|rva| {
                    self.program
                        .rva_to_va(*rva)
                })
                .ok_or(LocationProviderError::LocationNotFound(
                    name.to_string(),
                ))??
        )
    }
}
