use crate::wpilib::pwm::PwmType;
use frc_sys::wpilib;
use frc_sys::wpilib::frc_MotorSafety;
use frc_sys::wpilib::frc_PWM;
use frc_sys::wpilib::frc_Servo;
use std::ffi::c_void;
use uom::si::angle::degree;
use uom::si::f64::*;

/// Standard hobby style servo.
///
/// The range parameters default to the appropriate values for the Hitec HS-322HD
/// servo provided in the FIRST Kit of Parts in 2008.
pub struct Servo {
    pub handle: frc_Servo,
}

impl Servo {
    /// @param channel The PWM channel to which the servo is attached. 0-9 are
    ///                on-board, 10-19 are on the MXP port
    pub fn new(channel: i32) -> Self {
        Servo {
            handle: unsafe { frc_Servo::new(channel) },
        }
    }

    /// Set the servo position.
    ///
    /// Servo values range from 0.0 to 1.0 corresponding to the range of full left
    /// to full right.
    ///
    /// @param value Position from 0.0 to 1.0.
    pub fn set(&mut self, value: f64) {
        unsafe {
            self.handle.Set(value);
        }
    }

    /// Set the servo to offline.
    ///
    /// Set the servo raw value to 0 (undriven)
    pub fn set_offline(&mut self) {
        unsafe {
            self.handle.SetOffline();
        }
    }

    /// Get the servo position.
    ///
    /// Servo values range from 0.0 to 1.0 corresponding to the range of full left
    /// to full right.
    ///
    /// @return Position from 0.0 to 1.0.
    pub fn get(&mut self) -> f64 {
        unsafe { self.handle.Get() }
    }

    /// Set the servo angle.
    ///
    /// Assume that the servo angle is linear with respect to the PWM value (big
    /// assumption, need to test).
    ///
    /// Servo angles that are out of the supported range of the servo simply
    /// "saturate" in that direction. In other words, if the servo has a range of
    /// (X degrees to Y degrees) than angles of less than X result in an angle of
    /// X being set and angles of more than Y degrees result in an angle of Y being
    /// set.
    ///
    /// @param degrees The angle in degrees to set the servo.
    // pub fn set_angle(&mut self, angle: Angle) {
    //     unsafe {
    //         self.handle.SetAngle(angle.get::<degree>());
    //     }
    // }
    pub fn set_angle(&mut self, angle: Angle) {
        unsafe {
            self.handle.SetAngle(angle.get::<degree>());
        }
    }

    /// Get the servo angle.
    ///
    /// Assume that the servo angle is linear with respect to the PWM value (big
    /// assumption, need to test).
    ///
    /// @return The angle in degrees to which the servo is set.
    pub fn get_angle(&mut self) -> Angle {
        unsafe { Angle::new::<degree>(self.handle.GetAngle()) }
    }

    /// Get the maximum angle of the servo.
    ///
    /// @return The maximum angle of the servo in degrees.
    pub fn get_max_angle(&mut self) -> Angle {
        unsafe { Angle::new::<degree>(self.handle.GetMaxAngle()) }
    }

    /// Get the minimum angle of the servo.
    ///
    /// @return The minimum angle of the servo in degrees.
    pub fn get_min_angle(&mut self) -> Angle {
        unsafe { Angle::new::<degree>(self.handle.GetMinAngle()) }
    }
}

impl PwmType for Servo {
    fn get_handle_ptr(&mut self) -> *mut c_void {
        &mut self.handle as *mut _ as *mut c_void
    }
}

impl Drop for Servo {
    fn drop(&mut self) {
        unsafe {
            wpilib::frc_PWM_PWM_destructor(&mut self.handle as *mut _ as *mut frc_PWM);
            wpilib::frc_MotorSafety_MotorSafety_destructor(
                &mut self.handle as *mut _ as *mut frc_MotorSafety,
            )
        }
    }
}
