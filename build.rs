use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=boot/boot.S");

    let output = Command::new("as")
        .args(&["-32", "src/boot/boot.S", "-o", "src/boot/boot.o"])
        .status()
        .expect("Failed to assemble boot.S");

    assert!(output.success());
    println!("cargo:rustc-link-search=src/boot");
    println!("cargo:rustc-link-arg=src/boot/boot.o");
    println!("cargo:rustc-link-arg=-Tsrc/boot/boot.ld");
}
