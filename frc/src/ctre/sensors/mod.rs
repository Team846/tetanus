mod cancoder;
pub use cancoder::*;
use frc_sys::ctre::{
    ctre_phoenix_sensors_AbsoluteSensorRange, ctre_phoenix_sensors_MagnetFieldStrength,
    ctre_phoenix_sensors_SensorInitializationStrategy, ctre_phoenix_sensors_SensorTimeBase,
    ctre_phoenix_sensors_SensorVelocityMeasPeriod,
};

pub type AbsoluteSensorRange = ctre_phoenix_sensors_AbsoluteSensorRange;
pub type MagnetFieldStrength = ctre_phoenix_sensors_MagnetFieldStrength;
pub type SensorInitializationStrategy = ctre_phoenix_sensors_SensorInitializationStrategy;
pub type SensorTimeBase = ctre_phoenix_sensors_SensorTimeBase;
pub type SensorVelocityMeasPeriod = ctre_phoenix_sensors_SensorVelocityMeasPeriod;
