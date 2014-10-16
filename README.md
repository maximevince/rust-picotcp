# rust-picotcp

An ongoing effort to create bindings for the complete PicoTCP library
NOTE: Nothing is working yet!

## Installation

You can install `rust-picotcp` with `rustpkg`:

```
$ rustpkg install github.com/maximevince/rust-picotcp
```


## Usage

Here's a sample program using hello:

```
extern mod picotcp;

fn main() {
  picotcp::stack_init();
}
```

If you put that into a file called `picotest.rs`, you can:

```
$ rustc picotest.rs
$ ./picotest
```

## License

GPLv2 licensed.
