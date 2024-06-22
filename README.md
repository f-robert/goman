# Goman

`goman` is a version manager for the Go programing language written in Rust.
It allows you to easily install, switch, remove and list versions of Go.

## Installation

### Prerequisites

Ensure you have the following tools installed:

- Rust (you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install))
- Cargo (comes with Rust)

### Building `goman`

Build the executable:

```sh
git clone https://github.com/f-robert/goman.git
cd goman

cargo build --release
```

Copy the executable:

```sh
cp target/release/goman ~/.local/bin/
```
