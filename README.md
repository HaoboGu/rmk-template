# rmk-template

Template for initializing a [RMK](https://github.com/HaoboGu/rmk) keyboard firmware project.

## Usage

1. Install `cargo generate`

    ```shell
    cargo install cargo-generate
    ```

2. Create your project using the template
    ```shell
    cargo generate --git https://github.com/HaoboGu/rmk-template
    ```

3. Follow the steps in generated `README.md` to finish the initialization

4. (optional) Install [`probe-rs`](https://github.com/probe-rs/probe-rs) for flashing and debugging

   ```shell
   cargo install probe-rs --features cli
   ```

5. (optional) Build and flash

    ```shell
    cargo build --release
    cargo run
    ```    
