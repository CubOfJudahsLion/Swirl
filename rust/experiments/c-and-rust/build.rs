use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    Command::new("clang")
        .args([ "-c", "src/tinylib.c"
              , "--target=wasm32-unknown-unknown"
              , "-ffunction-sections", "-fdata-sections"
              , "-fPIC"
              , "-o", &format!("{}/tinylib.o", out_dir)
              ])
        .args([ "-O0", "-g"])
        .status()
        .unwrap();
    Command::new("llvm-ar")
        .args(&["rscu", "libtiny.a", "tinylib.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=tiny");
    println!("cargo:rerun-if-changed=src/tinylib.c");
}
