use std::error::Error;

use pelite::pe::Pe;
use pelite::pattern::Atom;

use crate::program::Program;

const CODE_RESTORATION_PATTERN: &[Atom] = pelite::pattern!("B9 ? ? ? ? E8 ? ? ? ? F3 0F 11 05 ? ? ? ? [0-128] ' 72 ? 48 8D ? ? ? ? ?");

/// Disables most instances of the arxan code restoration routines.
///
/// Avoid using this unless you absolutely have to hook the games memory image and you are
/// absolutely certain the game is removing your hooks. Prefer using the task runtime over hooking
/// the memory image where ever you can.
pub unsafe fn disable_code_restoration(program: &Program) -> Result<(), Box<dyn Error>> {
    let mut matches = program.scanner().matches_code(CODE_RESTORATION_PATTERN);
    let mut captures: [u32; 2] = [0; 2];

    while matches.next(&mut captures) {
        let jb_ptr = program.rva_to_va(captures[1])? as *mut u8;
        *jb_ptr = 0xEB;
    }

    Ok(())
}
