use dlrf::DLRFSingleton;
use pelite::pattern;
use pelite::pattern::Atom;
use pelite::pe64::{Pe, Rva};
use std::collections;
use std::sync;
use thiserror::Error;

use crate::program::Program;

pub type SingletonMap = collections::HashMap<String, usize>;
static SINGLETON_MAP: sync::OnceLock<SingletonMap> = sync::OnceLock::new();

#[derive(Error, Debug)]
pub enum SingletonMapError {
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

/// Looks up instances of singleton instances by their name.
/// Some singletons aren't necessarily always instanciated and available.
/// Discovered singletons are cached so invokes after the first will be much faster.
///
/// # Safety
/// User must ensure that:
///  - The main module (the exe) is a From Software title with DLRF reflection data.
///  - The DLRF reflection metadata has been populated (wait_for_system_init).
///  - Access to the singleton is exclusive (either by hooking or utilizing the task system).
///  - get_instance is not called multiple times such that it spawns multiple mutable references to the same singleton.
pub unsafe fn get_instance<T: DLRFSingleton>() -> Result<Option<&'static mut T>, LookupError> {
    let table = SINGLETON_MAP.get_or_init(|| {
        build_singleton_table(&Program::current())
            .map_err(LookupError::SingletonMapCreation)
            .expect("Could not create singleton map")
    });

    let ptr = table
        .get(T::DLRF_NAME)
        .map(usize::to_owned)
        .ok_or(LookupError::NotFound)?;

    unsafe { Ok((*(ptr as *const *mut T)).as_mut()) }
}

// MOV REG, [MEM]
// TEST REG, REG
// JNZ +2e
// LEA RCX, [runtime_class_metadata]
// CALL get_singleton_name
//
// OR
//
// MOV REG, [MEM]
// MOV REG, REG
// TEST REG, REG
// JNZ +2e
// LEA RCX, [runtime_class_metadata]
// CALL get_singleton_name
//
// OR
//
// MOV REG, [MEM]
// TEST REG, REG
// JNZ +2e
// LEA R9, [s_SingletonName]
// MOV REG, 0xaa
// const NULL_CHECK_PATTERN: &[Atom] = pattern!(
//     "
//     48 8b ? $ { ' }
//     (
//         48 85 ?
//         |
//         48 8b ?
//         48 85 ?
//     )
//     75 ?
//     4c 8d 0d $ { ' }
//     (
//         e8 $ { ' }
//         |
//         ba u4
//     )
//     "
// );
const NULL_CHECK_PATTERN: &[Atom] = pattern!(
    "
    48 8b ? $ { ' }
    (
        48 85 ?
        |
        48 8b ?
        48 85 ?
    )
    75 ?
    (4c|48) 8d 0d $ { ' }
    (
        e8 $ { ' }
        |
        ba u4
    )
    "
);

/// Builds a table of all the singletons. It does so by looking for null checks
/// in the game by using an instance pattern. It then cycles over all
/// candidates and vets the involved pointers. We expect a pointer to the
/// instance's static, a pointer to the reflection metadata and a pointer to
/// the get_singleton_name fn. Some singletons don't have reflection metadata and
/// use pointers to the singleton name instead. In this case we will check for
/// 3rd match being empty. Once all checks out we call get_singleton_name
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

    let rdata_range = program
        .section_headers()
        .by_name(".rdata")
        .ok_or(SingletonMapError::Section(".rdata"))?
        .virtual_range();

    let mut matches = program.scanner().matches_code(NULL_CHECK_PATTERN);
    let mut captures: [Rva; 4] = [Rva::default(); 4];
    let mut results: SingletonMap = Default::default();

    while matches.next(&mut captures) {
        let static_rva = captures[1];
        let metadata_rva = captures[2];
        let get_reflection_name_rva = captures[3];

        // in the case of second pattern type, the third capture is error code and for singletons they are 0xa0 or 0xaa
        let is_simple_singleton =
            get_reflection_name_rva == 0xa0 || get_reflection_name_rva == 0xaa;

        // Basic checks: static_rva and metadata_rva must be in .data section
        // Unless it's a pointer to the singleton name, in which case it must be in .rdata section
        if !data_range.contains(&static_rva)
            || (is_simple_singleton && !rdata_range.contains(&metadata_rva))
            || (!is_simple_singleton && !data_range.contains(&metadata_rva))
        {
            continue;
        }

        let metadata = program.rva_to_va(metadata_rva).unwrap();

        let name_ptr: *const i8 = if !is_simple_singleton {
            // Case 1: Standard singleton with metadata and get_singleton_name function call
            // get_reflection_name_rva must point to executable code in .text section.
            if !text_range.contains(&get_reflection_name_rva) {
                continue;
            }

            let get_singleton_name: extern "C" fn(u64) -> *const i8 =
                unsafe { std::mem::transmute(program.rva_to_va(get_reflection_name_rva).unwrap()) };

            // metadata is the VA of the metadata struct in this branch
            let name_ptr = get_singleton_name(metadata);

            if name_ptr.is_null() {
                // If the function returns a null pointer, we cannot get the name.
                continue;
            }
            name_ptr
        } else {
            // Case 2: Singleton name is a direct string pointer (LEA R9, [s_SingletonName])
            // metadata is the VA of the null-terminated string.
            let name_ptr = metadata as *const i8;
            if name_ptr.is_null() {
                continue;
            }
            name_ptr
        };

        // Convert the C string pointer to a Rust String.
        // This unsafe block relies on name_cstr_ptr being non-null (ensured by checks above)
        // and pointing to a valid, null-terminated C string.
        let cstr = unsafe { std::ffi::CStr::from_ptr(name_ptr) };
        let singleton_name = cstr
            .to_str()
            .map_err(|_| SingletonMapError::MalformedName)?
            .to_string();

        let singleton_va = program.rva_to_va(static_rva).unwrap();
        results.insert(singleton_name, singleton_va as usize);
    }

    Ok(results)
}
