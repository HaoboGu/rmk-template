let family = variable::get("microcontroller_family");
if family == "rp2040" {
    variable::set("target", "thumbv6m-none-eabi");
    variable::set("chip", "rp2040");
}

let chip = variable::get("chip");
let conn_type = variable::get("connection");
if chip == "nrf52840" || chip == "nrf52833" || chip == "nrf52832" || chip == "nrf52840" {
    variable::set("target", "thumbv7em-none-eabihf");
    variable::set("nrf_feature", chip);
    variable::set("chip", chip + "_xxAA");
}
else if chip == "nrf52805" || chip == "nrf52810" || chip == "nrf52811" || chip == "nrf52820" {
    variable::set("target", "thumbv7em-none-eabi");
    variable::set("nrf_feature", chip);
    variable::set("chip", chip + "_xxAA");
}
else if chip == "esp32c3" {
    variable::set("target", "riscv32imc-esp-espidf");
}
else if chip == "esp32s3" {
    variable::set("target", "xtensa-esp32s3-espidf");
}
