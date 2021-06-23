#[macro_use]
extern crate lazy_static;

use bindgen::Builder;
use bindgen::EnumVariation;
use cc::Build;
use chrono::Local;
use std::env;
use std::path::PathBuf;

const STATIC_LIBS: &[&str] = &[
    "ColorSensorV3",
    "CTRE_Phoenix_WPI",
    "CTRE_Phoenix",
    "CTRE_PhoenixCCI",
    "CTRE_PhoenixCore",
    "CTRE_PhoenixDiagnostics",
    "navx_frc",
    "SparkMax",
    "SparkMaxDriver",
];

const SHARED_LIBS: &[&str] = &[
    "wpilibc",
    "ntcore",
    "wpiHal",
    "wpiutil",
    "wpimath",
    "RoboRIO_FRC_ChipObject",
    "FRC_NetworkCommunication",
    "visa",
    "nirio_emb_can",
    "ni_emb",
    "ni_rtlog",
    "NiRioSrv",
    "NiFpga",
    "niriodevenum",
    "NiFpgaLv",
    "niriosession",
    "cameraserver",
];

// Path to libraries
const LIB_DIR: &str = "lib/linux/athena";

// Path to roborio toolchain
const TOOLCHAIN: &str = "../toolchain/roborio";

// Roborio gcc path (relative to TOOLCHAIN)
const GCC_PATH: &str = "bin/arm-frc2021-linux-gnueabi-gcc";

// Common flags passed to bindgen and cc
const GCC_FLAGS: &[&str] = &[
    "-std=c++17", // Use c++17 features (which wpilib does)
    "-nostdinc",  // ignore standard include paths
    "-nostdinc++",
];

// Toolchain include paths (paths relative to TOOLCHAIN)
const GCC_INCLUDE: &[&str] = &[
    "arm-frc2021-linux-gnueabi/usr/lib/gcc/arm-frc2021-linux-gnueabi/7.3.0/include",
    "arm-frc2021-linux-gnueabi/usr/lib/gcc/arm-frc2021-linux-gnueabi/7.3.0/include-fixed",
    "arm-frc2021-linux-gnueabi/usr/include/c++/7.3.0",
    "arm-frc2021-linux-gnueabi/usr/include/c++/7.3.0/arm-frc2021-linux-gnueabi",
    "arm-frc2021-linux-gnueabi/usr/include/c++/7.3.0/backward",
    "arm-frc2021-linux-gnueabi/usr/include",
];

lazy_static! {
    static ref CWD: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
}

fn main() {
    // Compile wpilib_RobotBase extension
    compile_cc("wpilib_RobotBase");

    // Generate HAL bindings
    generate_bindings(
        "hal",
        Builder::default()
            // Allow only HAL types/functions/constants
            .allowlist_type("HAL_.*")
            .allowlist_function("HAL_.*")
            .allowlist_var("HAL_.*"),
    );

    // Generate CTRE bindings
    generate_bindings(
        "ctre",
        Builder::default()
            .allowlist_type("ctre::.*")
            .allowlist_function("ctre::.*")
            .allowlist_var("ctre::.*"),
    );

    // Generate wpilib bindings
    generate_bindings(
        "wpilib",
        Builder::default()
            // Allow types/functions/constants in frc namespace
            .allowlist_type("frc::.*")
            .allowlist_function("frc::.*")
            .allowlist_var("frc::.*")
            // Block networktables stuff
            .blocklist_item("ITable.*")
            .blocklist_item("nt::.*")
            .blocklist_item("NT_.*")
            // Hide some wpi stuff
            .opaque_type("wpi::.*")
            .blocklist_function("wpi::.*"),
    );

    // Link with static libraries
    println!(
        "cargo:rustc-link-search={}",
        CWD.join(LIB_DIR).join("static").to_str().unwrap(),
    );
    for lib in STATIC_LIBS {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // Link with shared libraries
    println!(
        "cargo:rustc-link-search={}",
        CWD.join(LIB_DIR).join("shared").to_str().unwrap(),
    );
    for lib in SHARED_LIBS {
        println!("cargo:rustc-link-lib={}", lib)
    }

    // Link with stdc++
    println!("cargo:rustc-link-lib=stdc++");
}

fn compile_cc(name: &str) {
    // C++ source file path
    let source = PathBuf::from("wrappers").join(format!("{}.cc", name));

    // Rerun build if source changes
    println!("cargo:rerun-if-changed={}", source.to_str().unwrap());

    let mut build = Build::new();
    build
        .file(source)
        .compiler(CWD.join(TOOLCHAIN).join(GCC_PATH)) // Use roborio gcc
        .cpp(true) // Enable C++ mode
        .include("include");

    for flag in GCC_FLAGS {
        build.flag(flag);
    }

    for include in GCC_INCLUDE {
        build.include(CWD.join(TOOLCHAIN).join(include));
    }

    build.compile(&name);
}

fn generate_bindings(module: &str, builder: Builder) {
    // Header file path
    let header = PathBuf::from("wrappers").join(format!("{}.h", module));
    // Generated rust file path
    let rs = PathBuf::from("src").join(format!("{}.rs", module));

    // Rerun build if wrapper changes
    println!("cargo:rerun-if-changed={}", header.to_str().unwrap());

    let mut builder = builder
        .header(header.to_str().unwrap())
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .layout_tests(false) // Don't generate memory layout tests
        .opaque_type("std::.*") // Treat std types as opaque
        .clang_arg("-xc++") // Enable C++ header mode
        .clang_arg("-Iinclude");

    for flag in GCC_FLAGS {
        builder = builder.clang_arg(*flag);
    }

    for include in GCC_INCLUDE {
        builder = builder.clang_arg(format!(
            "-I{}",
            CWD.join(TOOLCHAIN).join(include).to_str().unwrap()
        ));
    }

    // Add comments on top of generated file with date and compiler flag info
    let comments = [
        format!("Generated on {:?}", Local::now()),
        builder.command_line_flags().join(" "),
    ];

    builder
        .raw_line(
            comments
                .iter()
                .map(|c| format!("// {}", c))
                .fold(String::new(), |a, b| a + &b + "\n"),
        )
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(rs)
        .expect("Couldn't write bindings");
}
