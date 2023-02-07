# Dependencies
### Cargo
### `raylib` crate

## Build instructions
install cargo using the rustup installer found [here](https://rustup.rs/)

once in the root project directory, just type `cargo run` to update the dependencies, build the required files, and run the binary.

# Known Bugs
- Window closes immediately after opening if executable is launched without `cargo run`(includes in the file browser and terminal)

# Other things
- different functions and structs are documented, so feel free to run `cargo doc --open` to open the documentation.

- The only thing implemented right now is player movement