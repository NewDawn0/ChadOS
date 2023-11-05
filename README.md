```
  ____ _               _  ___  ____
 / ___| |__   __ _  __| |/ _ \/ ___|
| |   | '_ \ / _` |/ _` | | | \___ \
| |___| | | | (_| | (_| | |_| |___) |
 \____|_| |_|\__,_|\__,_|\___/|____/
  https://github.com/NewDawn0/ChadOS
```

Welcome to ChadOS, an experimental operating system created as part of an
A-level thesis project. This operating system was developed to explore the
complexities and challenges involved in creating a custom OS. If you're
interested in learning more about the thesis behind ChadOS, you can find it in
the ChadOS/doc/thesis.pdf file.

<!-- vim-markdown-toc GFM -->

* [Build](#build)
    * [Building using Nix](#building-using-nix)
    * [Building from source](#building-from-source)
* [Running tests](#running-tests)
* [Running it in Qemu](#running-it-in-qemu)
    * [Using Nix](#using-nix)
    * [From source](#from-source)

<!-- vim-markdown-toc -->

## Build

Let's get started with building ChadOS.

1. Clone the Repository

```bash
git clone github.com/NewDawn0/ChadOS.git
cd ChadOS
```

### Building using Nix

Building ChadOS using Nix is a straightforward process. Ensure you have Nix
installed before proceeding.

Prerequisites:

- [Nix](https://nixos.org/download)

Build

```bash
nix-build # Currently doesn't work as compiler_builtins isn't found
# You'll find the resulting ChadOS.img in ./result/bin
```

Alternatively, you can work within a Nix shell for an isolated development
environment.

```bash
nix develop
cargo make clean
cargo make build
# Resulting ChadOS.img in the current directory
```

### Building from source

If you prefer to build ChadOS from source, follow these steps.

Prerequisites:

- [Qemu](https://www.qemu.org)
- [Rustup](https://rustup.rs)
- [dd (GNU Coreutils)](https://www.gnu.org/software/coreutils/)
- [strip (GNU Coreutils)](https://www.gnu.org/software/coreutils/)

Set up the environment

```bash
rustup default nightly # Install nightly toolchain
rustup component add rust-src llvm-tools-preview # Install rust plugins
rustup target add x86_64-unknown-none # Install rustup toolchain
cargo install cargo-bootimage
cargo install cargo-make
```

Building the OS

```bash
cargo make clean
cargo make build
# Resulting ChadOS.img in the current directory
```

## Running tests

You can run tests for ChadOS by executing the following command:

```bash
cargo make test
```

## Running it in Qemu

ChadOS can be run in the QEMU virtual machine emulator. Choose one of the
following methods to run it.

### Using Nix

```bash
nix-shell
cargo make clean
cargo make run
```

### From source

If you've built ChadOS from source, run it with the following commands:

```bash
cargo make clean
cargo make run
```
