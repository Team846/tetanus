use crate::wpilib::pwm::PwmType;
use frc_sys::wpilib;
use frc_sys::wpilib::frc_MotorSafety;
use frc_sys::wpilib::frc_PWM;
use frc_sys::wpilib::frc_Servo;
use std::ffi::c_void;
use uom::si::angle::degree;
use uom::si::f64::*;

pub struct Servo {
    pub handle: frc_Servo,
}

impl Servo {
    pub fn new(channel: i32) -> Self {
        Servo {
            handle: unsafe { frc_Servo::new(channel) },
        }
    }

    pub fn set(&mut self, value: f64) {
        unsafe {
            self.handle.Set(value);
        }
    }

    pub fn set_offline(&mut self) {
        unsafe {
            self.handle.SetOffline();
        }
    }

    pub fn get(&mut self) -> f64 {
        unsafe { self.handle.Get() }
    }

    pub fn set_angle(&mut self, angle: Angle) {
        unsafe {
            self.handle.SetAngle(angle.get::<degree>());
        }
    }

    pub fn get_angle(&mut self) -> Angle {
        unsafe { Angle::new::<degree>(self.handle.GetAngle()) }
    }

    pub fn get_max_angle(&mut self) -> Angle {
        unsafe { Angle::new::<degree>(self.handle.GetMaxAngle()) }
    }

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
