## How to Run
```shell
cargo install cargo-vcpkg
cargo vcpkg build
cargo run
```

### Usage
```
Conway's Game of Life in Rust!

Usage: conway-rs [OPTIONS]

Options:
  -W, --width <WIDTH>    The width of the board [default: 32]
  -H, --height <HEIGHT>  The height of the board [default: 32]
  -p, --periodic         Enable periodic boundary conditions
  -h, --help             Print help
  -V, --version          Print version
```