// TODO: replace with pelite
use std::{ops, slice};

use broadsword::runtime;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SectionLookupError {
    #[error("Could not locate game base")]
    NoGameBase,
    #[error("Could not locate exe section")]
    SectionNotFound,
}

pub fn get_section(
    section: &str,
) -> Result<(ops::Range<usize>, &[u8]), SectionLookupError> {
    let module = get_game_module()
        .ok_or(SectionLookupError::NoGameBase)?;

    let section_range = runtime::get_module_section_range(module, section)
        .map_err(|_| SectionLookupError::SectionNotFound)?;

    let program = unsafe { Program::current() };
    let section = program.section_headers()
        .by_name(section)
        .ok_or(|| SectionLookupError::SectionNotFound);

    let section_slice = unsafe {
        slice::from_raw_parts(
            section_range.start as *const u8,
            section_range.end - section_range.start
        )
    };

    Ok((section_range, section_slice))
}

/// Attempts to figure out what people called the exe
fn get_game_module() -> Option<&'static str> {
    const MODULE_NAMES: [&str; 2] = [
        "eldenring.exe",
        "start_protected_game.exe",
    ];

    for name in MODULE_NAMES.iter() {
        if runtime::get_module_handle(name).is_ok() {
            return Some(name)
        }
    }

    None
}

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
