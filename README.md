# Simfony Web IDE

[Simfony](https://github.com/BlockstreamResearch/simfony) is a high-level language for writing Bitcoin smart contracts.

Simfony looks and feels like [Rust](https://www.rust-lang.org). Just how Rust compiles down to assembly language, Simfony compiles down to Simplicity bytecode. Developers write Simfony, full nodes execute Simplicity.

[A live demo is running on GitHub pages](https://ide.simp-lang.com).

## Develop the project

### Install dependencies

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
