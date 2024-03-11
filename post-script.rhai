if variable::get("microcontroller_family") == "stm32" {
    file::rename("src/main_stm32.rs", "src/main.rs");
}
else if variable::get("microcontroller_family") == "rp2040" {
    file::rename("src/main_rp.rs", "src/main.rs");
}
else if variable::get("microcontroller_family") == "nrf" && variable::get("connection") == "BLE" {
    file::rename("src/main_nrf_ble.rs", "src/main.rs");
}
else if variable::get("microcontroller_family") == "nrf" {
    file::rename("src/main_nrf.rs", "src/main.rs");
}

if variable::get("microcontroller_family") == "stm32" {
    file::rename("Cargo_stm32.toml", "Cargo.toml");
}
else if variable::get("microcontroller_family") == "rp2040" {
    file::rename("Cargo_rp.toml", "Cargo.toml");
}
else if variable::get("microcontroller_family") == "nrf" && variable::get("connection") == "BLE" {
    file::rename("Cargo_nrf_ble.toml", "Cargo.toml");
}
else if variable::get("microcontroller_family") == "nrf" {
    file::rename("Cargo_nrf.toml", "Cargo.toml");
}

file::rename("README_gen.md", "README.md");

print("️️👉👉👉 For the following steps, search 'TODO' in generated project");
