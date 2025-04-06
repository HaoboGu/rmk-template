# RMK 

RMK is a feature-rich and easy-to-use keyboard firmware.

## Use the template

1. Install [probe-rs](https://github.com/probe-rs/probe-rs)

   ```shell
   # Linux/macOS
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

   # Windows
   irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
   ```

2. Build the firmware

   ```shell
   cargo build --release
   ```

3. Flash using debug probe

   If you have a debug probe connected to your  board, flashing is quite simple: run the following command to automatically compile and flash RMK firmware to the board:

   ```shell
   cargo run --release
   ```
