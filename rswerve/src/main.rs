use frc::hal;
use frc::wpilib::driver_station;
use frc::wpilib::robot_base::start_robot;
use uom::si::f64::*;
use uom::si::time::second;

fn main() {
    println!("Entry");
    start_robot(start_competition, end_competition)
}

extern "C" fn start_competition() {
    unsafe { hal::HAL_ObserveUserProgramStarting() };

    loop {
        driver_station::wait_for_data_with_timeout(Time::new::<second>(2.0));
        println!("{}", driver_station::get_battery_voltage());
    }
}

extern "C" fn end_competition() {
    println!("End");
}
