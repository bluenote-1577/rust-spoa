// build.rs
extern crate cc;
extern crate cmake;
use cmake::Config;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Build spoa with explicit output directory and disable installation
    let spoa_dst = Config::new("src/spoa")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_PREFIX", &out_dir)
        .define("BUILD_SHARED_LIBS", "OFF")  // Force static libraries
        .build_target("spoa")  // Only build the spoa target, don't install
        .build();
    
    // Find the actual library location
    let lib_path = spoa_dst.join("build").join("lib");
    let lib64_path = spoa_dst.join("build").join("lib64");
    
    cc::Build::new()
        .cpp(true)
        .shared_flag(false)
        .static_flag(true)
        .flag_if_supported("-O3")
        .flag_if_supported("-D_GNU_SOURCE")
        .flag_if_supported("-Wall")
        .flag_if_supported("-std=c++11")
        .include("src/spoa/include")
        .file("src/poa_func.cpp")
        .compile("poa_func");
    
    // Library search paths - check both potential locations
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-search=native={}", lib64_path.display());
    println!("cargo:rustc-link-search=native={}/lib", &out_dir);
    println!("cargo:rustc-link-search=native={}/lib64", &out_dir);
    
    // Specify the libraries to link
    println!("cargo:rustc-link-lib=static=poa_func");
    println!("cargo:rustc-link-lib=static=spoa");
    
    // Use the correct C++ standard library based on target platform
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }
}
