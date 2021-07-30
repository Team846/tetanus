//! Rust bindings to ctre

use anyhow::{anyhow, Result};
use frc_sys::ctre::ctre_phoenix_ErrorCode;

pub mod motorcontrol;
pub mod sensors;

pub(crate) fn to_result(err: ctre_phoenix_ErrorCode) -> Result<()> {
    match err {
        ctre_phoenix_ErrorCode::OK => Ok(()),
        e => Err(anyhow!(e)),
    }
}
