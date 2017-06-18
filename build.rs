use std::process::Command;

fn main() {
    let ll_path = match Command::new("llvm-config").arg("--libdir").output() {
        Ok(x) => String::from_utf8(x.stdout).unwrap(),
        Err(_) => panic!("llvm-config not found")
    };

    println!("cargo:rustc-link-search=native={}", ll_path);
}
