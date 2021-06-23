use fmt::format;
use frc::motor_control::pwm_motor_controller;
use wpi::sendable::sendable_builder;
use wpi::sendable::sendable_registry;

use frc;

unsafe fn pwm_motor_controller::set(speed: f64) {
  m_pwm.SetSpeed(m_isInverted ? -speed : speed);
}