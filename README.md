# CHIP-8 Emulator

A CHIP-8 emulator written in Rust.

## Running

Run with a ROM file. Some ROM files are included in the `roms/` directory:
```bash
cargo run -- roms/BREAKOUT
```

Use debug mode to see registers and instructions (but it lags):
```bash
cargo run -- --degug roms/INVADERS
```
