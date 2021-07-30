use frc::wpilib::robot_base;

mod funky_robot;
mod subsystems;

fn main() {
    println!("🦀🤖");
    robot_base::start_robot(start_competition, end_competition);
}

extern "C" fn start_competition() {
    match funky_robot::start_competition() {
        Ok(()) => {}
        Err(e) => panic!("Unhandled error {}", e),
    }
}

extern "C" fn end_competition() {
    funky_robot::end_competition();
}
