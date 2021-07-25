use frc::hal;
use frc::wpilib::generic_hid::JoystickHand;
use frc::wpilib::GenericHID;
use frc::wpilib::XboxController;
use frc::wpilib::{driver_station, robot_base::start_robot};
use uom::si::f64::*;
use uom::si::time::millisecond;

fn main() {
    println!("🦀🤖");
    start_robot(start_competition, end_competition);
}

extern "C" fn start_competition() {
    let mut driver = XboxController::new(0);

    unsafe {
        hal::HAL_ObserveUserProgramStarting();
    }

    loop {
        driver_station::wait_for_data_with_timeout(Time::new::<millisecond>(20.0));

        let strafe_x = driver.get_x(JoystickHand::kLeftHand);
        let strafe_y = driver.get_y(JoystickHand::kLeftHand);

        println!("x {}, y {}", strafe_x, strafe_y);
    }
}

extern "C" fn end_competition() {
    println!("End");
}
