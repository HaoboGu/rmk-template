# rmk-template

Collections of project templates for initializing a [RMK](https://github.com/HaoboGu/rmk) keyboard firmware project.

## Usage

1. Install `rmkit` and `flip-link`

    ```shell
    cargo install rmkit flip-link
    ```

2. Create your project using the template
    ```shell
    rmkit init
    ```

3. Follow the steps in generated `README.md` to finish the initialization

4. (optional) Install [`probe-rs`](https://github.com/probe-rs/probe-rs) for flashing and debugging

   ```shell
   # https://probe.rs/docs/getting-started/installation/
   # Linux or macOS
   curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
   # Windows
   irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
   ```

5. (optional) Build and flash

    ```shell
    cargo build --release
    cargo run
    ```    
