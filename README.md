# About
EZRand is a simple random number generator written in rust. I created this
as a fun project, and this has literally *no* point in existing, but is here
if you want to mess around with it

# dependencies
- Cargo
- `rand` crate
- `clap` crate(for CLI)

## install instructions
make sure you have rust and cargo installed via [rustup](https://rustup.rs).

once cargo has been installed and is on `PATH`, type `cargo build --release`
which will build the application inside `target/release`. from there, 
open the application from the terminal, and use it. the cli arguments are
as follows:

|Usage: | -b --bottom -t --top -h -V 		|
| ----- | --------------------------------- |
| -b    | the low number to randomize		|
| -t    | the high number to randomize      |
| -h    | show help information             |
| -V    | show the version number           |
