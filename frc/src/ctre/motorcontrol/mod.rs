use frc_sys::ctre::*;
use uom::si::electric_current::ampere;
use uom::si::f64::*;
use uom::si::time::second;

pub mod can;

pub type ControlFrame = ctre_phoenix_motorcontrol_ControlFrame;
pub type ControlMode = ctre_phoenix_motorcontrol_ControlMode;
pub type DemandType = ctre_phoenix_motorcontrol_DemandType;
pub type FeedbackDevice = ctre_phoenix_motorcontrol_FeedbackDevice;
pub type FollowerType = ctre_phoenix_motorcontrol_FollowerType;
pub type InvertType = ctre_phoenix_motorcontrol_InvertType;
pub type LimitSwitchNormal = ctre_phoenix_motorcontrol_LimitSwitchNormal;
pub type LimitSwitchSource = ctre_phoenix_motorcontrol_LimitSwitchSource;
pub type NeutralMode = ctre_phoenix_motorcontrol_NeutralMode;
pub type RemoteFeedbackDevice = ctre_phoenix_motorcontrol_RemoteFeedbackDevice;
pub type RemoteLimitSwitchSource = ctre_phoenix_motorcontrol_RemoteLimitSwitchSource;
pub type RemoteSensorSource = ctre_phoenix_motorcontrol_RemoteSensorSource;
pub type SensorTerm = ctre_phoenix_motorcontrol_SensorTerm;
pub type StatusFrame = ctre_phoenix_motorcontrol_StatusFrame;
pub type StatusFrameEnhanced = ctre_phoenix_motorcontrol_StatusFrameEnhanced;
pub type TalonFXControlMode = ctre_phoenix_motorcontrol_TalonFXControlMode;
pub type TalonFXFeedbackDevice = ctre_phoenix_motorcontrol_TalonFXFeedbackDevice;
pub type TalonFXInvertType = ctre_phoenix_motorcontrol_TalonFXInvertType;
pub type VelocityMeasPeriod = ctre_phoenix_motorcontrol_VelocityMeasPeriod;

pub struct StatorCurrentLimitConfiguration {
    pub enable: bool,
    pub current_limit: ElectricCurrent,
    pub trigger_threshold_current: ElectricCurrent,
    pub trigger_threshold_time: Time,
}

impl From<StatorCurrentLimitConfiguration>
    for ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration
{
    fn from(config: StatorCurrentLimitConfiguration) -> Self {
        ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration {
            enable: config.enable,
            currentLimit: config.current_limit.get::<ampere>(),
            triggerThresholdCurrent: config.trigger_threshold_current.get::<ampere>(),
            triggerThresholdTime: config.trigger_threshold_time.get::<second>(),
        }
    }
}

impl From<ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration>
    for StatorCurrentLimitConfiguration
{
    fn from(config: ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration) -> Self {
        StatorCurrentLimitConfiguration {
            enable: config.enable,
            current_limit: ElectricCurrent::new::<ampere>(config.currentLimit),
            trigger_threshold_current: ElectricCurrent::new::<ampere>(
                config.triggerThresholdCurrent,
            ),
            trigger_threshold_time: Time::new::<second>(config.triggerThresholdTime),
        }
    }
}

pub struct SupplyCurrentLimitConfiguration {
    pub enable: bool,
    pub current_limit: ElectricCurrent,
    pub trigger_threshold_current: ElectricCurrent,
    pub trigger_threshold_time: Time,
}

impl From<SupplyCurrentLimitConfiguration>
    for ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration
{
    fn from(config: SupplyCurrentLimitConfiguration) -> Self {
        ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration {
            enable: config.enable,
            currentLimit: config.current_limit.get::<ampere>(),
            triggerThresholdCurrent: config.trigger_threshold_current.get::<ampere>(),
            triggerThresholdTime: config.trigger_threshold_time.get::<second>(),
        }
    }
}

impl From<ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration>
    for SupplyCurrentLimitConfiguration
{
    fn from(config: ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration) -> Self {
        SupplyCurrentLimitConfiguration {
            enable: config.enable,
            current_limit: ElectricCurrent::new::<ampere>(config.currentLimit),
            trigger_threshold_current: ElectricCurrent::new::<ampere>(
                config.triggerThresholdCurrent,
            ),
            trigger_threshold_time: Time::new::<second>(config.triggerThresholdTime),
        }
    }
}
