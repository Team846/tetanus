use frc_sys::ctre::ctre_phoenix_motorcontrol_StatorCurrentLimitConfiguration;
use uom::si::electric_current::ampere;
use uom::si::f64::*;
use uom::si::time::second;

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
