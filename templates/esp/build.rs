//! Build script: bake the compressed `vial.json` blob into the firmware and set
//! the esp linker script. ESP chips use `esp-bootloader-esp-idf`, so there is no
//! `memory.x` — only `linkall.x`.

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{env, fs};

use const_gen::*;
use xz2::read::XzEncoder;

fn main() {
    println!("cargo:rerun-if-changed=keyboard.toml");
    println!("cargo:rerun-if-changed=vial.json");
    generate_vial_config();

    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
}

/// Compress `vial.json` (xz) and emit `config_generated.rs` with the
/// `VIAL_KEYBOARD_DEF` / `VIAL_KEYBOARD_ID` consts the firmware reads.
fn generate_vial_config() {
    let out_file = Path::new(&env::var_os("OUT_DIR").unwrap()).join("config_generated.rs");

    let p = Path::new("vial.json");
    let mut content = String::new();
    match File::open(p) {
        Ok(mut file) => {
            file.read_to_string(&mut content).expect("Cannot read vial.json");
        }
        Err(e) => println!("Cannot find vial.json {:?}: {}", p, e),
    };

    let vial_cfg = json::stringify(json::parse(&content).unwrap());
    let mut keyboard_def_compressed: Vec<u8> = Vec::new();
    XzEncoder::new(vial_cfg.as_bytes(), 6)
        .read_to_end(&mut keyboard_def_compressed)
        .unwrap();

    let keyboard_id: Vec<u8> = vec![0xB9, 0xBC, 0x09, 0xB2, 0x9D, 0x37, 0x4C, 0xEA];
    let const_declarations = [
        const_declaration!(pub VIAL_KEYBOARD_DEF = keyboard_def_compressed),
        const_declaration!(pub VIAL_KEYBOARD_ID = keyboard_id),
    ]
    .map(|s| "#[allow(clippy::redundant_static_lifetimes)]\n".to_owned() + s.as_str())
    .join("\n");
    fs::write(out_file, const_declarations).unwrap();
}
