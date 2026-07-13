# Template assembly contract

This repo is consumed by `rmkit create`. It is **data + parametric templates**,
not one full crate per chip. This document is the contract `rmkit` (and any other
assembler) implements.

## Layout

```
chip_matrix.toml        # one row per chip + stm32 core rules (the build matrix)
templates/<family>/     # a parametric crate per HAL family: rp, nrf, stm32, esp
overlays/<name>/        # opt-in modifier layers: split, pico_w
chips/<chip>/           # per-chip escape hatch for one-off quirks (e.g. esp32h2)
memory/<name>.x         # linker memory layouts referenced by chip_matrix `memory_x`
SPEC.md, README.md
# (legacy per-chip dirs are kept for one release while rmkit migrates)
```

## Assembly steps (`rmkit create`)

1. Resolve features: `rmk_config::firmware_features()` → chip id, series, board,
   `is_split`, and the `rmk` feature vector (+ `default-features`).
2. Read `chip_matrix.toml`; select `[chips.<id>]` (or, for `stm32*`, `[stm32]`
   plus the `core_rules` row matching the part's Cortex core).
3. Copy `templates/<family>/` into the project.
4. Apply layers **in this order, last wins** (plain file copy — a layer file
   replaces or adds the same path):
   a. `overlays/<overlay>/` if the row has `overlay = ...` (e.g. `pico_w`).
   b. `overlays/split/` if `is_split`.
   c. `chips/<chip>/` if the row has `overlay_chip = ...` (e.g. `esp32h2`).
   A layer may also carry a `Cargo.overlay.toml` fragment (see below).
5. Substitute placeholders (plain text replace) across `*.toml`, `*.json`,
   `*.rs`, `Makefile.toml`:
   | placeholder | source |
   |---|---|
   | `{{ project_name }}` | `[keyboard].name` (spaces → `_`) |
   | `{{ chip_name }}` | canonical chip id — fills `[keyboard].chip` *and* the stm32/esp HAL feature (`esp_chip` / stm32 part id) |
   | `{{ target }}` | matrix `target` (stm32: `core_rules` target) |
   | `{{ channel }}` | matrix `channel` (`stable`, or `esp` for Xtensa) |
   | `{{ build_std }}` | matrix `build_std` block, else removed |
   | `{{ uf2_family }}` | matrix `uf2_family` (`prefix7` → `chip[..7]`) |
   | `{{ esp_chip }}` | matrix `esp_chip` (esp family only) |
6. Write the `rmk` dependency line via the `cargo_toml` crate: the **version /
   source comes from `chip_matrix.toml` `[rmk]`** (a crates.io `version`, or
   `git` + `rev`) and the **features from step 1** (`default-features` +
   `firmware_features().rmk_features`). The rmk version thus lives in one place,
   and the whole chain — this template ref *and* rmk — can be git or released.
7. **Validate** the derived features against the *resolved* rmk's `[features]`
   table (`cargo metadata` on the generated project). A feature the pinned rmk
   doesn't define is a hard error with a clear message (e.g. `rmk has no feature
   'host_lock' at this version; use a matching rmk`) — not a cryptic Cargo error.
   This catches vocabulary skew: the feature names `firmware_features()` speaks
   are those of the rmk its rmk-config was released with, so `[rmk]` must point at
   that same rmk.
8. Merge every applied layer's `Cargo.overlay.toml` (if any) into `Cargo.toml`
   via `cargo_toml`: `[dependencies]` and `[patch.*]` tables are added, and
   `[[bin]]` is replaced when the fragment defines it.
9. Copy the memory layout: `memory/<memory_x>.x` → `memory.x` (families whose
   matrix row has no `memory_x`, i.e. stm32/esp, ship none).
10. Copy the user's `keyboard.toml` + `vial.json` into the project (`create`);
    `init` keeps the family default with `{{ chip_name }}` filled to the chip.

## Placeholder rule

Every template file is valid to read as-is (placeholders are inert text). The
family template's `rmk` line and any `{{ chip_name }}` are the only entries that
are *always* rewritten; the rest are matrix-driven and may be identical across a
family (e.g. `channel = "stable"`).

## `Cargo.overlay.toml`

A layer that needs to add crates or patches ships a `Cargo.overlay.toml`. It is a
partial manifest merged into the project `Cargo.toml`:

- `[dependencies]` — added/overridden (e.g. pico_w adds `cyw43`).
- `[patch.crates-io]` — added (e.g. pico_w's cyw43/embassy pins, esp32h2's esp-hal fork).
- `[features]` — added (e.g. pico_w's `skip-cyw43-firmware`).
- `[[bin]]` — replaces the family's single bin (split defines central + peripheral).

## Verification

- **Golden files**: `rmkit create` output for representative + escape-hatch chips
  is snapshotted under `tests/golden/` and drift-gated. (Lands with the rmkit
  rewrite, which is the only thing that can produce them.)
- **Compile matrix**: CI runs `rmkit create` + `cargo build --release` +
  `cargo make uf2` for rp2040, nrf52840(_split), esp32c3, esp32s3, stm32, pico_w.
