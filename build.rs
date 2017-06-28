extern crate bindgen;

use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    if let Ok(x) = Command::new("llvm-config").arg("--libdir").output() {
        let d = String::from_utf8(x.stdout).unwrap();
        env::set_var("LIBCLANG_PATH", d.trim());
        println!("cargo:rustc-link-search=native={}", d.trim());
    } else {
        panic!("llvm-config not found")
    }

    if let Ok(x) = Command::new("llvm-config").arg("--libs").output() {
        for n in String::from_utf8(x.stdout)
            .unwrap()
            .replace("-l", "")
            .trim()
            .split(" ") {
                println!("cargo:rustc-link-lib={}", n);
            }
    } else {
        panic!("llvm-config --libs failed")
    }

    let llvm_incldir = String::from_utf8(
        Command::new("llvm-config").arg("--includedir").output().unwrap().stdout
    ).unwrap();

    let bindings = bindgen::Builder::default()
        .header("src/bindings/wrapper.h")
        .clang_arg(format!("-I{}", llvm_incldir.trim()))
        .whitelisted_type("LLVM.+")
        .whitelisted_function("LLVM.+")
        .raw_line("#![allow(dead_code,non_snake_case,non_camel_case_types)]")
        .generate()
        .unwrap();
    let out_path = PathBuf::from("src/bindings");
    bindings
        .write_to_file(out_path.join("mod.rs"))
        .expect("Couldn't write bindings!");
}
