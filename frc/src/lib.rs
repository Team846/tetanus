//! Rust bindings to frc libraries (wpilib, hal, ctre)

#![allow(rustdoc::broken_intra_doc_links)]

pub mod ctre;
pub mod wpilib;

/// Rust bindings to HAL
pub mod hal {
    // Re-export frc_sys::hal since there's no need to write friendly wrappers around HAL functions
    pub use frc_sys::hal::*;
}

pub use frc_sys;
