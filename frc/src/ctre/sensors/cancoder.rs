use crate::ctre as frc_ctre;
use frc_sys::ctre::{ctre_phoenix_ErrorCode, ctre_phoenix_sensors_CANCoder};

use super::{
    AbsoluteSensorRange, MagnetFieldStrength, SensorInitializationStrategy, SensorTimeBase,
    SensorVelocityMeasPeriod,
};

use anyhow::Result;
use uom::si::angle::degree;
use uom::si::electric_potential::volt;
use uom::si::f64::*;

pub struct CANCoder {
    handle: ctre_phoenix_sensors_CANCoder,
}

impl CANCoder {
    pub fn new(device_number: i32) -> Self {
        CANCoder {
            handle: unsafe { ctre_phoenix_sensors_CANCoder::new(device_number) },
        }
    }

    pub fn get_position(&mut self) -> f64 {
        unsafe { self.handle.GetPosition() }
    }

    pub fn get_velocity(&mut self) -> f64 {
        unsafe { self.handle.GetVelocity() }
    }

    pub fn set_position(&mut self, new_position: f64, timeout_ms: i32 /* 0 */) -> Result<()> {
        frc_ctre::to_result(unsafe { self.handle.SetPosition(new_position, timeout_ms) })
    }

    pub fn set_position_to_absolute(&mut self, timeout_ms: i32) -> Result<()> {
        frc_ctre::to_result(unsafe { self.handle.SetPositionToAbsolute(timeout_ms) })
    }

    pub fn get_absolute_position(&mut self) -> f64 {
        unsafe { self.handle.GetAbsolutePosition() }
    }

    pub fn config_velocity_measurement_period(
        &mut self,
        period: SensorVelocityMeasPeriod,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigVelocityMeasurementPeriod(period, timeout_ms)
        })
    }

    pub fn config_velocity_measurement_window(
        &mut self,
        window_size: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigVelocityMeasurementWindow(window_size, timeout_ms)
        })
    }

    pub fn config_absolute_sensor_range(
        &mut self,
        absolute_sensor_range: AbsoluteSensorRange,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigAbsoluteSensorRange(absolute_sensor_range, timeout_ms)
        })
    }

    pub fn config_magnet_offset(
        &mut self,
        offset: Angle,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigMagnetOffset(offset.get::<degree>(), timeout_ms)
        })
    }

    pub fn config_sensor_initialization_strategy(
        &mut self,
        initialization_strategy: SensorInitializationStrategy,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigSensorInitializationStrategy(initialization_strategy, timeout_ms)
        })
    }

    pub fn config_feedback_coefficient(
        &mut self,
        _sensor_coefficient: f64,
        _unit_string: &str,
        _sensortime_base: SensorTimeBase,
        _timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        unimplemented!()
    }

    pub fn get_bus_voltage(&mut self) -> ElectricPotential {
        unsafe { ElectricPotential::new::<volt>(self.handle.GetBusVoltage()) }
    }

    pub fn get_magnet_field_strength(&mut self) -> MagnetFieldStrength {
        unsafe { self.handle.GetMagnetFieldStrength() }
    }

    pub fn config_sensor_direction(
        &mut self,
        b_sensor_direction: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigSensorDirection(b_sensor_direction, timeout_ms)
        })
    }

    pub fn get_last_error(&mut self) -> ctre_phoenix_ErrorCode {
        unsafe { self.handle.GetLastError() }
    }

    pub fn get_last_unit_string(&mut self) -> String {
        unimplemented!()
    }

    pub fn get_last_timestamp(&mut self) -> f64 {
        unsafe { self.handle.GetLastTimestamp() }
    }

    pub fn config_set_custom_param(
        &mut self,
        new_value: i32,
        param_index: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigSetCustomParam(new_value, param_index, timeout_ms)
        })
    }

    pub fn config_get_custom_param(
        &mut self,
        param_index: i32,
        timeout_ms: i32, /* 0 */
    ) -> i32 {
        unsafe { self.handle.ConfigGetCustomParam(param_index, timeout_ms) }
    }
}
