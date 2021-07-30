use std::ffi::c_void;

use frc_sys::ctre::{
    self, ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration,
    ctre_phoenix_motorcontrol_can_TalonFX_ConfigSupplyCurrentLimit,
};
use frc_sys::ctre::{
    ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration,
    ctre_phoenix_motorcontrol_can_TalonFX,
};

use super::base_motor_controller::BaseMotorController;
use super::base_talon::BaseTalon;
use crate::ctre as frc_ctre;
use crate::ctre::motorcontrol::{
    DemandType, StatorCurrentLimitConfiguration, SupplyCurrentLimitConfiguration,
    TalonFXControlMode, TalonFXFeedbackDevice, TalonFXInvertType,
};
use anyhow::Result;
pub struct TalonFX {
    handle: ctre_phoenix_motorcontrol_can_TalonFX,
}

impl TalonFX {
    pub fn new(device_number: i32) -> Self {
        TalonFX {
            handle: unsafe { ctre_phoenix_motorcontrol_can_TalonFX::new(device_number) },
        }
    }

    pub fn set(&mut self, mode: TalonFXControlMode, value: f64) {
        unsafe {
            self.handle.Set(mode, value);
        }
    }

    pub fn set1(
        &mut self,
        mode: TalonFXControlMode,
        demand0: f64,
        demand1_type: DemandType,
        demand1: f64,
    ) {
        unsafe {
            self.handle.Set1(mode, demand0, demand1_type, demand1);
        }
    }

    pub fn set_inverted(&mut self, invert_type: TalonFXInvertType) {
        unsafe {
            self.handle.SetInverted(invert_type);
        }
    }

    pub fn config_selected_feedback_sensor(
        &mut self,
        feedback_device: TalonFXFeedbackDevice,
        pid_idx: i32,    /* 0 */
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle
                .ConfigSelectedFeedbackSensor(feedback_device, pid_idx, timeout_ms)
        })
    }

    pub fn config_supply_current_limit(
        &mut self,
        curr_limit_configs: SupplyCurrentLimitConfiguration,
        timeout_ms: i32, /* 50 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            // handle method not generated
            ctre_phoenix_motorcontrol_can_TalonFX_ConfigSupplyCurrentLimit(
                &mut self.handle as *mut _ as *mut c_void,
                &curr_limit_configs.into()
                    as *const ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration,
                timeout_ms,
            )
        })
    }

    pub fn config_stator_current_limit(
        &mut self,
        curr_limit_configs: StatorCurrentLimitConfiguration,
        timeout_ms: i32, /* 50 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            self.handle.ConfigStatorCurrentLimit(
                &curr_limit_configs.into()
                    as *const ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration,
                timeout_ms,
            )
        })
    }

    pub fn config_get_supply_current_limit(
        &mut self,
        timeout_ms: i32, /* 50 */
    ) -> Result<SupplyCurrentLimitConfiguration> {
        let mut config = ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration {
            enable: false,
            currentLimit: 0.0,
            triggerThresholdCurrent: 0.0,
            triggerThresholdTime: 0.0,
        };

        frc_ctre::to_result(unsafe {
            self.handle.ConfigGetSupplyCurrentLimit(
                &mut config as *mut ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration,
                timeout_ms,
            )
        })?;

        Ok(config.into())
    }

    pub fn config_get_stator_current_limit(
        &mut self,
        timeout_ms: i32, /* 50 */
    ) -> Result<StatorCurrentLimitConfiguration> {
        let mut config = ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration {
            enable: false,
            currentLimit: 0.0,
            triggerThresholdCurrent: 0.0,
            triggerThresholdTime: 0.0,
        };

        frc_ctre::to_result(unsafe {
            self.handle.ConfigGetStatorCurrentLimit(
                &mut config as *mut ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration,
                timeout_ms,
            )
        })?;

        Ok(config.into())
    }
}

impl BaseMotorController for TalonFX {
    fn get_handle_ptr(&mut self) -> *mut c_void {
        &mut self.handle as *mut _ as *mut c_void
    }
}

impl BaseTalon for TalonFX {}

impl Drop for TalonFX {
    fn drop(&mut self) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_TalonFX_TalonFX_destructor(&mut self.handle);
        }
    }
}
