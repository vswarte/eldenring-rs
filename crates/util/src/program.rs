// TODO: replace with pelite

use pelite::pe::{Pe, PeFile, PeObject, PeView};
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

#[derive(Copy, Clone)]
pub enum Program<'a> {
    File(PeFile<'a>),
    Mapping(PeView<'a>)
}

impl Program<'_> {
    /// Returns the currently running programing.
    pub unsafe fn current() -> Self {
        let module = GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 as *const u8;
        Program::Mapping(PeView::module(module))
    }
}

unsafe impl<'a> Pe<'a> for Program<'a> {}
unsafe impl<'a> PeObject<'a> for Program<'a> {
    fn image(&self) -> &'a [u8] {
        match self {
            Self::File(file) => file.image(),
            Self::Mapping(mapping) => mapping.image(),
        }
    }

    fn align(&self) -> pelite::Align {
        match self {
            Self::File(file) => file.align(),
            Self::Mapping(mapping) => mapping.align(),
        }
    }
}
