# Learn Rust Game Dev — LazyFoo SDL3 Port

Porting the [LazyFoo SDL3 tutorials](https://lazyfoo.net) to Rust as a learning exercise.

## Dependencies

- [Rust](https://rustup.rs) with `stable-x86_64-pc-windows-msvc` toolchain
- SDL3 dev libraries: https://github.com/libsdl-org/SDL/releases
- SDL3_image dev libraries: https://github.com/libsdl-org/SDL_image/releases

## Setup

1. Download SDL3 and SDL3_image MSVC dev libraries and extract them somewhere on your machine.
2. Update `.cargo/config.toml` with your actual library paths:

```toml
[target.x86_64-pc-windows-msvc]
rustflags = [
    "-L", "YOUR_PATH\\SDL3\\lib\\x64",
    "-L", "YOUR_PATH\\SDL3_image\\lib\\x64",
]
```

3. Copy `SDL3.dll` and `SDL3_image.dll` to `target\debug\` after first build.

## Running a lesson

```
cargo run --bin lesson_01
cargo run --bin lesson_02
```
