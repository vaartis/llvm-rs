use std::process::Command;

fn main() {
    if let Ok(x) = Command::new("llvm-config").arg("--libdir").output() {
        println!("cargo:rustc-link-search=native={}", String::from_utf8(x.stdout).unwrap().trim());
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
}
