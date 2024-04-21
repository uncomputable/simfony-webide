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

