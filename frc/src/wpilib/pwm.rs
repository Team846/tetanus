use frc_sys::wpilib::{self, frc_MotorSafety};
use frc_sys::wpilib::{frc_PWM, frc_PWM_PeriodMultiplier};
use std::ffi::c_void;

/// Represents the amount to multiply the minimum servo-pulse pwm period by.
pub type PeriodMultiplier = frc_PWM_PeriodMultiplier;

pub trait PwmType {
    fn get_handle_ptr(&mut self) -> *mut c_void;

    /// Set the PWM value directly to the hardware.
    ///
    /// Write a raw value to a PWM channel.
    ///
    /// @param value Raw PWM value.
    fn set_raw(&mut self, value: u16) {
        unsafe {
            wpilib::frc_PWM_SetRaw(self.get_handle_ptr(), value);
        }
    }

    /// Get the PWM value directly from the hardware.
    ///
    /// Read a raw value from a PWM channel.
    ///
    /// @return Raw PWM control value.
    fn get_raw(&mut self) -> u16 {
        unsafe { wpilib::frc_PWM_GetRaw(self.get_handle_ptr()) }
    }

    /// Set the PWM value based on a position.
    ///
    /// This is intended to be used by servos.
    ///
    /// @pre SetMaxPositivePwm() called.
    /// @pre SetMinNegativePwm() called.
    ///
    /// @param pos The position to set the servo between 0.0 and 1.0.
    fn set_position(&mut self, pos: f64) {
        unsafe {
            wpilib::frc_PWM_SetPosition(self.get_handle_ptr(), pos);
        }
    }

    /// Get the PWM value in terms of a position.
    ///
    /// This is intended to be used by servos.
    ///
    /// @pre SetMaxPositivePwm() called.
    /// @pre SetMinNegativePwm() called.
    ///
    /// @return The position the servo is set to between 0.0 and 1.0.
    fn get_position(&mut self) -> f64 {
        unsafe { wpilib::frc_PWM_GetPosition(self.get_handle_ptr()) }
    }

    /// Set the PWM value based on a speed.
    ///
    /// This is intended to be used by speed controllers.
    ///
    /// @pre SetMaxPositivePwm() called.
    /// @pre SetMinPositivePwm() called.
    /// @pre SetCenterPwm() called.
    /// @pre SetMaxNegativePwm() called.
    /// @pre SetMinNegativePwm() called.
    ///
    /// @param speed The speed to set the speed controller between -1.0 and 1.0.
    fn set_speed(&mut self, speed: f64) {
        unsafe {
            wpilib::frc_PWM_SetSpeed(self.get_handle_ptr(), speed);
        }
    }

    /// Get the PWM value in terms of speed.
    ///
    /// This is intended to be used by speed controllers.
    ///
    /// @pre SetMaxPositivePwm() called.
    /// @pre SetMinPositivePwm() called.
    /// @pre SetMaxNegativePwm() called.
    /// @pre SetMinNegativePwm() called.
    ///
    /// @return The most recently set speed between -1.0 and 1.0.
    fn get_speed(&mut self) -> f64 {
        unsafe { wpilib::frc_PWM_GetSpeed(self.get_handle_ptr()) }
    }

    /// Temporarily disables the PWM output. The next set call will reenable
    /// the output.
    fn set_disabled(&mut self) {
        unsafe { wpilib::frc_PWM_SetDisabled(self.get_handle_ptr()) }
    }

    /// Slow down the PWM signal for old devices.
    ///
    /// @param mult The period multiplier to apply to this channel
    fn set_period_multiplier(&mut self, mult: PeriodMultiplier) {
        unsafe {
            wpilib::frc_PWM_SetPeriodMultiplier(self.get_handle_ptr() as *mut frc_PWM, mult);
        }
    }

    fn set_zero_latch(&mut self) {
        unsafe { wpilib::frc_PWM_SetZeroLatch(self.get_handle_ptr() as *mut frc_PWM) }
    }

    /// Optionally eliminate the deadband from a speed controller.
    ///
    /// @param eliminateDeadband If true, set the motor curve on the Jaguar to
    ///                          eliminate the deadband in the middle of the range.
    ///                          Otherwise, keep the full range without modifying
    ///                          any values.
    fn enable_deadband_elimination(&mut self, eliminate_deadband: bool) {
        unsafe {
            wpilib::frc_PWM_EnableDeadbandElimination(
                self.get_handle_ptr() as *mut frc_PWM,
                eliminate_deadband,
            );
        }
    }

    /// Set the bounds on the PWM pulse widths.
    ///
    /// This sets the bounds on the PWM values for a particular type of controller.
    /// The values determine the upper and lower speeds as well as the deadband
    /// bracket.
    ///
    /// @param max         The max PWM pulse width in ms
    /// @param deadbandMax The high end of the deadband range pulse width in ms
    /// @param center      The center (off) pulse width in ms
    /// @param deadbandMin The low end of the deadband pulse width in ms
    /// @param min         The minimum pulse width in ms
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

    /// Set the bounds on the PWM values.
    ///
    /// This sets the bounds on the PWM values for a particular each type of
    /// controller. The values determine the upper and lower speeds as well as the
    /// deadband bracket.
    ///
    /// @param max         The Minimum pwm value
    /// @param deadbandMax The high end of the deadband range
    /// @param center      The center speed (off)
    /// @param deadbandMin The low end of the deadband range
    /// @param min         The minimum pwm value
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

    /// Get the bounds on the PWM values.
    ///
    /// This Gets the bounds on the PWM values for a particular each type of
    /// controller. The values determine the upper and lower speeds as well as the
    /// deadband bracket.
    ///
    /// @param max         The Minimum pwm value
    /// @param deadbandMax The high end of the deadband range
    /// @param center      The center speed (off)
    /// @param deadbandMin The low end of the deadband range
    /// @param min         The minimum pwm value
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

/// Class implements the PWM generation in the FPGA.
///
/// The values supplied as arguments for PWM outputs range from -1.0 to 1.0. They
/// are mapped to the hardware dependent values, in this case 0-2000 for the
/// FPGA. Changes are immediately sent to the FPGA, and the update occurs at the
/// next FPGA cycle (5.005ms). There is no delay.
///
/// As of revision 0.1.10 of the FPGA, the FPGA interprets the 0-2000 values as
/// follows:
///   - 2000 = maximum pulse width
///   - 1999 to 1001 = linear scaling from "full forward" to "center"
///   - 1000 = center value
///   - 999 to 2 = linear scaling from "center" to "full reverse"
///   - 1 = minimum pulse width (currently 0.5ms)
///   - 0 = disabled (i.e. PWM output is held low)
pub struct Pwm {
    handle: frc_PWM,
}

impl Pwm {
    /// Allocate a PWM given a channel number.
    ///
    /// Checks channel value range and allocates the appropriate channel.
    /// The allocation is only done to help users ensure that they don't double
    /// assign channels.
    ///
    /// @param channel The PWM channel number. 0-9 are on-board, 10-19 are on the
    ///                MXP port
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
            wpilib::frc_MotorSafety_MotorSafety_destructor(
                &mut self.handle as *mut _ as *mut frc_MotorSafety,
            );
        }
    }
}
