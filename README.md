# Simplicity Web IDE

[Simplicity](https://github.com/BlockstreamResearch/simplicity) development in the browser!

Write and execute Simplicity programs.

Take a look at the example programs.

[A live demo is running on GitHub pages](https://uncomputable.github.io/simplicity-webide/).

## Develop the project

### Enter the development environment

Enter the provided developer shell.

```bash
nix develop
```

### Deploy a local website

Use trunk to compile the project and deploy a local website.

```bash
trunk serve
```

Trunk can open a browser if one is available.

```bash
trunk serve --open
```

Trunk will keep running in the background and make live updates to the website as you change the code.
