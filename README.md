# Tet.rs - Guideline-ish Tetris clone written in Rust
- Super Rotation System implemented (e.g. supports the same wallkicks as a modern Tetris game)
- Bag randomizer
- Ghost piece

## Differences from Guideline
- Holding a direction key snaps a piece to the corresponding edge of the screen after a configurable delay, e.g. `ARR = 0`
- No scoring/timing. Won't add the former, may add the latter one day
- Pieces never lock on their own, you have to manually hard drop them
- Only one piece preview to simplify graphics, may change later
- No gravity, probably won't add this
- Soft drop is instantaneous, will change if and only if the above changes
- Default controls are my preferred controls (up for hard drop, z/x for rotation, shift for hold)

## Demo
- https://youtu.be/pZjVrsYqs-w

## Installation
### Non-Nix(OS)
- Ensure GNU `make`, `cmake`, and the `SDL2` library are installed
- Use `cargo install --path .`
- Only tested on Linux but probably works for other platforms?

### Nix(OS) with flakes
- Provides a `flake.nix` adapted from [my rust flake template](https://github.com/DylanBulfin/rust-flake-template)
- `tetrs.overlays.default` is an overlay that adds tetrs to `pkgs`
- `tetrs.nixosModules.default` is a Home Manager module for configuring tetrs, only needed if you want to change the default settings (under `./config/default.toml`)
- Also provides a dev shell, enter root directory and run `nix shell`
- Or, if using `direnv` run the below commands from root directory:
    - `echo "use flake" >> .envrc`
    - `direnv allow`
- To run locally just use `cargo run` inside the shell environment, or use `nix flake run`

## Configuration
- Configuration file is optional and located at `~/.config/tetrs/config.toml`
- Supports configuration of all keybindings, as well as `DAS`, the length of time (in ms) you have to hold a direction key for the piece to snap to the edge of the screen. Default 150
- Default configuration is under `./config/default.toml` and should hopefully be self-explanatory. 