if variable::get("keyboard_type") == "normal" {
    file::rename("src/main_config.rs", "src/main.rs");
} else if variable::get("keyboard_type") == "split" {
    file::rename("Makefile_split.toml", "Makefile.toml");
}
if variable::get("microcontroller_family") == "stm32" {
    file::rename("Cargo_stm32.toml", "Cargo.toml");
}
else if variable::get("microcontroller_family") == "rp2040" {
    file::rename("Cargo_rp.toml", "Cargo.toml");
}
else if variable::get("microcontroller_family") == "esp" {
    file::rename("Cargo_esp.toml", "Cargo.toml");
}
else if variable::get("microcontroller_family") == "nrf" && variable::get("connection") == "BLE" {
    file::rename("Cargo_nrf_ble.toml", "Cargo.toml");
}
else if variable::get("microcontroller_family") == "nrf" {
    file::rename("Cargo_nrf.toml", "Cargo.toml");
}

file::rename("README_gen.md", "README.md");

print("️️👉👉👉 For the following steps, search 'TODO' in generated project");
