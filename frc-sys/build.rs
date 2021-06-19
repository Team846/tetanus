use bindgen::Builder;
use std::env;
use std::path::Path;
use chrono::Local;

// frc libraries to link with
const FRC_LIBS: &[&str] = &["wpiHal", "wpiutil", "wpilibc"];

// roborio libraries to link with
const ROBORIO_LIBS: &[&str] = &[
    "FRC_NetworkCommunication",
    "ni_emb",
    "ni_rtlog",
    "NiFpga",
    "NiFpgaLv",
    "nirio_emb_can",
    "niriodevenum",
    "niriosession",
    "NiRioSrv",
    "RoboRIO_FRC_ChipObject",
    "visa",
];

fn main() {
    // Generate HAL bindings
    let hal_regex = "HAL_.*";
    generate_bindings(
        "hal",
        Builder::default()
            .allowlist_type(hal_regex)
            .allowlist_function(hal_regex)
            .allowlist_var(hal_regex),
    );

    // Generate wpilib bindings
    let wpilib_regex = "frc::.*";
    generate_bindings(
        "wpilib",
        Builder::default()
            .opaque_type("wpi::.*")
            .opaque_type("frc::ErrorBase")
            .allowlist_type(wpilib_regex)
            .allowlist_function(wpilib_regex)
            .allowlist_var(wpilib_regex),
    );

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Link with wpilib + frc vendors
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir)
            .join("lib")
            .join("linux")
            .join("athena")
            .join("static")
            .display()
    );
    for lib in FRC_LIBS.iter() {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // Link with roborio libraries
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("lib").join("roborio").display()
    );
    for lib in ROBORIO_LIBS.iter() {
        println!("cargo:rustc-link-lib={}", lib)
    }

    // Link with stdc++
    println!("cargo:rustc-link-lib=stdc++");
}

fn generate_bindings(module: &str, builder: Builder) {
    let header = format!("wrappers/{}.h", module);
    let rs = format!("{}.rs", module);

    // Rerun build if wrapper changes
    println!("cargo:rerun-if-changed={}", header);

    let builder = builder
        .header(header)
        .layout_tests(false)
        .opaque_type("std::.*")
        .clang_arg("-xc++") // enable C++ header mode
        .clang_arg("-std=c++17") // c++17 features
        .clang_arg("-Iinclude") // use frc include directory
        // Use standard library headers from the wpilib toolchain instead of standard include directories
        .clang_arg("-nostdinc")
        .clang_arg("-nostdinc++")
        .clang_arg("-I../toolchain/roborio/arm-frc2021-linux-gnueabi/usr/lib/gcc/arm-frc2021-linux-gnueabi/7.3.0/include-fixed")
        .clang_arg("-I../toolchain/roborio/arm-frc2021-linux-gnueabi/usr/lib/gcc/arm-frc2021-linux-gnueabi/7.3.0/include")
        .clang_arg("-I../toolchain/roborio/arm-frc2021-linux-gnueabi/usr/include/c++/7.3.0/arm-frc2021-linux-gnueabi")
        .clang_arg("-I../toolchain/roborio/arm-frc2021-linux-gnueabi/usr/include/c++/7.3.0")
        .clang_arg("-I../toolchain/roborio/arm-frc2021-linux-gnueabi/usr/include");

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
        .write_to_file(Path::new("src").join(rs))
        .expect("Couldn't write bindings");
}
