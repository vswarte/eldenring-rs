use eldenring::cs::{CSTeamTypeVmt, TeamRelationTargetInfo};
use vtable_rs::VPtr;

pub struct CSTeamTypeNeutral {
    pub vftable: VPtr<dyn CSTeamTypeVmt, Self>,
}

impl CSTeamTypeVmt for CSTeamTypeNeutral {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        false
    }
}

pub struct CSTeamTypeFriend {
    pub vftable: VPtr<dyn CSTeamTypeVmt, Self>,
}

impl CSTeamTypeVmt for CSTeamTypeFriend {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        team_relation.friendly_target
    }
}

pub struct CSTeamTypeEnemy {
    pub vftable: VPtr<dyn CSTeamTypeVmt, Self>,
}

impl CSTeamTypeVmt for CSTeamTypeEnemy {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        team_relation.oppose_target
    }
}

pub struct CSTeamTypeRival {
    pub vftable: VPtr<dyn CSTeamTypeVmt, Self>,
}

impl CSTeamTypeVmt for CSTeamTypeRival {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        if self_target {
            return team_relation.self_target;
        }
        if !team_relation.oppose_target && !team_relation.friendly_target {
            return false;
        }
        true
    }
}

pub static TEAM_TYPE_RIVAL: CSTeamTypeRival = CSTeamTypeRival {
    vftable: VPtr::new(),
};

pub static TEAM_TYPE_ENEMY: CSTeamTypeEnemy = CSTeamTypeEnemy {
    vftable: VPtr::new(),
};

pub static TEAM_TYPE_FRIEND: CSTeamTypeFriend = CSTeamTypeFriend {
    vftable: VPtr::new(),
};
