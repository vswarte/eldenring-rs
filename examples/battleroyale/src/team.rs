use std::{ptr::NonNull, sync::Arc};

use game::{cs::CSTeamTypeBase, pointer::OwnedPtr};

use crate::{gamestate::GameStateProvider, rva::{RVA_AI_LOCK_ON_TEAMTYPE_RELATIONS_TABLE, RVA_DAMAGE_TEAMTYPE_RELATIONS_TABLE}, ProgramLocationProvider};

type TeamRelationsMatrix = [[NonNull<CSTeamTypeBase>; 79]; 79];

const TEAM_RELATION_TEAM_PLAYER: usize = 2;
const TEAM_RELATION_ENEMY_PLAYER: usize = 3;
const TEAM_RELATION_ENEMY_MONSTER: usize = 7;

pub struct TeamRelations {
    game: Arc<GameStateProvider>,
    location: Arc<ProgramLocationProvider>,
    applied_table_patches: bool,
}

impl TeamRelations {
    pub fn new(game: Arc<GameStateProvider>, location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            game,
            location,
            applied_table_patches: false,
        }
    }

    pub fn update(&mut self) {
        if self.game.match_loading() && !self.applied_table_patches {
            tracing::info!("Patching team relations");
            self.applied_table_patches == true;

            let mut table_1: OwnedPtr<TeamRelationsMatrix> = unsafe {
                std::mem::transmute(self.location.get(RVA_DAMAGE_TEAMTYPE_RELATIONS_TABLE).unwrap())
            };

            let mut table_2: OwnedPtr<TeamRelationsMatrix> = unsafe {
                std::mem::transmute(self.location.get(RVA_AI_LOCK_ON_TEAMTYPE_RELATIONS_TABLE).unwrap())
            };

            table_1[72][6] = table_1[2][6];
            table_1[6][72] = table_1[6][2];
            table_2[72][6] = table_2[2][6];
            table_2[6][72] = table_2[6][2];
        }
    }

    pub fn reset(&mut self) {
        self.applied_table_patches = false;
    }
}
