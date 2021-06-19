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
6. Code editor
   - Recommended: VSCode + rust-analyzer extention
      - https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer
   - Alternative: IntelliJ Ultimate/CLion + rust extension

# Project structure

```
├── frc             - package containing friendly wrappers around frc-sys
├── frc-sys         - package containing generated rust bindings to wpilib/vendor libraries
│   ├── include     - wpilib/vendor headers installed with install_deps.py
│   ├── lib
│   │   ├── linux   - wpilib/vendor libraries installed with install_deps.py
│   │   └── roborio - roborio shared libraries downloaded from roborio
│   ├── src         - wpilib/vendor autogenerated bindings
│   ├── vendordeps  - wpilib/vendor dependency JSON files
│   └── wrappers    - wpilib/vendor C headers to generate rust bindings from
└── toolchain
```