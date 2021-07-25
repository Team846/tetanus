use frc_sys::wpilib;
use frc_sys::wpilib::{frc_PWM, frc_PWM_PeriodMultiplier};
use std::ffi::c_void;

pub type PeriodMultiplier = frc_PWM_PeriodMultiplier;

pub trait PwmType {
    fn get_handle_ptr(&mut self) -> *mut c_void;

    fn set_raw(&mut self, value: u16) {
        unsafe {
            wpilib::frc_PWM_SetRaw(self.get_handle_ptr(), value);
        }
    }

    fn get_raw(&mut self) -> u16 {
        unsafe { wpilib::frc_PWM_GetRaw(self.get_handle_ptr()) }
    }

    fn set_position(&mut self, pos: f64) {
        unsafe {
            wpilib::frc_PWM_SetPosition(self.get_handle_ptr(), pos);
        }
    }

    fn get_position(&mut self) -> f64 {
        unsafe { wpilib::frc_PWM_GetPosition(self.get_handle_ptr()) }
    }

    fn set_speed(&mut self, speed: f64) {
        unsafe {
            wpilib::frc_PWM_SetSpeed(self.get_handle_ptr(), speed);
        }
    }

    fn get_speed(&mut self) -> f64 {
        unsafe { wpilib::frc_PWM_GetSpeed(self.get_handle_ptr()) }
    }

    fn set_disabled(&mut self) {
        unsafe { wpilib::frc_PWM_SetDisabled(self.get_handle_ptr()) }
    }

    fn set_period_multiplier(&mut self, mult: PeriodMultiplier) {
        unsafe {
            wpilib::frc_PWM_SetPeriodMultiplier(self.get_handle_ptr() as *mut frc_PWM, mult);
        }
    }

    fn set_zero_latch(&mut self) {
        unsafe { wpilib::frc_PWM_SetZeroLatch(self.get_handle_ptr() as *mut frc_PWM) }
    }

    fn enable_deadband_elimination(&mut self, eliminate_deadband: bool) {
        unsafe {
            wpilib::frc_PWM_EnableDeadbandElimination(
                self.get_handle_ptr() as *mut frc_PWM,
                eliminate_deadband,
            );
        }
    }

    fn set_bounds(
        &mut self,
        max: f64,
        deadband_max: f64,
        center: f64,
        deadband_min: f64,
        min: f64,
    ) {
        unsafe {
            wpilib::frc_PWM_SetBounds(
                self.get_handle_ptr() as *mut frc_PWM,
                max,
                deadband_max,
                center,
                deadband_min,
                min,
            );
        }
    }

    fn set_raw_bounds(
        &mut self,
        max: i32,
        deadband_max: i32,
        center: i32,
        deadband_min: i32,
        min: i32,
    ) {
        unsafe {
            wpilib::frc_PWM_SetRawBounds(
                self.get_handle_ptr() as *mut frc_PWM,
                max,
                deadband_max,
                center,
                deadband_min,
                min,
            );
        }
    }

    fn get_raw_bounds(&mut self) -> (i32, i32, i32, i32, i32) {
        let mut bounds = (0, 0, 0, 0, 0);
        unsafe {
            wpilib::frc_PWM_GetRawBounds(
                self.get_handle_ptr() as *mut frc_PWM,
                &mut bounds.0 as *mut i32,
                &mut bounds.1 as *mut i32,
                &mut bounds.2 as *mut i32,
                &mut bounds.3 as *mut i32,
                &mut bounds.4 as *mut i32,
            );
        }
        bounds
    }

    fn get_channel(&mut self) -> i32 {
        unsafe { wpilib::frc_PWM_GetChannel(self.get_handle_ptr() as *const frc_PWM) }
    }
}

pub struct Pwm {
    handle: frc_PWM,
}

impl Pwm {
    pub fn new(channel: i32) -> Self {
        Pwm {
            handle: unsafe { frc_PWM::new(channel) },
        }
    }
}

impl PwmType for Pwm {
    fn get_handle_ptr(&mut self) -> *mut c_void {
        &mut self.handle as *mut _ as *mut c_void
    }
}

impl Drop for Pwm {
    fn drop(&mut self) {
        unsafe {
            wpilib::frc_PWM_PWM_destructor(&mut self.handle);
        }
    }
}
