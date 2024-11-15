use crate::fd4::{FD4ResCap, FD4ResRep};

#[repr(C)]
/// Manages the event flags for the game.
///
/// Source of name: RTTI
#[dlrf::singleton("MsbRepository")]
pub struct MsbRepository {
    pub res_rep: FD4ResRep<()>,
}
