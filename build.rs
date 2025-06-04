use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=boot/boot.S");

    let output = Command::new("as")
        .args(&["-32", "boot/boot.S", "-o", "boot/boot.o"])
        .status()
        .expect("Failed to assemble boot.S");

    assert!(output.success());
    println!("cargo:rustc-link-search=boot");
    println!("cargo:rustc-link-arg=boot/boot.o");
}
