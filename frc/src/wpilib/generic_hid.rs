use std::ffi::c_void;

use frc_sys::wpilib::{
    self, frc_GenericHID, frc_GenericHID_HIDType, frc_GenericHID_JoystickHand,
    frc_GenericHID_RumbleType,
};

pub type RumbleType = frc_GenericHID_RumbleType;
pub type HIDType = frc_GenericHID_HIDType;
pub type JoystickHand = frc_GenericHID_JoystickHand;

pub trait GenericHID {
    fn get_handle_ptr(&mut self) -> *mut c_void;

    fn get_x(&mut self, hand: JoystickHand /* kRightHand */) -> f64;

    fn get_y(&mut self, hand: JoystickHand /* kRightHand */) -> f64;

    fn get_raw_button(&mut self, button: i32) -> bool {
        unsafe {
            wpilib::frc_GenericHID_GetRawButton(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID,
                button,
            )
        }
    }

    fn get_raw_button_pressed(&mut self, button: i32) -> bool {
        unsafe {
            wpilib::frc_GenericHID_GetRawButtonPressed(
                self.get_handle_ptr() as *mut _ as *mut frc_GenericHID,
                button,
            )
        }
    }

    fn get_raw_button_released(&mut self, button: i32) -> bool {
        unsafe {
            wpilib::frc_GenericHID_GetRawButtonReleased(
                self.get_handle_ptr() as *mut _ as *mut frc_GenericHID,
                button,
            )
        }
    }

    fn get_raw_axis(&mut self, axis: i32) -> f64 {
        unsafe {
            wpilib::frc_GenericHID_GetRawAxis(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID,
                axis,
            )
        }
    }

    fn get_pov(&mut self, pov: i32 /* 0 */) -> i32 {
        unsafe {
            wpilib::frc_GenericHID_GetPOV(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID,
                pov,
            )
        }
    }

    fn get_axis_count(&mut self) -> i32 {
        unsafe {
            wpilib::frc_GenericHID_GetAxisCount(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID
            )
        }
    }

    fn get_pov_count(&mut self) -> i32 {
        unsafe {
            wpilib::frc_GenericHID_GetPOVCount(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID
            )
        }
    }

    fn get_button_count(&mut self) -> i32 {
        unsafe {
            wpilib::frc_GenericHID_GetButtonCount(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID
            )
        }
    }

    fn is_connected(&mut self) -> bool {
        unsafe {
            wpilib::frc_GenericHID_IsConnected(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID
            )
        }
    }

    fn get_type(&mut self) -> HIDType {
        unsafe {
            wpilib::frc_GenericHID_GetType(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID
            )
        }
    }

    // fn get_name(&mut self) -> String { }

    fn get_axis_type(&mut self, axis: i32) -> i32 {
        unsafe {
            wpilib::frc_GenericHID_GetAxisType(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID,
                axis,
            )
        }
    }

    fn get_port(&mut self) -> i32 {
        unsafe {
            wpilib::frc_GenericHID_GetPort(
                self.get_handle_ptr() as *const _ as *const frc_GenericHID
            )
        }
    }

    fn set_output(&mut self, output_number: i32, value: bool) {
        unsafe {
            wpilib::frc_GenericHID_SetOutput(
                self.get_handle_ptr() as *mut _ as *mut frc_GenericHID,
                output_number,
                value,
            );
        }
    }

    fn set_outputs(&mut self, value: i32) {
        unsafe {
            wpilib::frc_GenericHID_SetOutputs(
                self.get_handle_ptr() as *mut _ as *mut frc_GenericHID,
                value,
            );
        }
    }

    fn set_rumble(&mut self, rumble_type: RumbleType, value: f64) {
        unsafe {
            wpilib::frc_GenericHID_SetRumble(
                self.get_handle_ptr() as *mut _ as *mut frc_GenericHID,
                rumble_type,
                value,
            );
        }
    }
}
