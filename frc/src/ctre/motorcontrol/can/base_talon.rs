use frc_sys::ctre;
use frc_sys::ctre::ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration;

use crate::ctre as frc_ctre;
use crate::ctre::motorcontrol::SupplyCurrentLimitConfiguration;

use super::base_motor_controller::BaseMotorController;

use anyhow::Result;
use uom::si::electric_current::ampere;
use uom::si::f64::*;

pub trait BaseTalon: BaseMotorController {
    fn get_output_current(&mut self) -> ElectricCurrent {
        unsafe {
            ElectricCurrent::new::<ampere>(
                ctre::ctre_phoenix_motorcontrol_can_BaseTalon_GetOutputCurrent(
                    self.get_handle_ptr(),
                ),
            )
        }
    }

    fn get_stator_current(&mut self) -> ElectricCurrent {
        unsafe {
            ElectricCurrent::new::<ampere>(
                ctre::ctre_phoenix_motorcontrol_can_BaseTalon_GetStatorCurrent(
                    self.get_handle_ptr() as *mut ctre::ctre_phoenix_motorcontrol_can_BaseTalon,
                ),
            )
        }
    }

    fn get_supply_current(&mut self) -> ElectricCurrent {
        unsafe {
            ElectricCurrent::new::<ampere>(
                ctre::ctre_phoenix_motorcontrol_can_BaseTalon_GetSupplyCurrent(
                    self.get_handle_ptr() as *mut ctre::ctre_phoenix_motorcontrol_can_BaseTalon,
                ),
            )
        }
    }

    fn config_supply_current_limit(
        &mut self,
        curr_limit_configs: SupplyCurrentLimitConfiguration,
        timeout_ms: i32, /* 50 */
    ) -> Result<()> {
        frc_ctre::to_result(unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseTalon_ConfigSupplyCurrentLimit(
                self.get_handle_ptr(),
                &curr_limit_configs as *const _
                    as *const ctre_phoenix_motorcontrol_SupplyCurrentLimitConfiguration,
                timeout_ms,
            )
        })
    }

    fn is_fwd_limit_switch_closed(&mut self) -> bool {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseTalon_IsFwdLimitSwitchClosed(
                self.get_handle_ptr() as *mut ctre::ctre_phoenix_motorcontrol_can_BaseTalon,
            ) == 1
        }
    }

    fn is_rev_limit_switch_closed(&mut self) -> bool {
        unsafe {
            ctre::ctre_phoenix_motorcontrol_can_BaseTalon_IsRevLimitSwitchClosed(
                self.get_handle_ptr() as *mut ctre::ctre_phoenix_motorcontrol_can_BaseTalon,
            ) == 1
        }
    }
}
