use std::ffi::c_void;

use frc_sys::ctre;

use crate::ctre::{
    self as frc_ctre,
    motorcontrol::{
        ControlFrame, ControlMode, DemandType, FeedbackDevice, FollowerType, InvertType,
        LimitSwitchNormal, LimitSwitchSource, NeutralMode, RemoteFeedbackDevice,
        RemoteLimitSwitchSource, RemoteSensorSource, SensorTerm, StatusFrame, StatusFrameEnhanced,
        VelocityMeasPeriod,
    },
};

use anyhow::Result;

use uom::si::electric_potential::volt;
use uom::si::f64::*;
use uom::si::ratio::percent;
use uom::si::time::second;

pub trait BaseMotorController {
    fn get_handle_ptr(&mut self) -> *mut c_void;

    fn get_device_id(&mut self) -> i32 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetDeviceID(
                self.get_handle_ptr(),
            )
        }
    }

    fn set(&mut self, mode: ControlMode, value: f64) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Set(
                self.get_handle_ptr(),
                mode,
                value,
            );
        }
    }

    fn set1(&mut self, mode: ControlMode, demand0: f64, demand1_type: DemandType, demand1: f64) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Set1(
                self.get_handle_ptr(),
                mode,
                demand0,
                demand1_type,
                demand1,
            );
        }
    }

    fn neutral_output(&mut self) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_NeutralOutput(
                self.get_handle_ptr(),
            );
        }
    }

    fn set_neutral_mode(&mut self, neutral_mode: NeutralMode) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetNeutralMode(
                self.get_handle_ptr(),
                neutral_mode,
            );
        }
    }

    fn set_sensor_phase(&mut self, phase: bool) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetSensorPhase(
                self.get_handle_ptr(),
                phase,
            );
        }
    }

    fn set_inverted(&mut self, invert: bool) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetInverted(
                self.get_handle_ptr(),
                invert,
            );
        }
    }

    fn set_inverted1(&mut self, invert_type: InvertType) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetInverted1(
                self.get_handle_ptr(),
                invert_type,
            );
        }
    }

    fn get_inverted(&mut self) -> bool {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetInverted(
                self.get_handle_ptr(),
            )
        }
    }

    fn config_factory_default(&mut self, timeout_ms: i32 /* 50 */) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigFactoryDefault(
                self.get_handle_ptr(),
                timeout_ms,
            )
        })
    }

    fn config_openloop_ramp(
        &mut self,
        time_from_neutral_to_full: Time,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigOpenloopRamp(
                self.get_handle_ptr(),
                time_from_neutral_to_full.get::<second>(),
                timeout_ms,
            )
        })
    }

    fn config_closedloop_ramp(
        &mut self,
        time_from_neutral_to_full: Time,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigClosedloopRamp(
                self.get_handle_ptr(),
                time_from_neutral_to_full.get::<second>(),
                timeout_ms,
            )
        })
    }

    fn config_peak_output_forward(
        &mut self,
        out: Ratio,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigPeakOutputForward(
                self.get_handle_ptr(),
                out.get::<percent>(),
                timeout_ms,
            )
        })
    }

    fn config_peak_output_reverse(
        &mut self,
        out: Ratio,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigPeakOutputReverse(
                self.get_handle_ptr(),
                out.get::<percent>(),
                timeout_ms,
            )
        })
    }

    fn config_nominal_output_forward(
        &mut self,
        out: Ratio,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigNominalOutputForward(
                self.get_handle_ptr(),
                out.get::<percent>(),
                timeout_ms,
            )
        })
    }

    fn config_nominal_output_reverse(
        &mut self,
        out: Ratio,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigNominalOutputReverse(
                self.get_handle_ptr(),
                out.get::<percent>(),
                timeout_ms,
            )
        })
    }

    fn config_neutral_deadband(
        &mut self,
        deadband: Ratio,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigNeutralDeadband(
                self.get_handle_ptr(),
                deadband.get::<percent>(),
                timeout_ms,
            )
        })
    }

    fn config_voltage_comp_saturation(
        &mut self,
        voltage: ElectricPotential,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigVoltageCompSaturation(
                self.get_handle_ptr(),
                voltage.get::<volt>(),
                timeout_ms,
            )
        })
    }

    fn config_voltage_measurement_filter(
        &mut self,
        filter_window_samples: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigVoltageMeasurementFilter(
                self.get_handle_ptr(),
                filter_window_samples,
                timeout_ms,
            )
        })
    }

    fn enable_voltage_compensation(&mut self, enable: bool) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_EnableVoltageCompensation(
                self.get_handle_ptr(),
                enable,
            );
        }
    }

    fn is_voltage_compensation_enabled(&mut self) -> bool {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_IsVoltageCompensationEnabled(
                self.get_handle_ptr(),
            )
        }
    }

    fn get_bus_voltage(&mut self) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetBusVoltage(
                self.get_handle_ptr(),
            )
        }
    }

    fn get_motor_output_percent(&mut self) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetMotorOutputPercent(
                self.get_handle_ptr(),
            )
        }
    }

    fn get_motor_output_voltage(&mut self) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetMotorOutputVoltage(
                self.get_handle_ptr(),
            )
        }
    }

    fn get_temperature(&mut self) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetTemperature(
                self.get_handle_ptr(),
            )
        }
    }

    fn config_selected_feedback_sensor(
        &mut self,
        feedback_device: RemoteFeedbackDevice,
        pid_idx: i32,    /* 0 */
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigSelectedFeedbackSensor(
                self.get_handle_ptr(),
                feedback_device,
                pid_idx,
                timeout_ms,
            )
        })
    }

    fn config_selected_feedback_sensor1(
        &mut self,
        feedback_device: FeedbackDevice,
        pid_idx: i32,    /* 0 */
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigSelectedFeedbackSensor1(
                self.get_handle_ptr(),
                feedback_device,
                pid_idx,
                timeout_ms,
            )
        })
    }

    fn config_selected_feedback_coefficient(
        &mut self,
        coefficient: f64,
        pid_idx: i32,    /* 0 */
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigSelectedFeedbackCoefficient(
                self.get_handle_ptr(),
                coefficient,
                pid_idx,
                timeout_ms,
            )
        })
    }

    fn config_remote_feedback_filter(
        &mut self,
        device_id: i32,
        remote_sensor_source: RemoteSensorSource,
        remote_ordinal: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigRemoteFeedbackFilter(
                self.get_handle_ptr(),
                device_id,
                remote_sensor_source,
                remote_ordinal,
                timeout_ms,
            )
        })
    }

    // fn ConfigRemoteFeedbackFilter1(&mut self, canCoderRef: &CANCoder, remoteOrdinal: i32, timeout_ms: i32 /* 0 */) -> Result<()>  { }

    fn config_sensor_term(
        &mut self,
        sensor_term: SensorTerm,
        feedback_device: FeedbackDevice,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigSensorTerm(
                self.get_handle_ptr(),
                sensor_term,
                feedback_device,
                timeout_ms,
            )
        })
    }

    fn config_sensor_term1(
        &mut self,
        sensor_term: SensorTerm,
        feedback_device: RemoteFeedbackDevice,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigSensorTerm1(
                self.get_handle_ptr(),
                sensor_term,
                feedback_device,
                timeout_ms,
            )
        })
    }

    fn get_selected_sensor_position(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetSelectedSensorPosition(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn get_selected_sensor_velocity(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetSelectedSensorVelocity(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn set_selected_sensor_position(
        &mut self,
        sensor_pos: f64,
        pid_idx: i32,    /* 0 */
        timeout_ms: i32, /* 50 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetSelectedSensorPosition(
                self.get_handle_ptr(),
                sensor_pos,
                pid_idx,
                timeout_ms,
            )
        })
    }

    fn set_control_frame_period(&mut self, frame: ControlFrame, period_ms: i32) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetControlFramePeriod(
                self.get_handle_ptr(),
                frame,
                period_ms,
            )
        })
    }

    fn set_status_frame_period(
        &mut self,
        frame: StatusFrame,
        period_ms: u8,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetStatusFramePeriod(
                self.get_handle_ptr(),
                frame,
                period_ms,
                timeout_ms,
            )
        })
    }

    fn set_status_frame_period1(
        &mut self,
        frame: StatusFrameEnhanced,
        period_ms: u8,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetStatusFramePeriod1(
                self.get_handle_ptr(),
                frame,
                period_ms,
                timeout_ms,
            )
        })
    }

    fn get_status_frame_period(&mut self, frame: StatusFrame, timeout_ms: i32 /* 0 */) -> i32 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetStatusFramePeriod(
                self.get_handle_ptr(),
                frame,
                timeout_ms,
            )
        }
    }

    fn get_status_frame_period1(
        &mut self,
        frame: StatusFrameEnhanced,
        timeout_ms: i32, /* 0 */
    ) -> i32 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetStatusFramePeriod1(
                self.get_handle_ptr(),
                frame,
                timeout_ms,
            )
        }
    }

    fn config_velocity_measurement_period(
        &mut self,
        period: VelocityMeasPeriod,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigVelocityMeasurementPeriod(
                self.get_handle_ptr(),
                period,
                timeout_ms,
            )
        })
    }

    fn config_velocity_measurement_window(
        &mut self,
        window_size: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigVelocityMeasurementWindow(
                self.get_handle_ptr(),
                window_size,
                timeout_ms,
            )
        })
    }

    fn config_forward_limit_switch_source(
        &mut self,
        type_: RemoteLimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        device_id: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigForwardLimitSwitchSource(
                self.get_handle_ptr(),
                type_,
                normal_open_or_close,
                device_id,
                timeout_ms,
            )
        })
    }

    fn config_reverse_limit_switch_source(
        &mut self,
        type_: RemoteLimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        device_id: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigReverseLimitSwitchSource(
                self.get_handle_ptr(),
                type_,
                normal_open_or_close,
                device_id,
                timeout_ms,
            )
        })
    }

    fn override_limit_switches_enable(&mut self, enable: bool) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_OverrideLimitSwitchesEnable(
                self.get_handle_ptr(),
                enable,
            );
        }
    }

    fn config_forward_limit_switch_source1(
        &mut self,
        type_: LimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigForwardLimitSwitchSource1(
                self.get_handle_ptr(),
                type_,
                normal_open_or_close,
                timeout_ms,
            )
        })
    }

    fn config_reverse_limit_switch_source1(
        &mut self,
        type_: LimitSwitchSource,
        normal_open_or_close: LimitSwitchNormal,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigReverseLimitSwitchSource1(
                self.get_handle_ptr(),
                type_,
                normal_open_or_close,
                timeout_ms,
            )
        })
    }

    fn config_forward_soft_limit_threshold(
        &mut self,
        forward_sensor_limit: f64,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigForwardSoftLimitThreshold(
                self.get_handle_ptr(),
                forward_sensor_limit,
                timeout_ms,
            )
        })
    }

    fn config_reverse_soft_limit_threshold(
        &mut self,
        reverse_sensor_limit: f64,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigReverseSoftLimitThreshold(
                self.get_handle_ptr(),
                reverse_sensor_limit,
                timeout_ms,
            )
        })
    }

    fn config_forward_soft_limit_enable(
        &mut self,
        enable: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigForwardSoftLimitEnable(
                self.get_handle_ptr(),
                enable,
                timeout_ms,
            )
        })
    }

    fn config_reverse_soft_limit_enable(
        &mut self,
        enable: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigReverseSoftLimitEnable(
                self.get_handle_ptr(),
                enable,
                timeout_ms,
            )
        })
    }

    fn override_soft_limits_enable(&mut self, enable: bool) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_OverrideSoftLimitsEnable(
                self.get_handle_ptr(),
                enable,
            );
        }
    }

    fn config_kp(&mut self, slot_idx: i32, value: f64, timeout_ms: i32 /* 0 */) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Config_kP(
                self.get_handle_ptr(),
                slot_idx,
                value,
                timeout_ms,
            )
        })
    }

    fn config_ki(&mut self, slot_idx: i32, value: f64, timeout_ms: i32 /* 0 */) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Config_kI(
                self.get_handle_ptr(),
                slot_idx,
                value,
                timeout_ms,
            )
        })
    }

    fn config_kd(&mut self, slot_idx: i32, value: f64, timeout_ms: i32 /* 0 */) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Config_kD(
                self.get_handle_ptr(),
                slot_idx,
                value,
                timeout_ms,
            )
        })
    }

    fn config_kf(&mut self, slot_idx: i32, value: f64, timeout_ms: i32 /* 0 */) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Config_kF(
                self.get_handle_ptr(),
                slot_idx,
                value,
                timeout_ms,
            )
        })
    }

    fn config_integral_zone(
        &mut self,
        slot_idx: i32,
        izone: f64,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Config_IntegralZone(
                self.get_handle_ptr(),
                slot_idx,
                izone,
                timeout_ms,
            )
        })
    }

    fn config_allowable_closedloop_error(
        &mut self,
        slot_idx: i32,
        allowable_close_loop_error: f64,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigAllowableClosedloopError(
                self.get_handle_ptr(),
                slot_idx,
                allowable_close_loop_error,
                timeout_ms,
            )
        })
    }

    fn config_max_integral_accumulator(
        &mut self,
        slot_idx: i32,
        iaccum: f64,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigMaxIntegralAccumulator(
                self.get_handle_ptr(),
                slot_idx,
                iaccum,
                timeout_ms,
            )
        })
    }

    fn config_closed_loop_peak_output(
        &mut self,
        slot_idx: i32,
        out: Ratio,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigClosedLoopPeakOutput(
                self.get_handle_ptr(),
                slot_idx,
                out.get::<percent>(),
                timeout_ms,
            )
        })
    }

    fn config_closed_loop_period(
        &mut self,
        slot_idx: i32,
        loop_time_ms: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigClosedLoopPeriod(
                self.get_handle_ptr(),
                slot_idx,
                loop_time_ms,
                timeout_ms,
            )
        })
    }

    fn config_aux_pidpolarity(&mut self, invert: bool, timeout_ms: i32 /* 0 */) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigAuxPIDPolarity(
                self.get_handle_ptr(),
                invert,
                timeout_ms,
            )
        })
    }

    fn set_integral_accumulator(
        &mut self,
        iaccum: f64,
        pid_idx: i32,    /* 0 */
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SetIntegralAccumulator(
                self.get_handle_ptr(),
                iaccum,
                pid_idx,
                timeout_ms,
            )
        })
    }

    fn get_closed_loop_error(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetClosedLoopError(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn get_integral_accumulator(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetIntegralAccumulator(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn get_error_derivative(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetErrorDerivative(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn select_profile_slot(&mut self, slot_idx: i32, pid_idx: i32) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_SelectProfileSlot(
                self.get_handle_ptr(),
                slot_idx,
                pid_idx,
            )
        })
    }

    fn get_closed_loop_target(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetClosedLoopTarget(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn get_active_trajectory_position(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetActiveTrajectoryPosition(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn get_active_trajectory_velocity(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetActiveTrajectoryVelocity(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    fn get_active_trajectory_arb_feed_fwd(&mut self, pid_idx: i32 /* 0 */) -> f64 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetActiveTrajectoryArbFeedFwd(
                self.get_handle_ptr(),
                pid_idx,
            )
        }
    }

    // fn ConfigMotionCruiseVelocity(&mut self, sensorUnitsPer100ms: f64,timeout_ms: i32 /* 0 */) -> Result<()> {}
    // fn ConfigMotionAcceleration(&mut self, sensorUnitsPer100msPerSec: f64,timeout_ms: i32 /* 0 */) -> Result<()> {}
    // fn ConfigMotionSCurveStrength(&mut self, curveStrength: i32, timeout_ms: i32 /* 0 */) -> Result<()>  {}
    // fn ClearMotionProfileTrajectories(&mut self, ) -> Result<()>  {        }
    // fn GetMotionProfileTopLevelBufferCount(&mut self, ) -> i32  {}
    // fn PushMotionProfileTrajectory(&mut self, const ctre::phoenix::motion::TrajectoryPoint& trajPt) -> Result<()>  {}
    // fn StartMotionProfile(&mut self, ctre::phoenix::motion::BufferedTrajectoryPointStream& stream, minBufferedPts: u32, motionProfControlMode: ControlMode ) -> Result<()>  {}
    // fn IsMotionProfileFinished(&mut self) -> bool {}
    // fn IsMotionProfileTopLevelBufferFull(&mut self, ) -> bool {}
    // fn ProcessMotionProfileBuffer(&mut self, )  {}
    // fn GetMotionProfileStatus(&mut self, ctre::phoenix::motion::MotionProfileStatus& statusToFill) -> Result<()>  {}
    // fn ClearMotionProfileHasUnderrun(&mut self, timeout_ms: i32 /* 0 */) -> Result<()>  {    }
    // fn ChangeMotionControlFramePeriod(&mut self, period_ms: i32) -> Result<()>  {        }
    // fn ConfigMotionProfileTrajectoryPeriod(&mut self, baseTrajDurationMs: i32, timeout_ms: i32 /* 0 */) -> Result<()>  {        }
    // fn ConfigMotionProfileTrajectoryInterpolationEnable(&mut self, enable: bool, timeout_ms: i32 /* 0 */) -> Result<()>  {}

    fn config_feedback_not_continuous(
        &mut self,
        feedback_not_continuous: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigFeedbackNotContinuous(
                self.get_handle_ptr(),
                feedback_not_continuous,
                timeout_ms,
            )
        })
    }

    fn config_remote_sensor_closed_loop_disable_neutral_on_los(
        &mut self,
        remote_sensor_closed_loop_disable_neutral_on_los: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigRemoteSensorClosedLoopDisableNeutralOnLOS(self.get_handle_ptr(), remote_sensor_closed_loop_disable_neutral_on_los, timeout_ms)
        })
    }

    fn config_clear_position_on_limit_f(
        &mut self,
        clear_position_on_limit_f: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigClearPositionOnLimitF(
                self.get_handle_ptr(),
                clear_position_on_limit_f,
                timeout_ms,
            )
        })
    }

    fn config_clear_position_on_limit_r(
        &mut self,
        clear_position_on_limit_r: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigClearPositionOnLimitR(
                self.get_handle_ptr(),
                clear_position_on_limit_r,
                timeout_ms,
            )
        })
    }

    fn config_clear_position_on_quad_idx(
        &mut self,
        clear_position_on_quad_idx: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigClearPositionOnQuadIdx(
                self.get_handle_ptr(),
                clear_position_on_quad_idx,
                timeout_ms,
            )
        })
    }

    fn config_limit_switch_disable_neutral_on_los(
        &mut self,
        limit_switch_disable_neutral_on_los: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigLimitSwitchDisableNeutralOnLOS(
                self.get_handle_ptr(),
                limit_switch_disable_neutral_on_los,
                timeout_ms,
            )
        })
    }

    fn config_soft_limit_disable_neutral_on_los(
        &mut self,
        soft_limit_disable_neutral_on_los: bool,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigSoftLimitDisableNeutralOnLOS(
                self.get_handle_ptr(),
                soft_limit_disable_neutral_on_los,
                timeout_ms,
            )
        })
    }

    fn config_pulse_width_period_edges_per_rot(
        &mut self,
        pulse_width_period_edges_per_rot: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigPulseWidthPeriod_EdgesPerRot(
                self.get_handle_ptr(),
                pulse_width_period_edges_per_rot,
                timeout_ms,
            )
        })
    }

    fn config_pulse_width_period_filter_window_sz(
        &mut self,
        pulse_width_period_filter_window_sz: i32,
        timeout_ms: i32, /* 0 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ConfigPulseWidthPeriod_FilterWindowSz(
                self.get_handle_ptr(),
                pulse_width_period_filter_window_sz,
                timeout_ms,
            )
        })
    }

    fn get_last_error(&mut self) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetLastError(
                self.get_handle_ptr(),
            )
        })
    }

    // fn GetFaults(&mut self, Faults& toFill) -> Result<()>  { }
    // fn GetStickyFaults(&mut self, StickyFaults& toFill) -> Result<()>  { }

    fn clear_sticky_faults(&mut self, timeout_ms: i32 /* 0 */) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ClearStickyFaults(
                self.get_handle_ptr(),
                timeout_ms,
            )
        })
    }

    fn get_firmware_version(&mut self) -> i32 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetFirmwareVersion(
                self.get_handle_ptr(),
            )
        }
    }

    fn has_reset_occurred(&mut self) -> bool {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_HasResetOccurred(
                self.get_handle_ptr(),
            )
        }
    }

    // fn ConfigSetCustomParam(&mut self, newValue: i32, paramIndex: i32,timeout_ms: i32 /* 0 */) -> Result<()> {}
    // fn ConfigGetCustomParam(paramIndex: i32,timeout_ms: i32 /* 0 */);
    // fn ConfigSetParameter(&mut self, ctre::phoenix::ParamEnum param, value: f64,uint8_t subValue, ordinal: i32, timeout_ms: i32 /* 0 */) -> Result<()> {}
    // fn ConfigGetParameter(&mut self, ctre::phoenix::ParamEnum param, ordinal: i32, timeout_ms: i32 /* 0 */) -> f64  {}
    // fn ConfigGetParameter(ParamEnum param, int32_t valueToSend,int32_t& valueReceived, uint8_t& subValue, int32_t ordinal,timeoutMs: i32) -> Result<()> {}

    fn get_base_id(&mut self) -> i32 {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetBaseID(self.get_handle_ptr())
        }
    }

    fn get_control_mode(&mut self) -> ControlMode {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_GetControlMode(
                self.get_handle_ptr(),
            )
        }
    }

    fn follow(
        &mut self,
        master_to_follow: &mut impl BaseMotorController,
        follower_type: FollowerType,
    ) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Follow(
                self.get_handle_ptr()
                    as *mut ctre::ctre_phoenix_motorcontrol_can_BaseMotorController,
                master_to_follow.get_handle_ptr()
                    as *mut ctre::ctre_phoenix_motorcontrol_IMotorController,
                follower_type,
            );
        }
    }

    fn follow1(&mut self, master_to_follow: &mut impl BaseMotorController) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_Follow1(
                self.get_handle_ptr(),
                master_to_follow.get_handle_ptr()
                    as *mut ctre::ctre_phoenix_motorcontrol_IMotorController,
            );
        }
    }

    fn value_updated(&mut self) {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseMotorController_ValueUpdated(
                self.get_handle_ptr(),
            )
        }
    }

    // fn GetSlotConfigs(&mut self, SlotConfiguration& slot, int slot_idx = 0, timeout_ms: i32 /* 50 */);
    // fn GetFilterConfigs(&mut self, FilterConfiguration& Filter, int ordinal = 0, timeout_ms: i32 /* 50 */);
}
