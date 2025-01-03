use std::sync::Arc;

use game::pointer::OwnedPtr;

use crate::{
    gamestate::GameStateProvider,
    rva::{RVA_AI_LOCK_ON_TEAMTYPE_RELATIONS_TABLE, RVA_DAMAGE_TEAMTYPE_RELATIONS_TABLE},
    ProgramLocationProvider,
};

const NEUTRAL_TEAM_INDEX: (u8, u8) = (0, 0);
// Host to Host
const FRIEND_TEAM_INDEX: (u8, u8) = (1, 1);
// Invader to host
const ENEMY_TEAM_INDEX: (u8, u8) = (13, 1);

pub const OTHER_PLAYER_TEAM_TYPE: u8 = 0;
pub const ENEMY_TEAM_TYPE: u8 = 7;
pub const LOCAL_PLAYER_TEAM_TYPE: u8 = 78;

type TeamRelationMatrix = [[u64; 79]; 79];

#[derive(Default)]
struct TeamRelationBackup {
    // 7 to 7
    enemy_to_ememy_ai: u64,
    enemy_to_enemy_hit: u64,
    // 7 to 79
    enemy_to_player_ai: u64,
    enemy_to_player_hit: u64,
    // 7 to 0
    enemy_to_participant_ai: u64,
    enemy_to_participant_hit: u64,
    // 79 to 79
    player_to_teammate_ai: u64,
    player_to_teammate_hit: u64,
    // 79 to 7
    player_to_enemy_ai: u64,
    player_to_enemy_hit: u64,
    // 79 to 0
    player_to_participant_ai: u64,
    player_to_participant_hit: u64,
}

pub struct TeamRelations {
    game: Arc<GameStateProvider>,
    location: Arc<ProgramLocationProvider>,
    backup: TeamRelationBackup,
    applied_table_patches: bool,
}

impl TeamRelations {
    pub fn new(game: Arc<GameStateProvider>, location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            game,
            location,
            backup: TeamRelationBackup::default(),
            applied_table_patches: false,
        }
    }
    pub fn update(&mut self) {
        if self.game.match_loading() && !self.applied_table_patches {
            tracing::info!("Patching team relations");
            self.backup_table();
            self.patch_tables();
            self.applied_table_patches = true;
        }
    }
    pub fn reset(&mut self) {
        self.restore_table();
        self.applied_table_patches = false;
    }

    unsafe fn ai_matrix(&self) -> OwnedPtr<TeamRelationMatrix> {
        let ai_table_va = self
            .location
            .get(RVA_AI_LOCK_ON_TEAMTYPE_RELATIONS_TABLE)
            .unwrap();

        std::mem::transmute(ai_table_va)
    }

    unsafe fn hit_matrix(&self) -> OwnedPtr<TeamRelationMatrix> {
        let hit_table_va = self
            .location
            .get(RVA_DAMAGE_TEAMTYPE_RELATIONS_TABLE)
            .unwrap();

        std::mem::transmute(hit_table_va)
    }

    fn patch_tables(&mut self) {
        let mut ai_matrix = unsafe { self.ai_matrix() };
        let mut hit_matrix = unsafe { self.hit_matrix() };

        let enemy_relation = hit_matrix[ENEMY_TEAM_INDEX.0 as usize][ENEMY_TEAM_INDEX.1 as usize];
        let friend_relation =
            hit_matrix[FRIEND_TEAM_INDEX.0 as usize][FRIEND_TEAM_INDEX.1 as usize];
        let neutral_relation =
            hit_matrix[NEUTRAL_TEAM_INDEX.0 as usize][NEUTRAL_TEAM_INDEX.1 as usize];

        // local player to local player
        hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] =
            friend_relation;
        ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] =
            friend_relation;
        // local player to enemy
        hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] = enemy_relation;
        ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] = enemy_relation;
        // local player to other player
        hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            enemy_relation;
        ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            enemy_relation;

        // enemy to enemy
        hit_matrix[ENEMY_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] = neutral_relation;
        ai_matrix[ENEMY_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] = neutral_relation;
        // enemy to local player
        hit_matrix[ENEMY_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] = enemy_relation;
        ai_matrix[ENEMY_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] = enemy_relation;
        // enemy to other player
        hit_matrix[ENEMY_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] = enemy_relation;
        ai_matrix[ENEMY_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] = enemy_relation;

        // other player to other player
        hit_matrix[OTHER_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            friend_relation;
        ai_matrix[OTHER_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            friend_relation;
        // other relations can be left as they are
        // because they are handled by remore players from their side
    }

    fn backup_table(&mut self) {
        let ai_matrix = unsafe { self.ai_matrix() };
        let hit_matrix = unsafe { self.hit_matrix() };

        self.backup.enemy_to_ememy_ai =
            ai_matrix[ENEMY_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize];
        self.backup.enemy_to_enemy_hit =
            hit_matrix[ENEMY_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize];
        self.backup.enemy_to_player_ai =
            ai_matrix[ENEMY_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize];
        self.backup.enemy_to_player_hit =
            hit_matrix[ENEMY_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize];
        self.backup.enemy_to_participant_ai =
            ai_matrix[ENEMY_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize];
        self.backup.enemy_to_participant_hit =
            hit_matrix[ENEMY_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize];
        self.backup.player_to_teammate_ai =
            ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize];
        self.backup.player_to_teammate_hit =
            hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize];
        self.backup.player_to_enemy_ai =
            ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize];
        self.backup.player_to_enemy_hit =
            hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize];
        self.backup.player_to_participant_ai =
            ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize];
        self.backup.player_to_participant_hit =
            hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize];
    }

    fn restore_table(&self) {
        let mut ai_matrix = unsafe { self.ai_matrix() };
        let mut hit_matrix = unsafe { self.hit_matrix() };

        ai_matrix[ENEMY_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] =
            self.backup.enemy_to_ememy_ai;
        hit_matrix[ENEMY_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] =
            self.backup.enemy_to_enemy_hit;
        ai_matrix[ENEMY_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] =
            self.backup.enemy_to_player_ai;
        hit_matrix[ENEMY_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] =
            self.backup.enemy_to_player_hit;
        ai_matrix[ENEMY_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            self.backup.enemy_to_participant_ai;
        hit_matrix[ENEMY_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            self.backup.enemy_to_participant_hit;
        ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] =
            self.backup.player_to_teammate_ai;
        hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][LOCAL_PLAYER_TEAM_TYPE as usize] =
            self.backup.player_to_teammate_hit;
        ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] =
            self.backup.player_to_enemy_ai;
        hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][ENEMY_TEAM_TYPE as usize] =
            self.backup.player_to_enemy_hit;
        ai_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            self.backup.player_to_participant_ai;
        hit_matrix[LOCAL_PLAYER_TEAM_TYPE as usize][OTHER_PLAYER_TEAM_TYPE as usize] =
            self.backup.player_to_participant_hit;
    }
}
