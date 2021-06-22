use frc_sys::wpilib::{self, frc_CANStatus};
use uom::si::electric_current::ampere;
use uom::si::electric_potential::volt;
use uom::si::f64::{ElectricCurrent, ElectricPotential, Time};
use uom::si::time::microsecond;

use super::driver_station;

pub type CANStatus = frc_CANStatus;

/// Return the FPGA Version number.
///
/// For now, expect this to be competition year.
///
/// @return FPGA Version number.
pub fn get_fpga_version() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFPGAVersion() }
}

/// Return the FPGA Revision number.
///
/// The format of the revision is 3 numbers. The 12 most significant bits are
/// the Major Revision. The next 8 bits are the Minor Revision. The 12 least
/// significant bits are the Build Number.
///
/// @return FPGA Revision number.
pub fn get_fpga_revision() -> i64 {
    unsafe { wpilib::frc_RobotController_GetFPGARevision() }
}

/// Read the microsecond-resolution timer on the FPGA.
///
/// @return The current time in microseconds according to the FPGA (since FPGA
///         reset).
pub fn get_fpga_time() -> Time {
    unsafe { Time::new::<microsecond>(wpilib::frc_RobotController_GetFPGATime() as f64) }
}

/// Get the state of the "USER" button on the roboRIO.
///
/// @return True if the button is currently pressed down
pub fn get_user_button() -> bool {
    unsafe { wpilib::frc_RobotController_GetUserButton() }
}

/// Read the battery voltage.
///
/// @return The battery voltage in Volts.
pub fn get_battery_voltage() -> ElectricPotential {
    // TODO does this do the same thing as driver_station? (RobotController::GetBatteryVoltage returns with c++ units)
    driver_station::get_battery_voltage()
}

/// Check if the FPGA outputs are enabled.
///
/// The outputs may be disabled if the robot is disabled or e-stopped, the
/// watchdog has expired, or if the roboRIO browns out.
///
/// @return True if the FPGA outputs are enabled.
pub fn is_sys_active() -> bool {
    unsafe { wpilib::frc_RobotController_IsSysActive() }
}

/// Check if the system is browned out.
///
/// @return True if the system is browned out
pub fn is_browned_out() -> bool {
    unsafe { wpilib::frc_RobotController_IsBrownedOut() }
}

/// Get the input voltage to the robot controller.
///
/// @return The controller input voltage value in Volts
pub fn get_input_voltage() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetInputVoltage()) }
}

/// Get the input current to the robot controller.
///
/// @return The controller input current value in Amps
pub fn get_input_current() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetInputCurrent()) }
}

/// Get the voltage of the 3.3V rail.
///
/// @return The controller 3.3V rail voltage value in Volts
pub fn get_voltage_3v3() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetVoltage3V3()) }
}

/// Get the current output of the 3.3V rail.
///
/// @return The controller 3.3V rail output current value in Amps
pub fn get_current_3v3() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetCurrent3V3()) }
}

/// Get the enabled state of the 3.3V rail. The rail may be disabled due to a
/// controller brownout, a short circuit on the rail, or controller
/// over-voltage.
///
/// @return The controller 3.3V rail enabled value. True for enabled.
pub fn get_enabled_3v3() -> bool {
    unsafe { wpilib::frc_RobotController_GetEnabled3V3() }
}

/// Get the count of the total current faults on the 3.3V rail since the
/// controller has booted.
///
/// @return The number of faults
pub fn get_fault_count_3v3() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFaultCount3V3() }
}

/// Get the voltage of the 5V rail.
///
/// @return The controller 5V rail voltage value in Volts
pub fn get_voltage_5v() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetVoltage5V()) }
}

/// Get the current output of the 5V rail.
///
/// @return The controller 5V rail output current value in Amps
pub fn get_current_5v() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetCurrent5V()) }
}

/// Get the enabled state of the 5V rail. The rail may be disabled due to a
/// controller brownout, a short circuit on the rail, or controller
/// over-voltage.
///
/// @return The controller 5V rail enabled value. True for enabled.
pub fn get_enabled_5v() -> bool {
    unsafe { wpilib::frc_RobotController_GetEnabled5V() }
}

/// Get the count of the total current faults on the 5V rail since the
/// controller has booted.
///
/// @return The number of faults
pub fn get_fault_count_5v() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFaultCount5V() }
}

/// Get the voltage of the 6V rail.
///
/// @return The controller 6V rail voltage value in Volts
pub fn get_voltage_6v() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(wpilib::frc_RobotController_GetVoltage6V()) }
}

/// Get the current output of the 6V rail.
///
/// @return The controller 6V rail output current value in Amps
pub fn get_current_6v() -> ElectricCurrent {
    unsafe { ElectricCurrent::new::<ampere>(wpilib::frc_RobotController_GetCurrent6V()) }
}

/// Get the enabled state of the 6V rail. The rail may be disabled due to a
/// controller brownout, a short circuit on the rail, or controller
/// over-voltage.
///
/// @return The controller 6V rail enabled value. True for enabled.
pub fn get_enabled_6v() -> bool {
    unsafe { wpilib::frc_RobotController_GetEnabled6V() }
}

/// Get the count of the total current faults on the 6V rail since the
/// controller has booted.
///
/// @return The number of faults.
pub fn get_fault_count_6v() -> i32 {
    unsafe { wpilib::frc_RobotController_GetFaultCount6V() }
}

pub fn get_can_status() -> CANStatus {
    unsafe { wpilib::frc_RobotController_GetCANStatus() }
}
