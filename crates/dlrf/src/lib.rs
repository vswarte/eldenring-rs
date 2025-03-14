/// Used to denote DLRF reflection names for singletons.
pub trait DLRFSingleton {
    const DLRF_NAME: &'static str;
}

pub use eldenring_dlrf_derive::singleton;
