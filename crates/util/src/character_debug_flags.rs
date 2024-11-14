use std::mem::transmute;
use std::sync::{LazyLock, RwLock};

use crate::program::Program;
use game::cs::CharacterDebugFlags;
use pelite::pattern;
use pelite::pattern::Atom;
use pelite::pe::Pe;

const CHARACTER_DEBUG_FLAGS_PATTERN: &[Atom] = pattern!(
    "
    ba b4 00 00 00
    48 8d 0d ? ? ? ?
    e8 ? ? ? ?
    80 3d $ { ' } 00
    0f 85 ? ? ? ?
    32 c0
    48 83 c4 20
    "
);

pub static CHARACTER_DEBUG_FLAGS: LazyLock<RwLock<&mut CharacterDebugFlags>> =
    LazyLock::new(|| {
        let program = unsafe { Program::current() };

        let mut matches = [0; 2];

        if !program
            .scanner()
            .finds_code(&CHARACTER_DEBUG_FLAGS_PATTERN, &mut matches)
        {
            todo!("need to add error type for this");
        }

        tracing::debug!("Found character properties pattern");
        RwLock::new(unsafe { transmute::<_, _>(program.rva_to_va(matches[1]).unwrap()) })
    });
