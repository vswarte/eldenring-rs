#![feature(once_cell_get_mut)]

mod stl;
pub use stl::*;

pub mod cs;
pub mod dl;
pub mod fd4;
pub mod matrix;
pub mod position;
pub mod pointer;

pub trait DLRFLocatable {
    const DLRF_NAME: &'static str;
}
