use game::{cs::CSEzDraw, matrix::FSVector4, position::HavokPosition};
use pelite::pe::Pe;

use crate::{
    program::Program,
    rva::{RVA_CS_EZ_DRAW_DRAW_LINE, RVA_CS_EZ_DRAW_SET_COLOR},
};

pub trait CSEzDrawExt {
    /// Draw line using havok pos.
    fn draw_line(&self, from: &HavokPosition, to: &HavokPosition);

    /// Set the color for the to-be-rendered primitives.
    fn set_color(&self, color: &FSVector4);
}

type FnDrawLine = extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition);
type FnSetColor = extern "C" fn(*const CSEzDraw, *const FSVector4);

impl CSEzDrawExt for CSEzDraw {
    fn set_color(&self, color: &FSVector4) {
        let target = unsafe {
            std::mem::transmute::<u64, FnSetColor>(
                Program::current()
                    .rva_to_va(RVA_CS_EZ_DRAW_SET_COLOR)
                    .unwrap(),
            )
        };

        target(self, color);
    }

    fn draw_line(&self, from: &HavokPosition, to: &HavokPosition) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawLine>(
                Program::current()
                    .rva_to_va(RVA_CS_EZ_DRAW_DRAW_LINE)
                    .unwrap(),
            )
        };

        target(self, from, to);
    }
}
