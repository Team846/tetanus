# Installation

1. Clone repository
2. Install rust
   - With rustup (https://www.rust-lang.org/tools/install)
   - Install the rust ARM toolchain
      - `$ rustup target add arm-unknown-linux-gnueabi`
3. Install RoboRIO toolchain
   - From https://github.com/wpilibsuite/roborio-toolchain/releases
   - Unzip and copy the `roborio/` folder to `./toolchain/` (copy the roborio folder itself, not the contents inside)
4. Install clang (required by bindgen)
   - Follow https://rust-lang.github.io/rust-bindgen/requirements.html
      - For Mac, install homebrew if you haven't already (https://brew.sh/)
5. Install wpilib/vendor dependencies
   - Run the `install_deps.py` script
      - `$ cd frc-sys && ./install_deps.py`
6. Compile 2021 swerve robot test package
   - `$ cargo build -p r2021-swerve --release`
6. Code editor
   - Recommended: VSCode + rust-analyzer extention
      - https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
   - Alternative: IntelliJ Ultimate/CLion + rust extension

# Project structure

```
├── frc             - package containing friendly wrappers around frc-sys
├── frc-sys         - package containing generated rust bindings to frc libraries
│   ├── include     - frc headers installed with install_deps.py
│   ├── lib         - frc libraries installed with install_deps.py
│   ├── src         - frc autogenerated bindings
│   ├── vendordeps  - frc dependency JSON files
│   └── wrappers    - frc C headers to generate rust bindings from
├── r2021-swerve    - 2021 swerve robot (mainly for testing)
└── toolchain       - roborio toolchain
```

# Remote debugging

Reference: https://github.com/frc1678/robot-code-public/blob/master/docs/gdb.md

1. Debug server on robot
   - ssh into robot
   - Kill autostarting code
      - `$ . /etc/profile.d/natinst-path.sh; /usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null`
   - Run gdbserver
      - `$ gdbserver localhost:2345 ./frcUserProgram` 
2. Run gdb on computer
   - Install gdb (Mac)
      - `toolchain/roborio/bin/arm-frc2021-linux-gnueabi-gdb` works but crashes randomly a lot
      - Install with homebrew
         - Use the solution here https://stackoverflow.com/questions/54541783/installing-gdb-with-all-targets-on-mac-with-homebrew to install ARM target as well
   - Run gdb
   - `$ gdb target/arm-unknown-linux-gnueabi/debug/rswerve`
      - Setup remote
         - `target remote roborio-846-frc.local:2345`
         - `set sysroot remote:/`
      - Start robot code
         - `continue`