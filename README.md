# Simplicity Web IDE

[Simplicity](https://github.com/BlockstreamResearch/simplicity) development in the browser!

Write and execute Simplicity programs.

Take a look at the example programs.

[A live demo is running on GitHub pages](https://uncomputable.github.io/simplicity-webide/).

## Develop the project

First install nix.

```bash
curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install
```

_You might need to open a new terminal for the changes to take effect._

Then enter the nix developer environment.

```bash
nix develop
```

Now you can use all `just` commands.

```bash
just --list
```

### Deploy a local website

Compile the website and serve it on localhost.

```bash
just serve
```

You can instruct the compiler to open the website on the default browser.

```bash
just open
```

The compiler will keep running in the background and make live updates to the website as you change the code.

### Deploy on Mac

https://book.leptos.dev/getting_started/index.html

```bash
cargo install trunk

# need the nightly build of rust
rustup toolchain install nightly
rustup default nightly

cargo build
rustup target add wasm32-unknown-unknown

cargo install just
just serve
# see justfile for other commands
# can also run trunk directly
trunk serve --port 3000 --open
```

If there are errors compiling secp256k1 then:
https://github.com/rust-bitcoin/rust-secp256k1/issues/283

```bash
brew install llvm
export PATH="/usr/local/opt/llvm/bin/:$PATH"
export CC=/usr/local/opt/llvm/bin/clang
export AR=/usr/local/opt/llvm/bin/llvm-ar
```
