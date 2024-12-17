use vtable_rs::VPtr;

#[repr(C)]
pub struct CSTeamTypeBase {
    vftable: VPtr<dyn CSTeamTypeVmt, Self>,
}

#[vtable_rs::vtable]
pub trait CSTeamTypeVmt {
    fn validate(&self, team_relation: &TeamRelationTargetInfo, self_target: bool) -> bool;
}

pub struct TeamRelationTargetInfo {
    pub oppose_target: bool,
    pub friendly_target: bool,
    pub self_target: bool,
}

pub struct CSTeamTypeNeutral {
    base: CSTeamTypeBase,
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
        return false;
    }
}

pub struct CSTeamTypeFriend {
    base: CSTeamTypeBase,
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
        return team_relation.friendly_target;
    }
}

pub struct CSTeamTypeEnemy {
    base: CSTeamTypeBase,
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
        return team_relation.oppose_target;
    }
}

pub struct CSTeamTypeRival {
    base: CSTeamTypeBase,
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
        if (team_relation.oppose_target == false) && (team_relation.friendly_target == false) {
            return false;
        }
        return true;
    }
}
