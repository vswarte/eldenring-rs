use eldenring::cs::{CSGaitemImp, CSGaitemIns, GaitemHandle};

pub trait CSGaitemImpExt {
    fn gaitem_ins_by_handle(&self, handle: &GaitemHandle) -> Option<&CSGaitemIns>;

    fn gaitem_ins_by_handle_mut(&mut self, handle: &GaitemHandle) -> Option<&mut CSGaitemIns>;
}

impl CSGaitemImpExt for CSGaitemImp {
    fn gaitem_ins_by_handle(&self, handle: &GaitemHandle) -> Option<&CSGaitemIns> {
        // Can't do a lookup for a handle that is not supposed to be in here anyway.
        if !handle.is_indexed() {
            return None;
        }

        let index = handle.index() as usize;
        if index > self.gaitems.len() {
            return None;
        }

        Some(self.gaitems[index].as_ref()?.as_ref())
    }

    fn gaitem_ins_by_handle_mut(&mut self, handle: &GaitemHandle) -> Option<&mut CSGaitemIns> {
        // Can't do a lookup for a handle that is not supposed to be in here anyway.
        if !handle.is_indexed() {
            return None;
        }

        let index = handle.index() as usize;
        if index > self.gaitems.len() {
            return None;
        }

        Some(self.gaitems[index].as_mut()?.as_mut())
    }
}
