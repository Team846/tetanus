#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)] // TODO better solution to ignore std::string
#![allow(rustdoc::all)]
#![allow(clippy::all)]

pub mod ctre;
pub mod hal;
pub mod wpilib;

use std::fmt::Display;

impl Display for ctre::ctre_phoenix_ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
