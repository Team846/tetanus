use frc_sys::ctre::*;

pub mod can;

mod stator_current_limit_configuration;
pub use stator_current_limit_configuration::StatorCurrentLimitConfiguration;

mod supply_current_limit_configuration;
pub use supply_current_limit_configuration::SupplyCurrentLimitConfiguration;

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
