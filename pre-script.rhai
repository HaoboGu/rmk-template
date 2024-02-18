let family = variable::get("mcu");

if family == "rp2040" {
    variable::set("target", "thumbv6m-none-eabi");
}

let chip = variable::get("chip");

if chip == "nrf52840" || chip == "nrf52833" || chip == "nrf52832" {
    variable::set("target", "thumbv7em-none-eabihf");
}
else if chip == "nrf52805" || chip == "nrf52810" || chip == "nrf52811" || chip == "nrf52820" {
    variable::set("target", "thumbv7em-none-eabi");
}