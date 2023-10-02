use std::process::Command;

fn main() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap().into_string().unwrap();
    Command::new("tailwindcss")
        .args(["--output", &format!("{out_dir}/style.css")])
        .output()
        .unwrap();

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=tailwind.config.js");
}
