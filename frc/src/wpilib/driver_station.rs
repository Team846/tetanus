//! Provide access to the network communication data to / from the Driver Station.

use std::ffi::CStr;
use std::ptr;

use frc_sys::hal::{
    HAL_GetJoystickDescriptor, HAL_GetMatchInfo, HAL_JoystickDescriptor, HAL_MatchInfo,
};
use frc_sys::wpilib::{frc_DriverStation, frc_DriverStation_Alliance, frc_DriverStation_MatchType};
use uom::si::electric_potential::volt;
use uom::si::f64::*;
use uom::si::time::second;

pub type Alliance = frc_DriverStation_Alliance;

pub type MatchType = frc_DriverStation_MatchType;

unsafe fn get_instance<'a>() -> &'a mut frc_DriverStation {
    frc_DriverStation::GetInstance().as_mut().unwrap()
}

fn to_string(ptr: *const u8) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

/// The state of one joystick button. Button indexes begin at 1.
///
/// @param stick  The joystick to read.
/// @param button The button index, beginning at 1.
/// @return The state of the joystick button.
pub fn get_stick_button(stick: i32, button: i32) -> bool {
    unsafe { get_instance().GetStickButton(stick, button) }
}

/// Whether one joystick button was pressed since the last check. Button
/// indexes begin at 1.
///
/// @param stick  The joystick to read.
/// @param button The button index, beginning at 1.
/// @return Whether the joystick button was pressed since the last check.
pub fn get_stick_button_pressed(stick: i32, button: i32) -> bool {
    unsafe { get_instance().GetStickButtonPressed(stick, button) }
}

/// Whether one joystick button was released since the last check. Button
/// indexes begin at 1.
///
/// @param stick  The joystick to read.
/// @param button The button index, beginning at 1.
/// @return Whether the joystick button was released since the last check.
pub fn get_stick_button_released(stick: i32, button: i32) -> bool {
    unsafe { get_instance().GetStickButtonReleased(stick, button) }
}

/// Get the value of the axis on a joystick.
///
/// This depends on the mapping of the joystick connected to the specified
/// port.
///
/// @param stick The joystick to read.
/// @param axis  The analog axis value to read from the joystick.
/// @return The value of the axis on the joystick.
pub fn get_stick_axis(stick: i32, axis: i32) -> f64 {
    unsafe { get_instance().GetStickAxis(stick, axis) }
}

/// Get the state of a POV on the joystick.
///
/// @return the angle of the POV in degrees, or -1 if the POV is not pressed.
pub fn get_stick_pov(stick: i32, pov: i32) -> i32 {
    unsafe { get_instance().GetStickPOV(stick, pov) }
}

/// The state of the buttons on the joystick.
///
/// @param stick The joystick to read.
/// @return The state of the buttons on the joystick.
pub fn get_stick_buttons(stick: i32) -> i32 {
    unsafe { get_instance().GetStickButtons(stick) }
}

/// Returns the number of axes on a given joystick port.
///
/// @param stick The joystick port number
/// @return The number of axes on the indicated joystick
pub fn get_stick_axis_count(stick: i32) -> i32 {
    unsafe { get_instance().GetStickAxisCount(stick) }
}

/// Returns the number of POVs on a given joystick port.
///
/// @param stick The joystick port number
/// @return The number of POVs on the indicated joystick
pub fn get_stick_pov_count(stick: i32) -> i32 {
    unsafe { get_instance().GetStickPOVCount(stick) }
}

/// Returns the number of buttons on a given joystick port.
///
/// @param stick The joystick port number
/// @return The number of buttons on the indicated joystick
pub fn get_stick_button_count(stick: i32) -> i32 {
    unsafe { get_instance().GetStickButtonCount(stick) }
}

/// Returns a boolean indicating if the controller is an xbox controller.
///
/// @param stick The joystick port number
/// @return A boolean that is true if the controller is an xbox controller.
pub fn get_joystick_is_xbox(stick: i32) -> bool {
    unsafe { get_instance().GetJoystickIsXbox(stick) }
}

/// Returns the type of joystick at a given port.
///
/// @param stick The joystick port number
/// @return The HID type of joystick at the given port
pub fn get_joystick_type(stick: i32) -> i32 {
    unsafe { get_instance().GetJoystickType(stick) }
}

/// Returns the name of the joystick at the given port.
///
/// @param stick The joystick port number
/// @return The name of the joystick at the given port
pub fn get_joystick_name(stick: i32) -> String {
    get_joystick_type(stick); // Check if stick is in range
    unsafe {
        // TODO test
        let desc: *mut HAL_JoystickDescriptor = ptr::null_mut();
        HAL_GetJoystickDescriptor(stick, desc);
        to_string(desc.as_ref().unwrap().name.as_ptr())
    }
}

/// Returns the types of Axes on a given joystick port.
///
/// @param stick The joystick port number and the target axis
/// @return What type of axis the axis is reporting to be
pub fn get_joystick_axis_type(stick: i32, axis: i32) -> i32 {
    unsafe { get_instance().GetJoystickAxisType(stick, axis) }
}

/// Returns if a joystick is connected to the Driver Station.
///
/// This makes a best effort guess by looking at the reported number of axis,
/// buttons, and POVs attached.
///
/// @param stick The joystick port number
/// @return true if a joystick is connected
pub fn is_joystick_connected(stick: i32) -> bool {
    unsafe { get_instance().IsJoystickConnected(stick) }
}

/// Check if the DS has enabled the robot.
///
/// @return True if the robot is enabled and the DS is connected
pub fn is_enabled() -> bool {
    unsafe { get_instance().IsEnabled() }
}

/// Check if the robot is disabled.
///
/// @return True if the robot is explicitly disabled or the DS is not connected
pub fn is_disabled() -> bool {
    unsafe { get_instance().IsDisabled() }
}

/// Check if the robot is e-stopped.
///
/// @return True if the robot is e-stopped
pub fn is_estopped() -> bool {
    unsafe { get_instance().IsEStopped() }
}

/// Check if the DS is commanding autonomous mode.
///
/// @return True if the robot is being commanded to be in autonomous mode
pub fn is_autonomous() -> bool {
    unsafe { get_instance().IsAutonomous() }
}

/// Check if the DS is commanding autonomous mode and if it has enabled the
/// robot.
///
/// @return True if the robot is being commanded to be in autonomous mode and
/// enabled.
pub fn is_autonomous_enabled() -> bool {
    unsafe { get_instance().IsAutonomousEnabled() }
}

/// Check if the DS is commanding teleop mode.
///
/// @return True if the robot is being commanded to be in teleop mode
pub fn is_operator_control() -> bool {
    unsafe { get_instance().IsOperatorControl() }
}

/// Check if the DS is commanding teleop mode and if it has enabled the robot.
///
/// @return True if the robot is being commanded to be in teleop mode and
/// enabled.
pub fn is_operator_control_enabled() -> bool {
    unsafe { get_instance().IsOperatorControlEnabled() }
}

/// Check if the DS is commanding test mode.
///
/// @return True if the robot is being commanded to be in test mode
pub fn is_test() -> bool {
    unsafe { get_instance().IsTest() }
}

/// Check if the DS is attached.
///
/// @return True if the DS is connected to the robot
pub fn is_ds_attatched() -> bool {
    unsafe { get_instance().IsDSAttached() }
}

/// Has a new control packet from the driver station arrived since the last
/// time this function was called?
///
/// Warning: If you call this function from more than one place at the same
/// time, you will not get the intended behavior.
///
/// @return True if the control data has been updated since the last call.
pub fn is_new_control_data() -> bool {
    unsafe { get_instance().IsNewControlData() }
}

/// Is the driver station attached to a Field Management System?
///
/// @return True if the robot is competing on a field being controlled by a
///         Field Management System
pub fn is_fms_attatched() -> bool {
    unsafe { get_instance().IsFMSAttached() }
}

/// Returns the game specific message provided by the FMS.
///
/// @return A string containing the game specific message.
pub fn get_game_specific_message() -> String {
    let info: *mut HAL_MatchInfo = ptr::null_mut();
    unsafe {
        // TODO use gameSpecificMessageSize;
        HAL_GetMatchInfo(info);
        to_string(info.as_ref().unwrap().gameSpecificMessage.as_ptr())
    }
}

/// Returns the name of the competition event provided by the FMS.
///
/// @return A string containing the event name
pub fn get_event_name() -> String {
    let info: *mut HAL_MatchInfo = ptr::null_mut();
    unsafe {
        // TODO test
        HAL_GetMatchInfo(info);
        to_string(info.as_ref().unwrap().eventName.as_ptr())
    }
}

/// Returns the type of match being played provided by the FMS.
///
/// @return The match type enum (kNone, kPractice, kQualification,
///         kElimination)
pub fn get_match_type() -> MatchType {
    unsafe { get_instance().GetMatchType() }
}

/// Returns the match number provided by the FMS.
///
/// @return The number of the match
pub fn get_match_number() -> i32 {
    unsafe { get_instance().GetMatchNumber() }
}

/// Returns the number of times the current match has been replayed from the
/// FMS.
///
/// @return The number of replays
pub fn get_replay_number() -> i32 {
    unsafe { get_instance().GetReplayNumber() }
}

/// Return the alliance that the driver station says it is on.
///
/// This could return kRed or kBlue.
///
/// @return The Alliance enum (kRed, kBlue or kInvalid)
pub fn get_alliance() -> Alliance {
    unsafe { get_instance().GetAlliance() }
}

/// Return the driver station location on the field.
///
/// This could return 1, 2, or 3.
///
/// @return The location of the driver station (1-3, 0 for invalid)
pub fn get_location() -> i32 {
    unsafe { get_instance().GetLocation() }
}

/// Wait until a new packet comes from the driver station.
///
/// This blocks on a semaphore, so the waiting is efficient.
///
/// This is a good way to delay processing until there is new driver station
/// data to act on.
///
/// Checks if new control data has arrived since the last waitForData call
/// on the current thread. If new data has not arrived, returns immediately.
pub fn wait_for_data() {
    unsafe {
        get_instance().WaitForData();
    }
}

/// Wait until a new packet comes from the driver station, or wait for a
/// timeout.
///
/// Checks if new control data has arrived since the last waitForData call
/// on the current thread. If new data has not arrived, returns immediately.
///
/// If the timeout is less then or equal to 0, wait indefinitely.
///
/// Timeout is in milliseconds
///
/// This blocks on a semaphore, so the waiting is efficient.
///
/// This is a good way to delay processing until there is new driver station
/// data to act on.
///
/// @param timeout Timeout time in seconds
///
/// @return true if new data, otherwise false
pub fn wait_for_data_with_timeout(timeout: Time) -> bool {
    unsafe { get_instance().WaitForData1(timeout.get::<second>()) }
}

/// Return the approximate match time.
///
/// The FMS does not send an official match time to the robots, but does send
/// an approximate match time. The value will count down the time remaining in
/// the current period (auto or teleop).
///
/// Warning: This is not an official time (so it cannot be used to dispute ref
/// calls or guarantee that a function will trigger before the match ends).
///
/// The Practice Match function of the DS approximates the behavior seen on
/// the field.
///
/// @return Time remaining in current match period (auto or teleop)
pub fn get_match_time() -> f64 {
    // TODO convert to time
    unsafe { get_instance().GetMatchTime() }
}

/// Read the battery voltage.
///
/// @return The battery voltage in Volts.
pub fn get_battery_voltage() -> ElectricPotential {
    unsafe { ElectricPotential::new::<volt>(get_instance().GetBatteryVoltage()) }
}

/// Only to be used to tell the Driver Station what code you claim to be
/// executing for diagnostic purposes only.
///
/// @param entering If true, starting disabled code; if false, leaving disabled
///                 code.
pub fn in_disabled(entering: bool) {
    unsafe {
        get_instance().m_userInDisabled = entering;
    }
}

/// Only to be used to tell the Driver Station what code you claim to be
/// executing for diagnostic purposes only.
///
/// @param entering If true, starting autonomous code; if false, leaving
///                 autonomous code.
pub fn in_autonomous(entering: bool) {
    unsafe {
        get_instance().m_userInAutonomous = entering;
    }
}

/// Only to be used to tell the Driver Station what code you claim to be
/// executing for diagnostic purposes only.
///
/// @param entering If true, starting teleop code; if false, leaving teleop
///                 code.
pub fn in_operator_control(entering: bool) {
    unsafe {
        get_instance().m_userInTeleop = entering;
    }
}

/// Only to be used to tell the Driver Station what code you claim to be
/// executing for diagnostic purposes only.
///
/// @param entering If true, starting test code; if false, leaving test code.
pub fn in_test(entering: bool) {
    unsafe {
        get_instance().m_userInTest = entering;
    }
}

/// Forces WaitForData() to return immediately.
pub fn wakeup_wait_for_data() {
    unsafe {
        get_instance().WakeupWaitForData();
    }
}

/// Allows the user to specify whether they want joystick connection warnings
/// to be printed to the console. This setting is ignored when the FMS is
/// connected -- warnings will always be on in that scenario.
///
/// @param silence Whether warning messages should be silenced.
pub fn silence_joystick_connection_warning(silence: bool) {
    unsafe {
        get_instance().SilenceJoystickConnectionWarning(silence);
    }
}

/// Returns whether joystick connection warnings are silenced. This will
/// always return false when connected to the FMS.
///
/// @return Whether joystick connection warnings are silenced.
pub fn is_joystick_connection_warning_silenced() -> bool {
    unsafe { get_instance().IsJoystickConnectionWarningSilenced() }
}
