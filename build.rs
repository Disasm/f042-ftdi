use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::copy("memory.x", out_dir.join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=memory.x");

    let target = env::var("TARGET").unwrap();
    let name = env::var("CARGO_PKG_NAME").unwrap();
    fs::copy(format!("bin/{}.a", target), out_dir.join(format!("lib{}.a", name))).unwrap();
    println!("cargo:rustc-link-lib=static={}", name);
    println!("cargo:rerun-if-changed=bin/{}.a", target);
}
