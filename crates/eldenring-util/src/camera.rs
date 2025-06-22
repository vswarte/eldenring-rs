use eldenring::{cs::CSCam, position::HavokPosition};

pub trait CSCamExt {
    fn position(&self) -> HavokPosition;
}

impl CSCamExt for CSCam {
    fn position(&self) -> HavokPosition {
        HavokPosition(
            self.matrix.3 .0,
            self.matrix.3 .1,
            self.matrix.3 .2,
            self.matrix.3 .3,
        )
    }
}
