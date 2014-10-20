# rust-picotcp

An ongoing effort to create bindings for the complete PicoTCP library
NOTE: Nothing is working yet!

## Installation

You don't have to install `rust-picotcp`,
but rather add it to your `Cargo.toml` as a dependency:

```
[dependencies.picotcp]
git = "https://github.com/maximevince/rust-picotcp.git"
```


## Usage

Here's a sample program using rust-picotcp:

```
extern crate picotcp;

fn main() {
    picotcp::stack_init();
}
```

If you put that into a file called `src/picotest.rs`, and add a corresponding `Cargo.toml` file, you can:

```
$ cargo build
$ ./target/picotest
```

## License

GPLv2 licensed.
