use frc_sys::wpilib::{self, frc_CANStatus};
use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricCurrent, ElectricPotential, Time};
use uom::si::time::microsecond;

use super::driver_station;

pub type CANStatus = frc_CANStatus;

pub fn get_fpga_version() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFPGAVersion() }
}

pub fn get_fpga_revision() -> i64 {
    unsafe { wpilib::frc_RobotController_GetFPGARevision() }
}

pub fn get_fpga_time() -> Time {
    unsafe { Time::new::<microsecond>(wpilib::frc_RobotController_GetFPGATime() as f64) }
}

pub fn get_user_button() -> bool {
    unsafe { wpilib::frc_RobotController_GetUserButton() }
}

pub fn get_battery_voltage() -> ElectricPotential {
    // TODO does this do the same thing as driver_station? (RobotController::GetBatteryVoltage returns with c++ units)
    driver_station::get_battery_voltage()
}

pub fn is_sys_active() -> bool {
    unsafe { wpilib::frc_RobotController_IsSysActive() }
}

pub fn is_browned_out() -> bool {
    unsafe { wpilib::frc_RobotController_IsBrownedOut() }
}

pub fn get_input_voltage() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetInputVoltage()) }
}

pub fn get_input_current() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetInputCurrent()) }
}

pub fn get_voltage_3v3() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetVoltage3V3()) }
}

pub fn get_current_3v3() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetCurrent3V3()) }
}

pub fn get_enabled_3v3() -> bool {
    unsafe { wpilib::frc_RobotController_GetEnabled3V3() }
}

pub fn get_fault_count_3v3() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFaultCount3V3() }
}

pub fn get_voltage_5v() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetVoltage5V()) }
}

pub fn get_current_5v() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetCurrent5V()) }
}

pub fn get_enabled_5v() -> bool {
    unsafe { wpilib::frc_RobotController_GetEnabled5V() }
}

pub fn get_fault_count_5v() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFaultCount5V() }
}

pub fn get_voltage_6v() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetVoltage6V()) }
}

pub fn get_current_6v() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetCurrent6V()) }
}

pub fn get_enabled_6v() -> bool {
    unsafe { wpilib::frc_RobotController_GetEnabled6V() }
}

pub fn get_fault_count_6v() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFaultCount6V() }
}

pub fn get_can_status() -> CANStatus {
    unsafe { wpilib::frc_RobotController_GetCANStatus() }
}
