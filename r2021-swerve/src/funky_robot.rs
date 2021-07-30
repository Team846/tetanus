use std::sync::{Arc, Mutex};

use anyhow::Result;
use frc::{hal, wpilib::driver_station};
use tetanus_core::node::{BaseNode, Node, NodeReceiver};
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::si::time::millisecond;

use crate::subsystems::{
    driver::Driver,
    drivetrain::{Drivetrain, DrivetrainMsg},
};

pub fn start_competition() -> Result<()> {
    let mut ds_ticker = BaseNode::new();

    let robot = FunkyRobot::new();

    ds_ticker
        .produce(robot.driver)
        .map(|msg| DrivetrainMsg {
            left: Ratio::new::<ratio>(msg.left_stick_y),
            right: Ratio::new::<ratio>(msg.right_stick_y),
        })
        .consume(robot.drivetrain);

    unsafe {
        hal::HAL_ObserveUserProgramStarting();
    }

    loop {
        driver_station::wait_for_data_with_timeout(Time::new::<millisecond>(20.0));
        if driver_station::is_operator_control_enabled() {
            unsafe {
                hal::HAL_ObserveUserProgramTeleop();
            }
        } else if driver_station::is_disabled() {
            unsafe {
                hal::HAL_ObserveUserProgramDisabled();
            }
        }
        ds_ticker.send(());
    }
}

pub fn end_competition() {
    println!("End");
}

pub struct FunkyRobot {
    driver: Arc<Mutex<Driver>>,
    drivetrain: Arc<Mutex<Drivetrain>>,
}

impl FunkyRobot {
    pub fn new() -> Self {
        FunkyRobot {
            driver: Arc::new(Mutex::new(Driver::new())),
            drivetrain: Arc::new(Mutex::new(Drivetrain::new())),
        }
    }
}
