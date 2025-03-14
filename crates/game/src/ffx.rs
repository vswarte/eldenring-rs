use vtable_rs::VPtr;

#[vtable_rs::vtable]
pub trait FXAppearanceVmt {
    fn destructor(&mut self);

    fn unk2(&self);
    fn unk3(&self);
    fn unk4(&self);
    fn unk5(&self);
    fn unk6(&self);
    fn unk7(&self);
    fn unk8(&self);
    fn unk9(&self);
    fn unk10(&self);
    fn unk11(&self);
    fn unk12(&self);
}

#[repr(C)]
/// Source of name: RTTI
pub struct FXAppearanceBase {
    pub vftable: VPtr<dyn FXAppearanceVmt, Self>,
    unk8: usize,
}
