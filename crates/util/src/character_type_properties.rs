use std::{mem::transmute, sync::{LazyLock, RwLock}};

use crate::program::Program;
use game::cs::CharacterTypePropertiesTable;
use pelite::pe::Pe;

const CHARACTER_TYPE_PROPERTIES_PATTERN: &str = concat!(
    // CS::ChrIns::CanUseRuneArc
    "80 b8 ff ? ? ? ?",
    "74 ? ",
    "48 8d 54 24 30 ",
    "48 8b cb ",
    "e8 ? ? ? ? ",
    "48 8b c8 ",
    "e8 $ { ",
    // inside CharacterTypeCanUseRuneArcs
    "48 63 01 ",
    "83 f8 16 ",
    "77 ? ",
    "48 8d 0c 80 ",
    // 48 8d 05 00 00 00 00
    "48 8d 05 $ { ' } ", // CharacterProperties[0x17]
    "0f b6 44 88 07 ",
    "c3 } ",
);

pub static CHARACTER_TYPE_PROPERTIES: LazyLock<RwLock<CharacterPropertiesHolder>> = LazyLock::new(|| {
    let program = unsafe { Program::current() };

    let mut matches = [0; 2];
    let pattern = pelite::pattern::parse(CHARACTER_TYPE_PROPERTIES_PATTERN).unwrap();
    if !program.scanner().finds_code(&pattern, &mut matches) {
        todo!("need to add error type for this");
    }

    RwLock::new(CharacterPropertiesHolder {
        table: unsafe {
            transmute::<_, _>(program.rva_to_va(matches[1]).unwrap())
        },
    })
});

#[derive(Debug)]
pub struct CharacterPropertiesHolder {
    pub table: &'static mut CharacterTypePropertiesTable,
}
