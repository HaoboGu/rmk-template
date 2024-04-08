let family = variable::get("microcontroller_family");
if family == "rp2040" {
    variable::set("target", "thumbv6m-none-eabi");
}

let chip = variable::get("chip");
let conn_type = variable::get("connection");
if chip == "nrf52840" || chip == "nrf52833" || chip == "nrf52832" || chip == "nrf52840" {
    variable::set("target", "thumbv7em-none-eabihf");
    if conn_type == "BLE" {
        variable::set("nrf_feature", chip);
        variable::set("chip", chip + "_xxAA");
    }
}
else if chip == "nrf52805" || chip == "nrf52810" || chip == "nrf52811" || chip == "nrf52820" {
    variable::set("target", "thumbv7em-none-eabi");
    if conn_type == "BLE" {
        variable::set("nrf_feature", chip);
        variable::set("chip", chip + "_xxAA");
    }
}
else if chip == "esp32c3" {
    variable::set("target", "riscv32imc-esp-espidf");
}
