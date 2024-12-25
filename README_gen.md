# RMK

Keyboard firmware for cortex-m, with layer/dynamic keymap/[vial](https://get.vial.today) support, written in Rust.

## Checklist after generating the firmware project

- [ ] Update `memory.x` for your microcontroller(if needed)

- [ ] Update your `keyboard.toml`

- [ ] Create your `vial.json` by your layout: https://get.vial.today/docs/porting-to-via.html, replace the default one

- [ ] (Optional) Check the chip name of `probe-rs` is right, if you don't use `cargo run`, you can skip this step

- [ ] Update the family ID of your microcontroller in `Makefile.toml`, if you want to generate .uf2 firmware. The available family ID can be found in `https://git.sr.ht/~fenris/hex-to-uf2/tree/main/item/hex_to_uf2/src/families.rs#L7`

{% if chip == "nrf52840_xxAA" -%}
## For BLE only

To use NRF BLE, you should have [nrf s140 softdevice 7.3.0](https://www.nordicsemi.com/Products/Development-software/s140/download) flashed to nrf52840 first. 

The following are the detailed steps:

1. Erase the flash:
   ```shell
   probe-rs erase --chip nrf52840_xxAA
   ```
2. Flash softdevice firmware to flash:
   ```shell
   probe-rs download --verify --format hex --chip nRF52840_xxAA s140_nrf52_7.3.0_softdevice.hex
   ```
3. Compile, flash and run the example
   ```shell
   cargo run --release
   ```
4. (Optional) generate .uf2 firmware
   ```shell
   cargo make uf2 --release
   ```
{% elsif chip == "nrf52832_xxAA" -%}
## For BLE only

To use NRF BLE, you should have [nrf s132 softdevice 7.3.0](https://www.nordicsemi.com/Products/Development-software/s132/download) flashed to nrf52832 first. 

The following are the detailed steps:

1. Erase the flash:
   ```shell
   probe-rs erase --chip nrf52832_xxAA
   ```
2. Flash softdevice firmware to flash:
   ```shell
   probe-rs download --verify --format hex --chip nRF52832_xxAA s132_nrf52_7.3.0_softdevice.hex
   ```
3. Compile, flash and run the example
   ```shell
   cargo run --release
   ```
4. (Optional) generate .uf2 firmware
   ```shell
   cargo make uf2 --release
   ```
{% elsif chip == "esp32c3" -%}
## For BLE only

To use ESP BLE, you should have Rust **nightly** and `esp-idf` toolchain installed. The full instruction of installing `esp-idf` toolchain can be found [here](https://docs.esp-rs.org/book/installation/index.html) and [here](https://docs.esp-rs.org/std-training/02_2_software.html)

To run the example, make sure that you have esp-idf environment, `ldproxy` and `espflash` installed correctly. Then, run 

```
cd boards/esp32c3_ble
cargo run --release
```

If everything is good, you'll see the log as the following:

```shell
cargo run --release  
    Compiling ...
    ...
    ...
    Finished `release` profile [optimized + debuginfo] target(s) in 51.39s
     Running `espflash flash --monitor --log-format defmt target/riscv32imc-esp-espidf/release/rmk-esp32c3`
[2024-04-07T12:49:21Z INFO ] Detected 2 serial ports
[2024-04-07T12:49:21Z INFO ] Ports which match a known common dev board are highlighted
[2024-04-07T12:49:21Z INFO ] Please select a port
[2024-04-07T12:50:24Z INFO ] Serial port: '/dev/cu/xx'
[2024-04-07T12:50:24Z INFO ] Connecting...
[2024-04-07T12:50:24Z INFO ] Using flash stub
Chip type:         esp32c3 (revision v0.4)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi, BLE
MAC address:       aa:aa:aa:aa:aa:aa
App/part. size:    607,488/4,128,768 bytes, 14.71%
[2024-04-07T12:50:24Z INFO ] Segment at address '0x0' has not changed, skipping write
[2024-04-07T12:50:24Z INFO ] Segment at address '0x8000' has not changed, skipping write
[00:00:03] [========================================]     337/337     0x10000                                                                                                                    [2024-04-07T12:50:28Z INFO ] Flashing has completed!
```
{% elsif chip == "esp32s3" -%}
## For BLE only

To use ESP BLE, you should have [**Rust for Xtensa target**](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html) and `esp-idf` toolchain installed. 
The full instruction of installing `esp-idf` toolchain can be found [here](https://docs.esp-rs.org/book/installation/index.html) and [here](https://docs.esp-rs.org/std-training/02_2_software.html)

To run the example, make sure that you have esp-idf environment, `ldproxy` and `espflash` installed correctly. Then, run 

```
cd boards/esp32s3_ble
cargo run --release
```

{% endif -%}
