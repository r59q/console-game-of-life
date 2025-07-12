# Conway's Game of Life
*v1.0* - Written in *Rust*

<p align="center">
  <img src="https://github.com/r59q/Conway-s-Game-of-Life-in-console/assets/6570193/7dece83c-74a1-487d-af4c-f63513b22dd0">
</p>

This is an iteration of the popular zero-player game 'Game of Life' by Conway.

It is a small project written in the Rust programming language as a learning exercise. It is a simple console program, which uses ASCII art to draw the graphics.

## Graphics
The rendering is handled by the [console engine](https://crates.io/crates/console_engine) crate.

## Game logic
All game logic is handled by the [Bevy Entity Component System(ECS)](https://crates.io/crates/bevy_ecs)

## Running it

You can either clone the repository and do `cargo run` or you can download a build from the links below

**Downloads**
- [Download page](https://consolegameoflife.com/download/)

## How to play
Conway's Game of Life is a so-called zero-player game, meaning you can simply configure the initial state of the simulation and observe it unfold.

1. Pause the game using `P`.
2. Setup your starting state by placing cells with `left-click`.
3. Run the simulation by unpausing again using `P`.

The game world is infinite, which means you can drag the simulation space by using `right-click drag`.

Use `R` to randomize a starting state.

If the speed is too slow increase it using `X`. *Note*: It is limited by the framerate.

You can decrease the simulation speed using `Z`.

If at any point you forget the key-bindings, a menu can be found by using `H`.

## Running with arguments (optional)
Optionally you can specify the target framerate of the program. By default the program will attempt to render in 60 fps.

However despite it's simplicity, it still uses console re-draws to handle rendering, so do not expect massive framerates.

For a target fps of 40 use the parameter
```
cargo run -- 40
```

## Building
To build the project, you need to have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

After installing Rust, you can build the project by running:
```bash
cargo build --release
```

**Building for windows on linux**

ensure you have the `x86_64-pc-windows-gnu` target installed. You can do this by running:
```bash
rustup target add x86_64-pc-windows-gnu
```
you may also need to install the `mingw-w64` toolchain, which can be done on Arch with:
```bash
sudo pacman -S mingw-w64-gcc
```
build the project with the target:
```bash
cargo build --release --target=x86_64-pc-windows-gnu
```