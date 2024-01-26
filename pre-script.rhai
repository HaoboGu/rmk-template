let family = variable::get("mcu");

if family == "rp" {
    variable::set("target", "thumbv6m-none-eabi")
}