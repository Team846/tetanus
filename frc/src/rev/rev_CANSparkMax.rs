use frc_sys::rev::{
    rev_CANEncoder_EncoderType as EncoderType, rev_CANSparkMax,
    rev_CANSparkMaxLowLevel_MotorType as MotorType, rev_CANSparkMax_Disable, rev_CANSparkMax_Get,
    rev_CANSparkMax_GetInverted, rev_CANSparkMax_PIDWrite, rev_CANSparkMax_Set,
    rev_CANSparkMax_SetInverted, rev_CANSparkMax_StopMotor,
};
use std::ffi::c_void;

use super::{
    AlternateEncoderType, AnalogMode, CANAnalog, CANDigitalInput, CANEncoder, CANError,
    CANPIDController, ExternalFollower, FaultID, IdleMode, LimitSwitchPolarity, SoftLimitDirection,
};
pub struct CANSparkMax {
    handle: rev_CANSparkMax,
}

impl CANSparkMax {
    pub fn new(device_number: i32, motor: MotorType) -> CANSparkMax {
        CANSparkMax {
            handle: unsafe { rev_CANSparkMax::new(device_number, motor) },
        }
    }

    pub fn set(&mut self, speed: f64) {
        unsafe {
            rev_CANSparkMax_Set(&mut self.handle as *mut _ as *mut c_void, speed);
        }
    }

    // pub fn set_voltage(&mut self, volt_t: output) { help me andy

    // }

    pub fn get(&mut self) -> f64 {
        unsafe { rev_CANSparkMax_Get(&mut self.handle as *mut _ as *mut c_void) }
    }

    pub fn set_inverted(&mut self, is_inverted: bool) {
        unsafe {
            rev_CANSparkMax_SetInverted(&mut self.handle as *mut _ as *mut c_void, is_inverted)
        }
    }

    pub fn get_inverted(&mut self) -> bool {
        unsafe { rev_CANSparkMax_GetInverted(&mut self.handle as *mut _ as *mut c_void) }
    }

    pub fn disable(&mut self) {
        unsafe { rev_CANSparkMax_Disable(&mut self.handle as *mut _ as *mut c_void) }
    }

    pub fn stop_motor(&mut self) {
        unsafe { rev_CANSparkMax_StopMotor(&mut self.handle as *mut _ as *mut c_void) }
    }

    pub fn pid_write(&mut self, output: f64) {
        {
            unsafe { rev_CANSparkMax_PIDWrite(&mut self.handle as *mut _ as *mut c_void, output) }
        }
    }

    pub fn get_encoder(&mut self, sensor_type: EncoderType, counts_per_rev: i32) -> CANEncoder {
        unsafe { self.handle.GetEncoder(sensor_type, counts_per_rev) }
    }

    pub fn get_alternate_encoder(
        &mut self,
        sensor_type: AlternateEncoderType,
        counts_per_rev: i32,
    ) -> CANEncoder {
        unsafe { self.handle.GetAlternateEncoder(sensor_type, counts_per_rev) }
    }

    pub fn get_analog(
        &mut self,
        mode: AnalogMode, /* CANAnalog::AnalogMode::kAbsolute */
    ) -> CANAnalog {
        unsafe { self.handle.GetAnalog(mode) }
    }

    pub fn get_PID_controller(&mut self) -> CANPIDController {
        unsafe { self.handle.GetPIDController() }
    }

    pub fn get_forward_limit_switch(&mut self, polarity: LimitSwitchPolarity) -> CANDigitalInput {
        unsafe { self.handle.GetForwardLimitSwitch(polarity) }
    }

    pub fn get_reverse_limit_switch(&mut self, polarity: LimitSwitchPolarity) -> CANDigitalInput {
        unsafe { self.handle.GetReverseLimitSwitch(polarity) }
    }

    pub fn set_smart_current_limit(&mut self, limit: u32) -> CANError {
        unsafe { self.handle.SetSmartCurrentLimit(limit) }
    }

    pub fn set_smart_current_limit1(
        &mut self,
        stall_limit: u32,
        free_limit: u32,
        limit_RPM: u32, /* 20000 */
    ) -> CANError {
        unsafe {
            self.handle
                .SetSmartCurrentLimit1(stall_limit, free_limit, limit_RPM)
        }
    }

    pub fn set_secondary_current_limit(
        &mut self,
        limit: f64,
        limit_cycles: i32, /* 0 */
    ) -> CANError {
        unsafe { self.handle.SetSecondaryCurrentLimit(limit, limit_cycles) }
    }

    pub fn set_idle_mode(&mut self, mode: IdleMode) -> CANError {
        unsafe { self.handle.SetIdleMode(mode) }
    }

    pub fn get_idle_mode(&mut self) -> IdleMode {
        unsafe { self.handle.GetIdleMode() }
    }

    pub fn enable_voltage_compensation(&mut self, nominal_voltage: f64) -> CANError {
        unsafe { self.handle.EnableVoltageCompensation(nominal_voltage) }
    }

    pub fn disable_voltage_compensation(&mut self) -> CANError {
        unsafe { self.handle.DisableVoltageCompensation() }
    }

    pub fn get_voltage_compensation_nominal_voltage(&mut self) -> f64 {
        unsafe { self.handle.GetVoltageCompensationNominalVoltage() }
    }

    pub fn set_open_loop_ramp_rate(&mut self, rate: f64) -> CANError {
        unsafe { self.handle.SetOpenLoopRampRate(rate) }
    }

    pub fn set_closed_loop_ramp_rate(&mut self, rate: f64) -> CANError {
        unsafe { self.handle.SetClosedLoopRampRate(rate) }
    }

    pub fn get_open_loop_ramp_rate(&mut self) -> f64 {
        unsafe { self.handle.GetOpenLoopRampRate() }
    }

    pub fn get_closed_loop_ramp_rate(&mut self) -> f64 {
        unsafe { self.handle.GetClosedLoopRampRate() }
    }

    pub fn follow(
        &mut self,
        leader: *const rev_CANSparkMax,
        invert: bool, /* false */
    ) -> CANError {
        unsafe { self.handle.Follow(leader, invert) }
    }

    pub fn follow1(
        &mut self,
        leader: ExternalFollower,
        device_id: i32,
        invert: bool, /* flase */
    ) -> CANError {
        unsafe { self.handle.Follow1(leader, device_id, invert) }
    }

    pub fn is_follower(&mut self) -> bool {
        unsafe { self.handle.IsFollower() }
    }

    pub fn get_faults(&mut self) -> u16 {
        unsafe { self.handle.GetFaults() }
    }

    pub fn get_sticky_faults(&mut self) -> u16 {
        unsafe { self.handle.GetStickyFaults() }
    }

    pub fn get_fault(&mut self, fault_id: FaultID) -> bool {
        unsafe { self.handle.GetFault(fault_id) }
    }

    pub fn get_sticky_fault(&mut self, fault_id: FaultID) -> bool {
        unsafe { self.handle.GetStickyFault(fault_id) }
    }

    pub fn get_bus_voltage(&mut self) -> f64 {
        unsafe { self.handle.GetBusVoltage() }
    }

    pub fn get_applied_output(&mut self) -> f64 {
        unsafe { self.handle.GetAppliedOutput() }
    }

    pub fn get_output_current(&mut self) -> f64 {
        unsafe { self.handle.GetOutputCurrent() }
    }

    pub fn get_motor_temperature(&mut self) -> f64 {
        unsafe { self.handle.GetMotorTemperature() }
    }

    pub fn clear_faults(&mut self) -> CANError {
        unsafe { self.handle.ClearFaults() }
    }

    pub fn burn_flash(&mut self) -> CANError {
        unsafe { self.handle.BurnFlash() }
    }

    pub fn set_CAN_timeout(&mut self, millis: i32) -> CANError {
        unsafe { self.handle.SetCANTimeout(millis) }
    }

    pub fn enable_soft_limit(&mut self, direction: SoftLimitDirection, enable: bool) -> CANError {
        unsafe { self.handle.EnableSoftLimit(direction, enable) }
    }

    pub fn is_soft_limit_enabled(&mut self, direction: SoftLimitDirection) -> bool {
        unsafe { self.handle.IsSoftLimitEnabled(direction) }
    }

    pub fn set_soft_limit(&mut self, direction: SoftLimitDirection, limit: f64) -> CANError {
        unsafe { self.handle.SetSoftLimit(direction, limit) }
    }

    pub fn get_soft_limit(&mut self, direction: SoftLimitDirection) -> f64 {
        unsafe { self.handle.GetSoftLimit(direction) }
    }

    pub fn get_last_error(&mut self) -> CANError {
        unsafe { self.handle.GetLastError() }
    }
}
