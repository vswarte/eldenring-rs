use std::collections::HashMap;

use pelite::pe::{Pe, Rva, Va};
use thiserror::Error;
use util::program::Program;

#[derive(Debug, Error)]
pub enum LocationProviderError {
    #[error("Could not convert RVA to VA")]
    AddressConversion(#[from] pelite::Error),
}

pub struct ProgramLocationProvider {
    program: Program<'static>,
}

impl ProgramLocationProvider {
    pub fn new() -> Self {
        Self {
            program: unsafe { Program::current() },
        }
    }

    pub fn get(&self, rva: u32) -> Result<Va, LocationProviderError> {
        Ok(self.program.rva_to_va(rva)?)
    }
}
