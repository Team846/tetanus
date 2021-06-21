// #include <frc/Compressor.h>
#include <frc/DriverStation.h>
// #include <frc/Joystick.h>
// #include <frc/Solenoid.h>
#include <frc/RobotBase.h>
// #include <frc/XboxController.h>



// wpilib_RobotBase.cc

typedef void (*rust_fn)();

namespace frc {

int StartRobotRs(rust_fn start_competition,
                 rust_fn end_competition);

}