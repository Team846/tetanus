pub fn start_robot(start_competition: extern "C" fn(), end_competition: extern "C" fn()) {
    unsafe {
        frc_sys::wpilib::frc_StartRobotRs(Some(start_competition), Some(end_competition));
    }
}
