use game::{cs::CSCam, position::HavokPosition};

pub trait CSCamExt {
    fn position(&self) -> HavokPosition;
}

impl CSCamExt for CSCam {
    fn position(&self) -> HavokPosition {
        self.matrix.3.into()
    }
}
