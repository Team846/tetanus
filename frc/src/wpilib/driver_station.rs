use std::ffi::CStr;
use std::ptr;

use frc_sys::hal::HAL_GetMatchInfo;
use frc_sys::hal::HAL_MatchInfo;
use frc_sys::hal::{HAL_GetJoystickDescriptor, HAL_JoystickDescriptor};
use frc_sys::wpilib::frc_DriverStation;
use uom::si::f64::*;
use uom::si::time::second;

pub enum Alliance {
    Red,
    Blue,
    Invalid,
}

impl From<u32> for Alliance {
    fn from(ordinal: u32) -> Self {
        match ordinal {
            0 => Alliance::Red,
            1 => Alliance::Blue,
            2 => Alliance::Invalid,
            _ => panic!("Invalid Alliance ordinal"),
        }
    }
}

pub enum MatchType {
    None,
    Practice,
    Qualification,
    Elimination,
}

impl From<u32> for MatchType {
    fn from(ordinal: u32) -> Self {
        match ordinal {
            0 => MatchType::None,
            1 => MatchType::Practice,
            2 => MatchType::Qualification,
            3 => MatchType::Elimination,
            _ => panic!("Invalid MatchType ordinal"),
        }
    }
}

unsafe fn get_instance<'a>() -> &'a mut frc_DriverStation {
    frc_DriverStation::GetInstance().as_mut().unwrap()
}

fn to_string(ptr: *const u8) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
}

pub fn get_stick_button(stick: i32, button: i32) -> bool {
    unsafe { get_instance().GetStickButton(stick, button) }
}

pub fn get_stick_button_pressed(stick: i32, button: i32) -> bool {
    unsafe { get_instance().GetStickButtonPressed(stick, button) }
}

pub fn get_stick_button_released(stick: i32, button: i32) -> bool {
    unsafe { get_instance().GetStickButtonReleased(stick, button) }
}

pub fn get_stick_axis(stick: i32, axis: i32) -> f64 {
    unsafe { get_instance().GetStickAxis(stick, axis) }
}

pub fn get_stick_pov(stick: i32, pov: i32) -> i32 {
    unsafe { get_instance().GetStickPOV(stick, pov) }
}

pub fn get_stick_buttons(stick: i32) -> i32 {
    unsafe { get_instance().GetStickButtons(stick) }
}

pub fn get_stick_axis_count(stick: i32) -> i32 {
    unsafe { get_instance().GetStickAxisCount(stick) }
}

pub fn get_stick_pov_count(stick: i32) -> i32 {
    unsafe { get_instance().GetStickPOVCount(stick) }
}

pub fn get_stick_button_count(stick: i32) -> i32 {
    unsafe { get_instance().GetStickButtonCount(stick) }
}

pub fn get_joystick_is_xbox(stick: i32) -> bool {
    unsafe { get_instance().GetJoystickIsXbox(stick) }
}

pub fn get_joystick_type(stick: i32) -> i32 {
    unsafe { get_instance().GetJoystickType(stick) }
}

pub fn get_joystick_name(stick: i32) -> String {
    get_joystick_type(stick); // Check if stick is in range
    unsafe {
        // TODO test
        let desc: *mut HAL_JoystickDescriptor = ptr::null_mut();
        HAL_GetJoystickDescriptor(stick, desc);
        to_string(desc.as_ref().unwrap().name.as_ptr())
    }
}

pub fn get_joystick_axis_type(stick: i32, axis: i32) -> i32 {
    unsafe { get_instance().GetJoystickAxisType(stick, axis) }
}

pub fn is_joystick_connected(stick: i32) -> bool {
    unsafe { get_instance().IsJoystickConnected(stick) }
}

pub fn is_enabled() -> bool {
    unsafe { get_instance().IsEnabled() }
}

pub fn is_disabled() -> bool {
    unsafe { get_instance().IsDisabled() }
}

pub fn is_estopped() -> bool {
    unsafe { get_instance().IsEStopped() }
}

pub fn is_autonomous() -> bool {
    unsafe { get_instance().IsAutonomous() }
}

pub fn is_autonomous_enabled() -> bool {
    unsafe { get_instance().IsAutonomousEnabled() }
}

pub fn is_operator_control() -> bool {
    unsafe { get_instance().IsOperatorControl() }
}

pub fn is_operator_control_enabled() -> bool {
    unsafe { get_instance().IsOperatorControlEnabled() }
}

pub fn is_test() -> bool {
    unsafe { get_instance().IsTest() }
}

pub fn is_ds_attatched() -> bool {
    unsafe { get_instance().IsDSAttached() }
}

pub fn is_new_control_data() -> bool {
    unsafe { get_instance().IsNewControlData() }
}

pub fn is_fms_attatched() -> bool {
    unsafe { get_instance().IsFMSAttached() }
}

pub fn get_game_specific_message() -> String {
    let info: *mut HAL_MatchInfo = ptr::null_mut();
    unsafe {
        // TODO use gameSpecificMessageSize;
        HAL_GetMatchInfo(info);
        to_string(info.as_ref().unwrap().gameSpecificMessage.as_ptr())
    }
}

pub fn get_event_name() -> String {
    let info: *mut HAL_MatchInfo = ptr::null_mut();
    unsafe {
        // TODO test
        HAL_GetMatchInfo(info);
        to_string(info.as_ref().unwrap().eventName.as_ptr())
    }
}

pub fn get_match_type() -> MatchType {
    unsafe { MatchType::from(get_instance().GetMatchType()) }
}

pub fn get_match_number() -> i32 {
    unsafe { get_instance().GetMatchNumber() }
}

pub fn get_replay_number() -> i32 {
    unsafe { get_instance().GetReplayNumber() }
}

pub fn get_alliance() -> Alliance {
    unsafe { Alliance::from(get_instance().GetAlliance()) }
}

pub fn get_location() -> i32 {
    unsafe { get_instance().GetLocation() }
}

pub fn wait_for_data() {
    unsafe { get_instance().WaitForData() }
}

pub fn wait_for_data_with_timeout(timeout: Time) -> bool {
    unsafe { get_instance().WaitForData1(timeout.get::<second>()) }
}

pub fn get_match_time() -> f64 {
    unsafe { get_instance().GetMatchTime() }
}

pub fn get_battery_voltage() -> f64 {
    unsafe { get_instance().GetBatteryVoltage() }
}

pub fn in_disabled(entering: bool) {
    unsafe { get_instance().m_userInDisabled = entering }
}

pub fn in_autonomous(entering: bool) {
    unsafe { get_instance().m_userInAutonomous = entering }
}

pub fn in_operator_control(entering: bool) {
    unsafe { get_instance().m_userInTeleop = entering }
}

pub fn in_test(entering: bool) {
    unsafe { get_instance().m_userInTest = entering }
}

pub fn wakeup_wait_for_data() {
    unsafe { get_instance().WakeupWaitForData() }
}

pub fn silence_joystick_connection_warning(silence: bool) {
    unsafe { get_instance().SilenceJoystickConnectionWarning(silence) }
}

pub fn is_joystick_connection_warning_silenced() -> bool {
    unsafe { get_instance().IsJoystickConnectionWarningSilenced() }
}
