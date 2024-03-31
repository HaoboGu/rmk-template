let family = variable::get("microcontroller_family");
if family == "rp2040" {
    variable::set("target", "thumbv6m-none-eabi");
}

let conn_type = variable::get("connection");

let chip = if conn_type == "BLE" {
    variable::set("connection", "BLE");
    let chip = variable::get("chip");
    variable::set("chip", chip + "_xxAA");
    variable::set("nrf_feature", chip);
    chip
} else {
    variable::set("connection", "USB");
    variable::get("chip")
};

if chip == "nrf52840" || chip == "nrf52833" || chip == "nrf52832" || chip == "nrf52840" {
    variable::set("target", "thumbv7em-none-eabihf");
}
else if chip == "nrf52805" || chip == "nrf52810" || chip == "nrf52811" || chip == "nrf52820" {
    variable::set("target", "thumbv7em-none-eabi");
}
