use std::error::Error;

use pelite::pattern::Atom;
use pelite::pe64::Pe;

use crate::program::Program;

const CODE_RESTORATION_PATTERN: &[Atom] =
    pelite::pattern!("B9 ? ? ? ? E8 ? ? ? ? F3 0F 11 05 ? ? ? ? [0-128] ' 72 ? 48 8D ? ? ? ? ?");

/// Returns the RVAs of the arxan code restoration routines.
/// This is useful for hooking the memory image of the game.
///
/// You probably need to change the protection of the memory page to PAGE_EXECUTE_READWRITE
/// because there is no guarantee that arxan will change it before this function is called.
pub fn get_arxan_code_restoration_rvas(program: &Program) -> Vec<u32> {
    let mut result = Vec::new();
    let mut matches = program.scanner().matches_code(CODE_RESTORATION_PATTERN);
    let mut captures: [u32; 2] = [0; 2];

    while matches.next(&mut captures) {
        result.push(captures[1]);
    }
    result
}

/// Disables the arxan code restoration routine at the given RVA.
pub unsafe fn disable_code_restoration_at(
    program: &Program,
    rva: u32,
) -> Result<(), Box<dyn Error>> {
    let jb_ptr = program.rva_to_va(rva)? as *mut u8;

    tracing::debug!("Disabling code restoration at {:#x}", jb_ptr as usize);

    std::ptr::write(jb_ptr, 0xEB);

    Ok(())
}

/// Disables most instances of the arxan code restoration routines.
///
/// Avoid using this unless you absolutely have to hook the games memory image and you are
/// absolutely certain the game is removing your hooks. Prefer using the task runtime over hooking
/// the memory image where ever you can.
pub unsafe fn disable_code_restoration(program: &Program) -> Result<(), Box<dyn Error>> {
    let rvas = get_arxan_code_restoration_rvas(program);
    for rva in rvas {
        disable_code_restoration_at(program, rva)?;
    }
    Ok(())
}
