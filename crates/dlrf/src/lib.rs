/// Used to denote DLRF reflection names for singletons.
pub trait DLRFSingleton {
    const DLRF_NAME: &'static str;
}

#[allow(unused)]
pub use dlrf_derive::singleton;
