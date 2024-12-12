use std::sync::Arc;

use game::cs::CSMenuMan;
use util::singleton::get_instance;

use crate::{ProgramLocationProvider, LOCATION_PRESENT_MP_MESSAGE};

#[repr(u32)]
pub enum Message {
    DemigodFelled = 1,
    LegendFelled = 2,
    GreatEnemyFelled = 3,
    EnemyFelled = 4,
    YouDied = 5,
    HostVanquished = 7,
    BloodFingerVanquished = 8,
    DutyFullFilled = 9,
    LostGraceDiscovered = 11,
    Commence = 13,
    Victory = 14,
    Stalemate = 15,
    Defeat = 16,
    MapFound = 17,
    GreatRuneRestored = 21,
    GodSlain = 22,
    DuelistVanquished = 23,
    RecusantVanquished = 24,
    InvaderVanquished = 25,
    FurledFingerRankAdvanced = 30,
    FurledFingerRankAdvanced2 = 31,
    DuelistRankAdvanced = 32,
    DuelistRankAdvanced2 = 33,
    BloodyFingerRankAdvanced = 34,
    BloodyFingerRankAdvanced2 = 35,
    RecusantRankAdvanced = 36,
    RecusantRankAdvanced2 = 37,
    HunterRankAdvanced = 38,
    HunterRankAdvanced2 = 39,
    HeartStolen = 40,
    MenuText = 41,
    YouDiedWithFade= 50,
}

pub struct NotificationPresenter {
    location: Arc<ProgramLocationProvider>,
}

impl NotificationPresenter {
    pub fn new(location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            location,
        }
    }

    /// Displays state message on the screen ala "YOU DIED"
    pub fn present_mp_message(&self, message: Message) {
        let Some(menu_man) = unsafe { get_instance::<CSMenuMan>() }.unwrap() else {
            return;
        };

        let display_message: extern "C" fn(&mut CSMenuMan, Message) = unsafe {
            std::mem::transmute(self.location.get(LOCATION_PRESENT_MP_MESSAGE).unwrap())
        };

        (display_message)(menu_man, message);
    }
}
