/*
 * Copy of RunRobot and StartRobot in frc/RobotBase.h with changes to make rust bindings friendlier.
 * 
 * Instead of wpilib's c++ way of creating a subclass of RobotBase and passing it as a template to
 * RunRobot and StartRobot, StartRobotRs takes in start_competition and end_competition as rust
 * function pointers. The rust functions are used to create RobotBaseRs.
 */

#include "wpilib.h"

#include <frc/RobotBase.h>

namespace frc {

class RobotBaseRs : public frc::RobotBase {
 public:
  RobotBaseRs(rust_fn start_competition, rust_fn end_competition)
      : start_competition(start_competition),
        end_competition(end_competition) {}

  void StartCompetition() override { start_competition(); }

  void EndCompetition() override { end_competition(); }

 private:
  rust_fn start_competition, end_competition;
};

namespace impl {

void RunRobotRs(wpi::mutex& m, RobotBaseRs** robot,
                rust_fn start_competition,
                rust_fn end_competition) {
  static RobotBaseRs theRobot(start_competition, end_competition);
  {
    std::scoped_lock lock{m};
    *robot = &theRobot;
  }
  theRobot.StartCompetition();
}

}  // namespace impl

int StartRobotRs(rust_fn start_competition,
                 rust_fn end_competition) {
  int halInit = RunHALInitialization();
  if (halInit != 0) {
    return halInit;
  }

  static wpi::mutex m;
  static wpi::condition_variable cv;
  static RobotBaseRs* robot = nullptr;
  static bool exited = false;

  if (HAL_HasMain()) {
    std::thread thr([=] {
      try {
        impl::RunRobotRs(m, &robot, start_competition, end_competition);
      } catch (...) {
        HAL_ExitMain();
        {
          std::scoped_lock lock{m};
          robot = nullptr;
          exited = true;
        }
        cv.notify_all();
        throw;
      }

      HAL_ExitMain();
      {
        std::scoped_lock lock{m};
        robot = nullptr;
        exited = true;
      }
      cv.notify_all();
    });

    HAL_RunMain();

    // signal loop to exit
    if (robot) {
      robot->EndCompetition();
    }

    // prefer to join, but detach to exit if it doesn't exit in a timely manner
    using namespace std::chrono_literals;
    std::unique_lock lock{m};
    if (cv.wait_for(lock, 1s, [] { return exited; })) {
      thr.join();
    } else {
      thr.detach();
    }
  } else {
    impl::RunRobotRs(m, &robot, start_competition, end_competition);
  }

  HAL_Shutdown();

  return 0;
}

}  // namespace frc