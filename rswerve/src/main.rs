use frc::hal;
use frc::wpilib::robot_controller;
use frc::wpilib::Servo;
use frc::wpilib::{driver_station, robot_base::start_robot};
use uom::si::angle::degree;
use uom::si::f64::*;
use uom::si::time::millisecond;

fn main() {
    println!("🦀🤖");
    start_robot(start_competition, end_competition);
}

extern "C" fn start_competition() {
    let mut servo = Servo::new(0);

    unsafe {
        hal::HAL_ObserveUserProgramStarting();
    }

    let mut is_enabled = false;
    loop {
        driver_station::wait_for_data_with_timeout(Time::new::<millisecond>(20.0));

        if !is_enabled && driver_station::is_enabled() {
            println!("Enabled");
            is_enabled = true;
        } else if is_enabled && driver_station::is_disabled() {
            println!("Disabled");
            is_enabled = false;
        }

        if is_enabled {
            if robot_controller::get_user_button() {
                servo.set_angle(Angle::new::<degree>(90.0));
            } else {
                servo.set_angle(Angle::new::<degree>(0.0));
            }
        }
    }
}

extern "C" fn end_competition() {
    println!("End");
}
