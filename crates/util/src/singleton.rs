use game::DLRFLocatable;
use pelite::pattern;
use pelite::pe::Pe;
use pelite::pe::Rva;
use std::collections;
use std::io::Write;
use std::sync;
use thiserror::Error;

use crate::program::Program;

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

        build_singleton_table(&program)
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
    "48 8b 0d $ { ' }",
    //  7 TEST REG, REG
    "48 85 ? ",
    // 10 JNZ +2e
    "75 ? ",
    // 12 LEA RCX, [runtime_class_metadata]
    "48 8d 0d $ { ' }",
    // 19 CALL get_singleton_name
    "e8 $ { ' }"
);

/// Builds a table of all the singletons. It does so by looking for null checks
/// in the game by using an instance pattern. It then cycles over all
/// candidates and vets the involved pointers. We expect a pointer to the
/// instance's static, a pointer to the reflection metadata and a pointer to
/// the get_singleton_name fn. Once all checks out we call get_singleton_name
/// with the metadata to obtain the instance's type name.
pub fn build_singleton_table(program: &Program) -> Result<SingletonMap, SingletonMapError> {
    let text_range = program
        .section_headers()
        .by_name(".text")
        .ok_or(SingletonMapError::Section(".text"))?
        .virtual_range();

    let data_range = program
        .section_headers()
        .by_name(".data")
        .ok_or(SingletonMapError::Section(".data"))?
        .virtual_range();

    tracing::info!("Found sections. text_range = {text_range:x?}, data_range = {data_range:x?}");

    let pattern = pattern::parse(NULL_CHECK_PATTERN).unwrap();
    let mut matches = program.scanner().matches_code(&pattern);
    let mut captures: [Rva; 4] = [Rva::default(); 4];
    let mut results: SingletonMap = Default::default();

    tracing::debug!("Scanning for singleton nullcheck candidates");
    while matches.next(&mut captures) {
        tracing::trace!("Singleton nullcheck candidate. captures = {captures:#x?}");

        let static_rva = captures[1];
        let metadata_rva = captures[2];
        let get_reflection_name_rva = captures[3];

        // Check if all RVAs are plausible.
        if !data_range.contains(&static_rva)
            || !data_range.contains(&metadata_rva)
            || !text_range.contains(&get_reflection_name_rva)
        {
            continue;
        }

        tracing::trace!(
            "Found singleton null check candidate. {} {} {}",
            data_range.contains(&static_rva),
            data_range.contains(&metadata_rva),
            text_range.contains(&get_reflection_name_rva),
        );

        let metadata = program.rva_to_va(metadata_rva).unwrap();
        let get_singleton_name: extern "C" fn(u64) -> *const i8 = unsafe {
            std::mem::transmute(program.rva_to_va(get_reflection_name_rva).unwrap())
        };

        let cstr = unsafe { std::ffi::CStr::from_ptr(get_singleton_name(metadata)) };
        let singleton_name = cstr
            .to_str()
            .map_err(|_| SingletonMapError::MalformedName)?
            .to_string();

        let singleton_va = program.rva_to_va(static_rva).unwrap();
        tracing::debug!("Discovered singleton {} at {:x}", singleton_name, singleton_va);

        results.insert(singleton_name, singleton_va as usize);
    }

    Ok(results)
}
