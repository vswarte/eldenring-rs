use vtable_rs::VPtr;

#[repr(C)]
pub struct CSTeamTypeBase {
    vftable: VPtr<dyn CSTeamTypeVmt, Self>,
}

#[vtable_rs::vtable]
pub trait CSTeamTypeVmt {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool;
}

impl CSTeamTypeVmt for CSTeamTypeBase {
    extern "C" fn validate(
        &self,
        team_relation: &TeamRelationTargetInfo,
        self_target: bool,
    ) -> bool {
        unimplemented!("CSTeamTypeBase should not be used directly");
    }
}

pub struct TeamRelationTargetInfo {
    pub oppose_target: bool,
    pub friendly_target: bool,
    pub self_target: bool,
}
