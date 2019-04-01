use cmake::Config;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    if !Path::new("libvgm/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bld = dst.join("build");

    fs::create_dir_all(&bld).unwrap();

    let mut include_root = "-I".to_owned();
    include_root.push_str(&String::from(
        env::current_dir()
        .unwrap()
        .join("libvgm")
        .into_os_string()
        .into_string()
        .unwrap()));

    let dst = Config::new(Path::new("libvgm/emu"))
        .cflag(include_root)
        .build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=vgm-emu");
}
