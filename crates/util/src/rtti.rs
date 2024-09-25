use pelite::pe::{
    msvc::RTTICompleteObjectLocator,
    Pe, Rva, Va,
};
use undname::Flags;

use crate::program::Program;

// TODO: this cast to u32 is probably not going to cause panics but can be prettier.
const VA_SIZE: u32 = size_of::<Va>() as u32;

/// Builds an iterator that walks over the entire .rdata section looking for
/// recoverable classes.
pub fn find_rtti_classes<'a>(program: &'a Program) -> impl Iterator<Item = Class<'a>> + 'a {
    let text = program
        .section_headers()
        .by_name(".text")
        .expect("no .text section found");

    let rdata = program
        .section_headers()
        .by_name(".rdata")
        .expect("no .rdata section found");

    rdata
        .virtual_range()
        .step_by(size_of::<Va>())
        .filter_map(move |candidate_rva| {
            let vtable_meta_rva = candidate_rva;
            let vtable_rva = candidate_rva + VA_SIZE;

            let vtable_meta_rva = program
                .derva(vtable_meta_rva)
                .and_then(|va| program.va_to_rva(*va))
                .ok()?;

            let vtable_entry_rva = program
                .derva(vtable_rva)
                .and_then(|va| program.va_to_rva(*va))
                .ok()?;

            if rdata.virtual_range().contains(&vtable_meta_rva)
                && text.virtual_range().contains(&vtable_entry_rva)
            {
                let _: &RTTICompleteObjectLocator = program.derva(vtable_meta_rva).ok()?;

                Some((vtable_meta_rva, vtable_rva))
            } else {
                None
            }
        })
        .filter_map(|(meta, vtable)| {
            let col: &RTTICompleteObjectLocator = program.derva(meta).ok()?;

            let ty_name = program
                .derva_c_str(col.type_descriptor + 16)
                .ok()?
                .to_string();
            if !ty_name
                .chars()
                .all(|ch| (0x20..=0x7e).contains(&(ch as u8)))
            {
                return None;
            }

            let demangled = undname::demangle(ty_name.as_str(), Flags::NAME_ONLY)
                .map(|s| s.to_string())
                .ok()?;

            Some(Class {
                program,
                name: demangled,
                vtable,
            })
        })
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct RttiCandidate {
    vtable_meta_rva: Rva,
    vtable_rva: Rva,
}

pub struct Class<'a> {
    program: &'a Program<'a>,
    pub name: String,
    pub vtable: Rva,
}

impl Class<'_> {
    /// Retrieves the function pointer from the VMT.
    ///
    /// # Safety
    /// Does not validate if the index is actually contained within the VMT.
    pub unsafe fn vmt_index(&self, index: u32) -> Option<Va> {
        Some(*self.program.derva(self.vtable + VA_SIZE * index).ok()?)
    }
}
