use std::ffi::c_void;

use frc_sys::wpilib::{
    self, frc_XboxController, frc_XboxController_Axis, frc_XboxController_Button,
};

use super::{generic_hid::JoystickHand, GenericHID};

pub type Button = frc_XboxController_Button;
pub type Axis = frc_XboxController_Axis;

pub struct XboxController {
    handle: frc_XboxController,
}

impl XboxController {
    pub fn new(port: i32) -> Self {
        XboxController {
            handle: unsafe { frc_XboxController::new(port) },
        }
    }

    pub fn get_trigger_axis(&mut self, hand: JoystickHand) -> f64 {
        unsafe { self.handle.GetTriggerAxis(hand) }
    }

    pub fn get_bumper(&mut self, hand: JoystickHand) -> bool {
        unsafe { self.handle.GetBumper(hand) }
    }

    pub fn get_bumper_pressed(&mut self, hand: JoystickHand) -> bool {
        unsafe { self.handle.GetBumperPressed(hand) }
    }

    pub fn get_bumper_release(&mut self, hand: JoystickHand) -> bool {
        unsafe { self.handle.GetBumperReleased(hand) }
    }

    pub fn get_stick_button(&mut self, hand: JoystickHand) -> bool {
        unsafe { self.handle.GetStickButton(hand) }
    }

    pub fn get_stick_button_pressed(&mut self, hand: JoystickHand) -> bool {
        unsafe { self.handle.GetStickButtonPressed(hand) }
    }

    pub fn get_stick_button_release(&mut self, hand: JoystickHand) -> bool {
        unsafe { self.handle.GetStickButtonReleased(hand) }
    }

    pub fn get_a_button(&mut self) -> bool {
        unsafe { self.handle.GetAButton() }
    }

    pub fn get_a_button_pressed(&mut self) -> bool {
        unsafe { self.handle.GetAButtonPressed() }
    }

    pub fn get_a_button_release(&mut self) -> bool {
        unsafe { self.handle.GetAButtonReleased() }
    }

    pub fn get_b_button(&mut self) -> bool {
        unsafe { self.handle.GetBButton() }
    }

    pub fn get_b_button_pressed(&mut self) -> bool {
        unsafe { self.handle.GetBButtonPressed() }
    }

    pub fn get_b_button_release(&mut self) -> bool {
        unsafe { self.handle.GetBButtonReleased() }
    }

    pub fn get_x_button(&mut self) -> bool {
        unsafe { self.handle.GetXButton() }
    }

    pub fn get_x_button_pressed(&mut self) -> bool {
        unsafe { self.handle.GetXButtonPressed() }
    }

    pub fn get_x_button_release(&mut self) -> bool {
        unsafe { self.handle.GetXButtonReleased() }
    }

    pub fn get_y_button(&mut self) -> bool {
        unsafe { self.handle.GetYButton() }
    }

    pub fn get_y_button_pressed(&mut self) -> bool {
        unsafe { self.handle.GetYButtonPressed() }
    }

    pub fn get_y_button_release(&mut self) -> bool {
        unsafe { self.handle.GetYButtonReleased() }
    }

    pub fn get_back_button(&mut self) -> bool {
        unsafe { self.handle.GetBackButton() }
    }

    pub fn get_back_button_pressed(&mut self) -> bool {
        unsafe { self.handle.GetBackButtonPressed() }
    }

    pub fn get_back_button_release(&mut self) -> bool {
        unsafe { self.handle.GetBackButtonReleased() }
    }

    pub fn get_start_button(&mut self) -> bool {
        unsafe { self.handle.GetStartButton() }
    }

    pub fn get_start_button_pressed(&mut self) -> bool {
        unsafe { self.handle.GetStartButtonPressed() }
    }

    pub fn get_start_button_release(&mut self) -> bool {
        unsafe { self.handle.GetStartButtonReleased() }
    }
}

impl GenericHID for XboxController {
    fn get_handle_ptr(&self) -> *const c_void {
        &self.handle as *const _ as *const c_void
    }

    fn get_x(&self, hand: JoystickHand /* kRightHand */) -> f64 {
        unsafe { wpilib::frc_XboxController_GetX(self.get_handle_ptr() as *mut _, hand) }
    }

    fn get_y(&self, hand: JoystickHand /* kRightHand */) -> f64 {
        unsafe { wpilib::frc_XboxController_GetY(self.get_handle_ptr() as *mut _, hand) }
    }
}
