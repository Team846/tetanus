use core::panic;

use anyhow::Result;
use frc::ctre::motorcontrol::can::{BaseMotorController, TalonFX};
use frc::ctre::motorcontrol::{
    NeutralMode, StatorCurrentLimitConfiguration, SupplyCurrentLimitConfiguration,
    TalonFXControlMode,
};
use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::f64::*;
use uom::si::time::second;

pub const CONFIG_TIMEOUT_MS: i32 = 1000;

pub enum OffloadedEsc {
    TalonFX(TalonFX),
}

impl OffloadedEsc {
    pub fn talon_fx(id: i32) -> Self {
        Self::TalonFX(TalonFX::new(id))
    }

    pub fn setup(&mut self, config: OffloadedEscConfig) -> Result<()> {
        match self {
            OffloadedEsc::TalonFX(talon) => {
                talon.config_factory_default(CONFIG_TIMEOUT_MS)?;
                talon.set_neutral_mode(NeutralMode::Brake);

                talon.config_openloop_ramp(config.open_loop_ramp, CONFIG_TIMEOUT_MS)?;
                talon.config_closedloop_ramp(config.closed_loop_ramp, CONFIG_TIMEOUT_MS)?;
                talon.config_peak_output_forward(
                    config.peak_output_forward / config.voltage_comp_saturation,
                    CONFIG_TIMEOUT_MS,
                )?;
                talon.config_peak_output_reverse(
                    config.peak_output_reverse / config.voltage_comp_saturation,
                    CONFIG_TIMEOUT_MS,
                )?;
                talon.config_nominal_output_forward(
                    config.nominal_output_forward / config.voltage_comp_saturation,
                    CONFIG_TIMEOUT_MS,
                )?;
                talon.config_nominal_output_reverse(
                    config.nominal_output_reverse / config.voltage_comp_saturation,
                    CONFIG_TIMEOUT_MS,
                )?;
                talon.config_voltage_comp_saturation(
                    config.voltage_comp_saturation,
                    CONFIG_TIMEOUT_MS,
                )?;

                talon.config_stator_current_limit(
                    StatorCurrentLimitConfiguration {
                        enable: true,
                        current_limit: config.continuous_current_limit,
                        trigger_threshold_current: config.peak_current_limit,
                        trigger_threshold_time: config.peak_current_duration,
                    },
                    CONFIG_TIMEOUT_MS,
                )?;
                talon.config_supply_current_limit(
                    SupplyCurrentLimitConfiguration {
                        enable: true,
                        current_limit: config.continuous_current_limit,
                        trigger_threshold_current: config.peak_current_limit,
                        trigger_threshold_time: config.peak_current_duration,
                    },
                    CONFIG_TIMEOUT_MS,
                )?;
            }
        }

        Ok(())
    }

    pub fn output_velocity(&mut self, value: f64) {
        match self {
            OffloadedEsc::TalonFX(talon) => talon.set(TalonFXControlMode::Velocity, value),
            _ => panic!("Not a TalonFx"),
        }
    }

    pub fn output_percent(&mut self, value: f64) {
        match self {
            OffloadedEsc::TalonFX(talon) => talon.set(TalonFXControlMode::PercentOutput, value),
        }
    }

    pub fn as_talon_fx(&mut self) -> &mut TalonFX {
        match self {
            OffloadedEsc::TalonFX(talon) => talon,
            _ => panic!("Not a TalonFX"),
        }
    }

    pub fn config_talon_fx(&mut self, f: impl FnOnce(&mut TalonFX)) {
        f(self.as_talon_fx());
    }
}

#[derive(Clone, Copy)]
pub struct OffloadedEscConfig {
    open_loop_ramp: Time,
    closed_loop_ramp: Time,
    peak_output_forward: ElectricPotential,
    peak_output_reverse: ElectricPotential,
    nominal_output_forward: ElectricPotential,
    nominal_output_reverse: ElectricPotential,
    voltage_comp_saturation: ElectricPotential,
    continuous_current_limit: ElectricCurrent,
    peak_current_limit: ElectricCurrent,
    peak_current_duration: Time,
}

impl Default for OffloadedEscConfig {
    fn default() -> Self {
        OffloadedEscConfig {
            open_loop_ramp: Time::new::<second>(0.0),
            closed_loop_ramp: Time::new::<second>(0.0),
            peak_output_forward: ElectricPotential::new::<volt>(12.0),
            peak_output_reverse: ElectricPotential::new::<volt>(-12.0),
            nominal_output_forward: ElectricPotential::new::<volt>(0.0),
            nominal_output_reverse: ElectricPotential::new::<volt>(0.0),
            voltage_comp_saturation: ElectricPotential::new::<volt>(12.0),
            continuous_current_limit: ElectricCurrent::new::<ampere>(25.0),
            peak_current_limit: ElectricCurrent::new::<ampere>(40.0),
            peak_current_duration: Time::new::<second>(0.0),
        }
    }
}
