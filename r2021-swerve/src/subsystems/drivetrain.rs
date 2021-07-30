use frc::ctre::motorcontrol::{can::BaseMotorController, FollowerType, TalonFXInvertType};
use tetanus_core::consumer::Consumer;
use tetanus_frc::esc::{self, OffloadedEsc, OffloadedEscConfig};
use uom::si::f64::*;
use uom::si::ratio::ratio;

#[derive(Clone, Copy, Debug)]
pub struct DrivetrainMsg {
    pub left: Ratio,
    pub right: Ratio,
}

#[allow(dead_code)]
pub struct Drivetrain {
    left_master_esc: OffloadedEsc,
    left_slave_esc: OffloadedEsc,
    right_master_esc: OffloadedEsc,
    right_slave_esc: OffloadedEsc,
}

impl Drivetrain {
    const LEFT_MASTER_ID: i32 = 30;
    const LEFT_SLAVE_ID: i32 = 32;
    const RIGHT_MASTER_ID: i32 = 33;
    const RIGHT_SLAVE_ID: i32 = 31;

    const LEFT_INVERSION: TalonFXInvertType = TalonFXInvertType::CounterClockwise;
    const LEFT_SENSOR_INVERSION: bool = true;

    const RIGHT_INVERSION: TalonFXInvertType = TalonFXInvertType::Clockwise;
    const RIGHT_SENSOR_INVERSION: bool = false;

    const SPEED_FACTOR: f64 = 0.2;

    pub fn new() -> Self {
        let mut left_master_esc = OffloadedEsc::talon_fx(Self::LEFT_MASTER_ID);
        let mut left_slave_esc = OffloadedEsc::talon_fx(Self::LEFT_SLAVE_ID);
        let mut right_master_esc = OffloadedEsc::talon_fx(Self::RIGHT_MASTER_ID);
        let mut right_slave_esc = OffloadedEsc::talon_fx(Self::RIGHT_SLAVE_ID);

        left_master_esc
            .setup(OffloadedEscConfig::default())
            .unwrap();
        left_master_esc.config_talon_fx(|t| {
            t.set_selected_sensor_position(0.0, 0, esc::CONFIG_TIMEOUT_MS)
                .unwrap();
            t.set_inverted(Self::LEFT_INVERSION);
            t.set_sensor_phase(Self::LEFT_SENSOR_INVERSION);
        });

        left_slave_esc.setup(OffloadedEscConfig::default()).unwrap();
        left_slave_esc.config_talon_fx(|t| {
            t.follow(
                left_master_esc.as_talon_fx(),
                FollowerType::FollowerType_PercentOutput,
            );
            t.set_inverted(Self::LEFT_INVERSION);
        });

        right_master_esc
            .setup(OffloadedEscConfig::default())
            .unwrap();
        right_master_esc.config_talon_fx(|t| {
            t.set_selected_sensor_position(0.0, 0, esc::CONFIG_TIMEOUT_MS)
                .unwrap();
            t.set_inverted(Self::RIGHT_INVERSION);
            t.set_sensor_phase(Self::RIGHT_SENSOR_INVERSION);
        });

        right_slave_esc
            .setup(OffloadedEscConfig::default())
            .unwrap();
        right_slave_esc.config_talon_fx(|t| {
            t.follow(
                right_master_esc.as_talon_fx(),
                FollowerType::FollowerType_PercentOutput,
            );
            t.set_inverted(Self::RIGHT_INVERSION);
        });

        Drivetrain {
            left_master_esc,
            left_slave_esc,
            right_master_esc,
            right_slave_esc,
        }
    }
}

impl Consumer for Drivetrain {
    type Msg = DrivetrainMsg;

    fn output(&mut self, msg: Self::Msg) {
        self.left_master_esc
            .output_percent(msg.left.get::<ratio>() * Self::SPEED_FACTOR);
        self.right_master_esc
            .output_percent(msg.right.get::<ratio>() * Self::SPEED_FACTOR);
    }
}
