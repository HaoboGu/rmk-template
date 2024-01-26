# RMK

Keyboard firmware for cortex-m, with layer/dynamic keymap/[vial](https://get.vial.today) support, written in Rust.

## Checklist after generating the firmware project

- [ ] Update `memory.x` for your microcontroller(if needed)

- [ ] Create your `vial.json` by your layout: https://get.vial.today/docs/porting-to-via.html, replace the default one

- [ ] Update your default keymap at `src/keymap.rs`

- [ ] Update USB interrupt binding at `src/main.rs`(if needed)

- [ ] Check the chip name of `probe-rs` is right, if you don't use `cargo run`, you can skip this step
