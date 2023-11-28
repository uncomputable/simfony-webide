# Simplicity Web IDE

[Simplicity](https://github.com/BlockstreamResearch/simplicity) development in the browser!

Write Simplicity programs and see how they are executed on the Bit Machine.

[A live demo is running on GitHub pages](https://uncomputable.github.io/simplicity-webide/).

## Develop the project

Enter the provided nix shell.

```bash
nix-shell
```

### Deploy a local website

Use trunk to compile the project and deploy a local website.

```bash
trunk serve --open
```

Trunk will keep running in the background and make live updates to the website as you change the code.

### Deploy a global website

Use trunk to generate static HTML and JavaScript artifacts.

```bash
trunk build --release
```

Upload the artifacts to a web server.
