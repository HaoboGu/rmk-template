if variable::get("microcontroller_family") == "stm32" {
    file::rename("src/main_stm32.rs", "src/main.rs");
}
else if variable::get("microcontroller_family") == "rp" {
    file::rename("src/main_rp.rs", "src/main.rs");
}

print("❗️ For the following steps, search 'TODO' in generated project");
