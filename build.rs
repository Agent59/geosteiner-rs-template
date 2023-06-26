/// build.rs Example
///
/// BUILD TIMES CAN BE VERY LONG! Don't be suprised.
///
/// This is needed to compile the geosteiner C library
/// and place it into the correct path.
/// Adapt GEOSTEINER_C_PATH and TARGET for your usage.

#[cfg(feature = "rebuild_c")]
use std::process::Command;

// The path of the Geosteiner C library
#[cfg(feature = "rebuild_c")]
const GEOSTEINER_C_PATH: &str = "./src/geosteiner/";

/// Cleans up all compiled C files in the `GEOSTEINER_C_PATH`.
/// And then runs make to compile the C libraries.
#[cfg(feature = "rebuild_c")]
fn rebuild_c_libs() {
    Command::new("make")
        .arg("clean")
        .arg("-C")
        .arg(format!("{}", GEOSTEINER_C_PATH))        
        .output()
        .expect("make clean in path {GEOSTEINER_C_PATH} should have succeded");
    
    Command::new("make")
        .arg("-C")
        .arg(format!("{}", GEOSTEINER_C_PATH))
        .arg("librs_geosteiner.la")
        .output()
        .expect("make command in path {GEOSTEINER_C_PATH} should have succeeded"); 
}

#[cfg(not(feature = "rebuild_c"))]
fn rebuild_c_libs() {}


fn main() {
    rebuild_c_libs();

    // Links the C library to Rust
    println!("cargo:rustc-link-search=dependency=src/geosteiner");
    println!("cargo:rustc-link-lib=dylib=rs_geosteiner");
}
