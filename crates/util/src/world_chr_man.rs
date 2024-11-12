use game::cs::{ChrIns, FieldInsHandle, WorldChrMan};

pub trait WorldChrManExt {
    fn chr_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut ChrIns>;
}

impl WorldChrManExt for WorldChrMan {
    fn chr_ins_by_handle(&mut self, handle: &FieldInsHandle) -> Option<&mut ChrIns> {
        let chr_set_index = handle.selector.container() as usize;
        let chr_set = self.chr_sets.get_mut(chr_set_index)?.as_mut()?;

        chr_set.chr_ins_by_handle(handle)
    }
}
