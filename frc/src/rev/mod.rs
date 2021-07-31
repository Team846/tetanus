pub mod rev_CANSparkMax;

use frc_sys::rev::rev_CANAnalog_AnalogMode;
use frc_sys::rev::rev_CANDigitalInput_LimitSwitchPolarity;
use frc_sys::rev::rev_CANEncoder_AlternateEncoderType;
use frc_sys::rev::rev_CANEncoder_EncoderType;
use frc_sys::rev::rev_CANSparkMax_ExternalFollower;
use frc_sys::rev::rev_CANSparkMax_FaultID;
use frc_sys::rev::rev_CANSparkMax_IdleMode;
use frc_sys::rev::rev_CANSparkMax_SoftLimitDirection;

pub type EncoderType = rev_CANEncoder_EncoderType;
pub type AlternateEncoderType = rev_CANEncoder_AlternateEncoderType;
pub type AnalogMode = rev_CANAnalog_AnalogMode;
pub type LimitSwitchPolarity = rev_CANDigitalInput_LimitSwitchPolarity;
pub type IdleMode = rev_CANSparkMax_IdleMode;
pub type ExternalFollower = rev_CANSparkMax_ExternalFollower;
pub type FaultID = rev_CANSparkMax_FaultID;
pub type SoftLimitDirection = rev_CANSparkMax_SoftLimitDirection;
