typedef void (*rust_fn)();

namespace frc {

int StartRobotRs(rust_fn start_competition,
                 rust_fn end_competition);

}