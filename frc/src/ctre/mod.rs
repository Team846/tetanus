//! Rust bindings to ctre

use frc_sys::ctre::ctre_phoenix_ErrorCode;

pub mod motorcontrol;
pub mod sensors;

pub type ErrorCode = ctre_phoenix_ErrorCode;

pub(crate) fn to_result(err: ctre_phoenix_ErrorCode) -> Result<(), ErrorCode> {
    match err {
        ErrorCode::OK => Ok(()),
        _ => Err(err),
    }
}
