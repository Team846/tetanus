use frc_sys::ctre::ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration;
use uom::si::electric_current::ampere;
use uom::si::f64::*;
use uom::si::time::second;

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
