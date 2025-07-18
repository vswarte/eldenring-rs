use eldenring::{
    cs::CSEzDraw,
    position::{HavokPosition, PositionDelta},
};
use pelite::pe64::Pe;
use shared::FSVector4;

use crate::{
    program::Program,
    rva::{
        RVA_CS_EZ_DRAW_DRAW_CAPSULE, RVA_CS_EZ_DRAW_DRAW_LINE, RVA_CS_EZ_DRAW_DRAW_SPHERE,
        RVA_CS_EZ_DRAW_DRAW_WEDGE, RVA_CS_EZ_DRAW_SET_COLOR,
    },
};

pub trait CSEzDrawExt {
    /// Set the color for the to-be-rendered primitives.
    fn set_color(&self, color: &FSVector4);

    fn draw_line(&self, from: &HavokPosition, to: &HavokPosition);

    fn draw_capsule(&self, top: &HavokPosition, bottom: &HavokPosition, radius: f32);

    fn draw_sphere(&self, origin: &HavokPosition, radius: f32);

    fn draw_wedge(
        &self,
        origin: &HavokPosition,
        direction: &PositionDelta,
        inner_length: f32,
        outer_length: f32,
        degrees: f32,
    );
}

type FnSetColor = extern "C" fn(*const CSEzDraw, *const FSVector4);
type FnDrawLine = extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition);
type FnDrawCapsule =
    extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition, f32);
type FnDrawSphere = extern "C" fn(*const CSEzDraw, *const HavokPosition, f32);
type FnDrawFan =
    extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition, f32, f32, f32);

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

    fn draw_capsule(&self, top: &HavokPosition, bottom: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawCapsule>(
                Program::current()
                    .rva_to_va(RVA_CS_EZ_DRAW_DRAW_CAPSULE)
                    .unwrap(),
            )
        };

        target(self, top, bottom, radius);
    }

    fn draw_sphere(&self, origin: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawSphere>(
                Program::current()
                    .rva_to_va(RVA_CS_EZ_DRAW_DRAW_SPHERE)
                    .unwrap(),
            )
        };

        target(self, origin, radius);
    }

    fn draw_wedge(
        &self,
        origin: &HavokPosition,
        direction: &PositionDelta,
        inner_length: f32,
        outer_length: f32,
        degrees: f32,
    ) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawFan>(
                Program::current()
                    .rva_to_va(RVA_CS_EZ_DRAW_DRAW_WEDGE)
                    .unwrap(),
            )
        };

        let direction = HavokPosition(direction.0, direction.1, direction.2, 0.0);

        target(
            self,
            origin,
            &direction,
            inner_length,
            outer_length,
            degrees,
        );
    }
}
