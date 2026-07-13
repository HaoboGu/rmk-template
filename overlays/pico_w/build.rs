//! pico_w build script (replaces the rp family build.rs): bakes the vial blob,
//! sets the RP2040 linker layout, and downloads the cyw43 Wi-Fi/BLE firmware
//! blobs unless the `skip-cyw43-firmware` feature is set.

use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

use const_gen::*;
use xz2::read::XzEncoder;

fn main() {
    println!("cargo:rerun-if-changed=vial.json");
    println!("cargo:rerun-if-changed=keyboard.toml");
    generate_vial_config();

    #[cfg(not(feature = "skip-cyw43-firmware"))]
    download_cyw43_firmware();

    // Put `memory.x` in OUT_DIR and on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    println!("cargo:rustc-linker=flip-link");
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

/// Download the cyw43 Wi-Fi/BLE firmware blobs into `cyw43-firmware/`.
#[cfg(not(feature = "skip-cyw43-firmware"))]
fn download_cyw43_firmware() {
    let download_folder = "cyw43-firmware";
    let url_base = "https://github.com/embassy-rs/embassy/raw/refs/heads/main/cyw43-firmware";
    let file_names = [
        "43439A0.bin",
        "43439A0_btfw.bin",
        "43439A0_clm.bin",
        "LICENSE-permissive-binary-license-1.0.txt",
        "README.md",
    ];

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", download_folder);
    std::fs::create_dir_all(download_folder).expect("Failed to create download directory");

    for file in file_names {
        let url = format!("{}/{}", url_base, file);
        if std::path::Path::new(download_folder).join(file).exists() {
            continue;
        }
        match reqwest::blocking::get(&url) {
            Ok(response) => {
                let content = response.bytes().expect("Failed to read file content");
                let file_path = PathBuf::from(download_folder).join(file);
                std::fs::write(file_path, &content).expect("Failed to write file");
            }
            Err(err) => panic!(
                "Failed to download the cyw43 firmware from {}: {}, required for pi-pico-w",
                url, err
            ),
        }
    }
}
