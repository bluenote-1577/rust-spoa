// build.rs
extern crate cc;
extern crate cmake;
use cmake::Config;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    
    // Build spoa with explicit output directory to avoid system installation
    let spoa_dst = Config::new("src/spoa")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_PREFIX", &out_dir)
        .out_dir(&out_dir)
        .build();
    
    cc::Build::new()
        .cpp(true)
        .shared_flag(false)
        .static_flag(true)
        .flag_if_supported("-O3")
        .flag_if_supported("-D_GNU_SOURCE")
        .flag_if_supported("-Wall")
        .flag_if_supported("-std=c++11")
        .include("src/spoa/include")  // Use .include() instead of -I flag
        .file("src/poa_func.cpp")
        .compile("poa_func");
    
    // Explicitly specify the library paths - use separate println! statements
    println!("cargo:rustc-link-search=native={}/lib", spoa_dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", spoa_dst.display());
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
