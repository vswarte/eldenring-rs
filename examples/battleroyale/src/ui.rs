use std::sync::Arc;

use game::cs::CSMenuMan;
use util::singleton::get_instance;

use crate::{gamestate::GameStateProvider, rva::RVA_PRESENT_MP_MESSAGE, ProgramLocationProvider};

/// Generates and spawns random loot over the map
pub struct Ui {
    game: Arc<GameStateProvider>,
    location: Arc<ProgramLocationProvider>,

    /// Did we give the start signal?
    presented_start: bool,

    /// Did we present people with the result of the match?
    presented_result: bool,
}

impl Ui {
    pub fn new(game: Arc<GameStateProvider>, location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            game,
            location,
            presented_start: false,
            presented_result: false,
        }
    }

    pub fn update(&mut self) {
        if self.game.match_in_game() && !self.presented_start {
            self.presented_start = true;
            self.present_match_start();
        }

        if self.game.match_concluded() && !self.presented_result {
            self.presented_result = true;
            self.present_match_result(self.game.is_winner());
        }
    }

    pub fn reset(&mut self) {
        self.presented_start = false;
        self.presented_result = false;
    }

    fn present_match_start(&self) {
        let Some(menu_man) = unsafe { get_instance::<CSMenuMan>() }.unwrap() else {
            return;
        };

        let display_fullscreen_message: extern "C" fn(&mut CSMenuMan, FullscreenMessage) =
            unsafe { std::mem::transmute(self.location.get(RVA_PRESENT_MP_MESSAGE).unwrap()) };

        (display_fullscreen_message)(menu_man, FullscreenMessage::Commence);
    }


    fn present_match_result(&self, win: bool) {
        let Some(menu_man) = unsafe { get_instance::<CSMenuMan>() }.unwrap() else {
            return;
        };

        let display_fullscreen_message: extern "C" fn(&mut CSMenuMan, FullscreenMessage) =
            unsafe { std::mem::transmute(self.location.get(RVA_PRESENT_MP_MESSAGE).unwrap()) };

        let message = match win {
            true => FullscreenMessage::Victory,
            false => FullscreenMessage::Defeat,
        };
 
        (display_fullscreen_message)(menu_man, message);
    }
}

#[repr(u32)]
enum FullscreenMessage {
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
    YouDiedWithFade = 50,
}
