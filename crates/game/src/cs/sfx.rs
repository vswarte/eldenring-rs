use crate::gxffx::GXFfxSceneCtrl;
use crate::matrix::FSMatrix4x4;
use crate::pointer::OwnedPtr;
use crate::{Tree, Vector};

#[repr(C)]
/// Source of name: RTTI
#[dlrf::singleton("CSSfx")]
pub struct CSSfxImp {
    vftable: usize,
    unk8: usize,
    unk10: Tree<()>,
    unk28: Tree<()>,
    unk40: usize,
    unk48: usize,
    unk50: usize,
    unk58: usize,
    pub scene_ctrl: OwnedPtr<GXFfxSceneCtrl>,
}
