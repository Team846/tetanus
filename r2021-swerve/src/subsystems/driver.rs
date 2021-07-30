use frc::wpilib::{generic_hid::JoystickHand, GenericHID, XboxController};
use tetanus_core::producer::Producer;

#[derive(Clone, Copy, Debug)]
pub struct DriverMsg {
    pub left_stick_y: f64,
    pub right_stick_y: f64,
}

pub struct Driver {
    controller: XboxController,
}

impl Driver {
    const XBOX_PORT: i32 = 1;

    pub fn new() -> Self {
        Driver {
            controller: XboxController::new(Self::XBOX_PORT),
        }
    }
}

// TODO
unsafe impl Sync for Driver {}
unsafe impl Send for Driver {}

impl Producer for Driver {
    type Msg = DriverMsg;

    fn next(&self) -> Self::Msg {
        DriverMsg {
            left_stick_y: self.controller.get_y(JoystickHand::kLeftHand),
            right_stick_y: self.controller.get_y(JoystickHand::kRightHand),
        }
    }
}
