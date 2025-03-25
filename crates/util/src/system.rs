use std::sync;
use std::sync::atomic::AtomicPtr;

use pelite::pattern;
use pelite::pattern::Atom;
use pelite::pe64::{Pe, PeView, Rva};
use thiserror::Error;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

// WinMain -> SetBaseAddr
// used to set base executable address for CSWindowImp
// and can be used to determine if the game has finished initializing
const GLOBAL_INIT_BASE_ADDR_PATTERN: &[Atom] = pattern!(
    "
    48 8b ce
    48 8b f8
    e8 $ {
        48 89 0d $ { ' }
        c3
    }
    "
);

static GLOBAL_INIT_BASE_ADDR: sync::OnceLock<AtomicPtr<usize>> = sync::OnceLock::new();

#[derive(Error, Debug)]
pub enum SystemInitError {
    #[error("System initialization timed out after {0}ms")]
    Timeout(i32),
}
/// Wait for the system to finish initializing.
/// returns an error if the system initialization timed out.
/// -1 for no timeout will wait indefinitely.
/// https://github.com/Dasaav-dsv/libER/blob/main/source/dantelion2/system.cpp
pub fn wait_for_system_init(timeout: i32) -> Result<(), SystemInitError> {
    let global_init_flip_counter = GLOBAL_INIT_BASE_ADDR.get_or_init(|| {
        let module = unsafe {
            let handle = GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 as *const u8;
            PeView::module(handle)
        };

        let mut captures = [Rva::default(); 2];
        module
            .scanner()
            .finds_code(GLOBAL_INIT_BASE_ADDR_PATTERN, &mut captures);

        let global_init_flip_counter = module.rva_to_va(captures[1]).unwrap();
        (global_init_flip_counter as *mut usize).into()
    });

    let counter = global_init_flip_counter.load(std::sync::atomic::Ordering::Relaxed);

    if timeout >= 0 {
        let start = std::time::Instant::now();
        let timeout_duration = std::time::Duration::from_millis(timeout as u64);

        while unsafe { *counter } == 0 {
            if start.elapsed() > timeout_duration {
                return Err(SystemInitError::Timeout(timeout));
            }
            std::thread::yield_now();
        }
    } else {
        while unsafe { *counter } == 0 {
            std::thread::yield_now();
        }
    }

    Ok(())
}
