use broadsword::scanner;
use game::DLRFLocatable;
use pelite::pe::Pe;
use std::collections;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::Path;
use std::sync;
use thiserror::Error;

use crate::program::get_section;
use crate::program::Program;
use crate::program::SectionLookupError;

pub type SingletonMap = collections::HashMap<String, usize>;
static SINGLETON_MAP: sync::OnceLock<SingletonMap> = sync::OnceLock::new();

#[derive(Error, Debug)]
pub enum SingletonMapError {
    #[error("Could not parse pattern. {0:?}")]
    Pattern(broadsword::scanner::ParserError),
    #[error("Could not find section {0}.")]
    Section(&'static str),
    #[error("Could not parse the discovered singleton's name.")]
    MalformedName,
}

#[derive(Error, Debug)]
pub enum LookupError {
    #[error("Singleton was not found.")]
    NotFound,
    #[error("Could not create the singleton map. {0}")]
    SingletonMapCreation(SingletonMapError),
}

/// Looks up instances of singleton'd classes by their name.
/// It builds a singleton map in the by matching an instruction pattern for
/// some exception creation.
/// Some singletons aren't necessarily always alive. Hence the
/// Result<Option<T>, E>. An example of such is WorldChrMan of which an
/// instance only exists if you're actually in the game world.
pub fn get_instance<T: DLRFLocatable>() -> Result<Option<&'static mut T>, LookupError> {
    let table = SINGLETON_MAP.get_or_init(move || {
        let program = unsafe { Program::current() };

        build_singleton_table(program)
            .map_err(LookupError::SingletonMapCreation)
            .expect("Could not create singleton map")
    });

    let ptr = table
        .get(T::DLRF_NAME)
        .map(usize::to_owned)
        .ok_or(LookupError::NotFound)?;

    unsafe { Ok((*(ptr as *const *mut T)).as_mut()) }
}

const NULL_CHECK_PATTERN: &str = concat!(
    //  0 MOV REG, [MEM]
    "01001... 10001011 00...101 [........ ........ ........ ........]",
    //  7 TEST REG, REG
    "01001... 10000101 11......",
    // 10 JNZ +2e
    "01110101 ........",
    // 12 LEA RCX, [runtime_class_metadata]
    "01001... 10001101 00001101 [........ ........ ........ ........]",
    // 19 CALL get_singleton_name
    "11101000 [........ ........ ........ ........]",
);

/// Builds a table of all the singletons. It does so by looking for null checks
/// in the game by using an instance pattern. It then cycles over all
/// candidates and vets the involved pointers. We expect a pointer to the
/// instance's static, a pointer to the reflection metadata and a pointer to
/// the get_singleton_name fn. Once all checks out we call get_singleton_name
/// with the metadata to obtain the instance's type name.
pub fn build_singleton_table<'a>(program: &'a Program) -> Result<SingletonMap, SingletonMapError> {
    let (text_range, text_slice) = program
        .section_headers()
        .by_name(".text")
        .map(|s| {
            let virtual_range = s.virtual_range();

            let range = std::ops::Range {
                start: program.rva_to_va(virtual_range.start).unwrap() as usize,
                end: program.rva_to_va(virtual_range.end).unwrap() as usize,
            };

            let slice: &[u8] = program.derva_slice(s.VirtualAddress, s.VirtualSize as usize)
                .expect("Could not get slice");

            (range, slice)
        })
        .ok_or(SingletonMapError::Section(".text"))?;

    let data_range = program
        .section_headers()
        .by_name(".data")
        .map(|s| {
            let virtual_range = s.virtual_range();

            std::ops::Range {
                start: program.rva_to_va(virtual_range.start).unwrap() as usize,
                end: program.rva_to_va(virtual_range.end).unwrap() as usize,
            }
        })
        .ok_or(SingletonMapError::Section(".data"))?;

    tracing::debug!("Found sections: text_range = {text_range:x?}, data_range = {data_range:x?}");

    let pattern = scanner::Pattern::from_bit_pattern(NULL_CHECK_PATTERN)
        .map_err(SingletonMapError::Pattern)?;

    tracing::debug!("Scanning for singleton nullcheck candidates");
    let mut results: SingletonMap = Default::default();
    for candidate in scanner::simple::scan_all(text_slice, &pattern) {
        let static_offset =
            u32::from_le_bytes(candidate.captures[0].bytes.as_slice().try_into().unwrap());

        let metadata_offset =
            u32::from_le_bytes(candidate.captures[1].bytes.as_slice().try_into().unwrap());

        let fn_offset =
            u32::from_le_bytes(candidate.captures[2].bytes.as_slice().try_into().unwrap());

        let candidate_base = text_range.start + candidate.location;

        // Pointer to the instance of the singleton'd class
        let static_address = candidate_base + 7 + static_offset as usize;
        tracing::trace!("Candidate singleton static address. static_address = {static_address:x}");
        if !data_range.contains(&static_address) {
            continue;
        }

        // Pointer to the reflection metadata
        let metadata_address = candidate_base + 19 + metadata_offset as usize;
        tracing::trace!("Candidate reflection metadata. metadata_address = {metadata_address:x}");
        if !data_range.contains(&metadata_address) {
            continue;
        }

        // Pointer to the name getter fn. char* get_singleton_name(metadata)
        let fn_address = candidate_base + 24 + fn_offset as usize;
        tracing::trace!("Candidate get_singleton_name. fn_address = {fn_address:x}");
        if !text_range.contains(&fn_address) {
            continue;
        }

        let get_singleton_name: extern "C" fn(usize) -> *const i8 =
            unsafe { mem::transmute(fn_address) };

        let cstr = unsafe { std::ffi::CStr::from_ptr(get_singleton_name(metadata_address)) };

        let name = cstr
            .to_str()
            .map_err(|_| SingletonMapError::MalformedName)?
            .to_string();

        tracing::trace!("Candidate name. name = {name}");

        results.insert(name, static_address);
    }

    tracing::info!("Built singleton table. results.len() = {}", results.len());

    Ok(results)
}
