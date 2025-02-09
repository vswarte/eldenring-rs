use std::sync::Arc;

use game::{cs::WorldChrMan, pointer::OwnedPtr};
use util::singleton::get_instance;

use crate::{
    gamestate::GameStateProvider,
    rva::{RVA_AI_LOCK_ON_TEAMTYPE_RELATIONS_TABLE, RVA_DAMAGE_TEAMTYPE_RELATIONS_TABLE},
    ProgramLocationProvider,
};

const NEUTRAL_TEAM_INDEX: (usize, usize) = (0, 0);
// Host to Host
const FRIEND_TEAM_INDEX: (usize, usize) = (1, 1);
// Invader to host
const ENEMY_TEAM_INDEX: (usize, usize) = (13, 1);

pub const OTHER_PLAYER_TEAM_TYPE: u8 = 0;
pub const LOCAL_PLAYER_TEAM_TYPE: u8 = 1;
pub const ENEMY_TEAM_TYPE: u8 = 7;

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
    overrided_teams: bool,
}

impl TeamRelations {
    pub fn new(game: Arc<GameStateProvider>, location: Arc<ProgramLocationProvider>) -> Self {
        Self {
            game,
            location,
            backup: TeamRelationBackup::default(),
            applied_table_patches: false,
            overrided_teams: false,
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

    pub fn override_teams(&mut self, party: Vec<u64>) {
        if self.overrided_teams {
            return;
        }
        if let Ok(Some(world_chr_man)) = unsafe { get_instance::<WorldChrMan>() } {
            world_chr_man.player_chr_set.characters().for_each(|c| {
                c.chr_ins.team_type = if c.chr_ins.field_ins_handle
                    == world_chr_man
                        .main_player
                        .as_ref()
                        .unwrap()
                        .chr_ins
                        .field_ins_handle
                    || party.contains(
                        &c.player_session_holder
                            .player_network_session
                            .remote_identity,
                    ) {
                    LOCAL_PLAYER_TEAM_TYPE
                } else {
                    OTHER_PLAYER_TEAM_TYPE
                };
            })
        } else {
            return;
        }

        self.overrided_teams = true;
    }

    pub fn reset(&mut self) {
        if self.applied_table_patches {
            self.restore_table();
            self.applied_table_patches = false;
        }
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

        let enemy_relation = hit_matrix[ENEMY_TEAM_INDEX.0][ENEMY_TEAM_INDEX.1];
        let friend_relation = hit_matrix[FRIEND_TEAM_INDEX.0][FRIEND_TEAM_INDEX.1];
        let neutral_relation = hit_matrix[NEUTRAL_TEAM_INDEX.0][NEUTRAL_TEAM_INDEX.1];

        let enemy = ENEMY_TEAM_TYPE as usize;
        let local_player = LOCAL_PLAYER_TEAM_TYPE as usize;
        let other_player = OTHER_PLAYER_TEAM_TYPE as usize;

        // local player to local player
        hit_matrix[local_player][local_player] = friend_relation;
        ai_matrix[local_player][local_player] = friend_relation;
        // local player to enemy
        hit_matrix[local_player][enemy] = enemy_relation;
        ai_matrix[local_player][enemy] = enemy_relation;
        // local player to other player
        hit_matrix[local_player][other_player] = enemy_relation;
        ai_matrix[local_player][other_player] = enemy_relation;

        // enemy to enemy
        hit_matrix[enemy][enemy] = neutral_relation;
        ai_matrix[enemy][enemy] = neutral_relation;
        // enemy to local player
        hit_matrix[enemy][local_player] = enemy_relation;
        ai_matrix[enemy][local_player] = enemy_relation;
        // enemy to other player
        hit_matrix[enemy][other_player] = enemy_relation;
        ai_matrix[enemy][other_player] = enemy_relation;

        // other player to other player
        hit_matrix[other_player][other_player] = friend_relation;
        ai_matrix[other_player][other_player] = friend_relation;
        // other relations can be left as they are
        // because they are handled by remore players from their side
    }

    fn backup_table(&mut self) {
        let ai_matrix = unsafe { self.ai_matrix() };
        let hit_matrix = unsafe { self.hit_matrix() };

        let enemy = ENEMY_TEAM_TYPE as usize;
        let local_player = LOCAL_PLAYER_TEAM_TYPE as usize;
        let other_player = OTHER_PLAYER_TEAM_TYPE as usize;

        self.backup.enemy_to_ememy_ai = ai_matrix[enemy][enemy];
        self.backup.enemy_to_enemy_hit = hit_matrix[enemy][enemy];
        self.backup.enemy_to_player_ai = ai_matrix[enemy][local_player];
        self.backup.enemy_to_player_hit = hit_matrix[enemy][local_player];
        self.backup.enemy_to_participant_ai = ai_matrix[enemy][other_player];
        self.backup.enemy_to_participant_hit = hit_matrix[enemy][other_player];
        self.backup.player_to_teammate_ai = ai_matrix[local_player][local_player];
        self.backup.player_to_teammate_hit = hit_matrix[local_player][local_player];
        self.backup.player_to_enemy_ai = ai_matrix[local_player][enemy];
        self.backup.player_to_enemy_hit = hit_matrix[local_player][enemy];
        self.backup.player_to_participant_ai = ai_matrix[local_player][other_player];
        self.backup.player_to_participant_hit = hit_matrix[local_player][other_player];
    }

    fn restore_table(&self) {
        let mut ai_matrix = unsafe { self.ai_matrix() };
        let mut hit_matrix = unsafe { self.hit_matrix() };

        let enemy = ENEMY_TEAM_TYPE as usize;
        let local_player = LOCAL_PLAYER_TEAM_TYPE as usize;
        let other_player = OTHER_PLAYER_TEAM_TYPE as usize;

        ai_matrix[enemy][enemy] = self.backup.enemy_to_ememy_ai;
        hit_matrix[enemy][enemy] = self.backup.enemy_to_enemy_hit;
        ai_matrix[enemy][local_player] = self.backup.enemy_to_player_ai;
        hit_matrix[enemy][local_player] = self.backup.enemy_to_player_hit;
        ai_matrix[enemy][other_player] = self.backup.enemy_to_participant_ai;
        hit_matrix[enemy][other_player] = self.backup.enemy_to_participant_hit;
        ai_matrix[local_player][local_player] = self.backup.player_to_teammate_ai;
        hit_matrix[local_player][local_player] = self.backup.player_to_teammate_hit;
        ai_matrix[local_player][enemy] = self.backup.player_to_enemy_ai;
        hit_matrix[local_player][enemy] = self.backup.player_to_enemy_hit;
        ai_matrix[local_player][other_player] = self.backup.player_to_participant_ai;
        hit_matrix[local_player][other_player] = self.backup.player_to_participant_hit;
    }
}
