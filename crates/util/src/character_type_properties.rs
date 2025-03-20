use std::{
    mem::transmute,
    sync::{LazyLock, RwLock},
};

use crate::program::Program;
use game::cs::CharacterTypePropertiesTable;
use pelite::pattern;
use pelite::pattern::Atom;
use pelite::pe::Pe;

const CHARACTER_TYPE_PROPERTIES_PATTERN: &[Atom] = pattern!(
    "
    80 b8 ff ? ? ? ?
    74 ?
    48 8d 54 24 30
    48 8b cb
    e8 ? ? ? ?
    48 8b c8
    e8 $ {
    48 63 01
    83 f8 16
    77 ?
    48 8d 0c 80
    48 8d 05 $ { ' }
    0f b6 44 88 07
    c3 }
    "
);

pub static CHARACTER_TYPE_PROPERTIES: LazyLock<RwLock<CharacterPropertiesHolder>> =
    LazyLock::new(|| {
        let program = Program::current();

        let mut matches = [0; 2];
        if !program
            .scanner()
            .finds_code(CHARACTER_TYPE_PROPERTIES_PATTERN, &mut matches)
        {
            panic!("Failed to find character properties pattern");
        }

        tracing::debug!("Found character properties pattern");
        RwLock::new(CharacterPropertiesHolder {
            table: unsafe { transmute::<_, _>(program.rva_to_va(matches[1]).unwrap()) },
        })
    });

pub struct CharacterPropertiesHolder {
    pub table: &'static mut CharacterTypePropertiesTable,
}
