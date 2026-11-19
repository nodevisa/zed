# Nodevisa for Zed

Editor support for Nodevisa build projects.

The extension runs the language server in the `nodevisa-build` binary,
which must be **installed independently**.
Install either:

- the `nodevisa-build` package, or
- the `nodevisa` metapackage.

They can be installed:

- globally, with `pixi global`, or
- locally, in a project's [pixi](https://pixi.prefix.dev) environment,
such that it uses the same version of `nodevisa-build` that the project uses.

## Finding the server

The extension runs the first of these that it finds:

1. The command in Zed's settings.
2. `pixi run --frozen nodevisa-build lsp`.
3. `nodevisa-build lsp`.
4. `nodevisa build lsp`.

## Settings

In Zed's `settings.json`, under `lsp.nodevisa`:

```json
{
  "lsp": {
    "nodevisa": {
      "binary": { "path": "nodevisa", "arguments": ["build", "lsp"] },
      "settings": { "pixi": { "path": "~/.pixi/bin/pixi", "environment": "default" } }
    }
  }
}
```

| Setting | Description |
| --- | --- |
| `binary` | The command that runs the language server. |
| `settings.pixi.path` | The `pixi` program, if it is not on `PATH`. |
| `settings.pixi.environment` | The pixi environment with `nodevisa-build`, if not `default`. |

## Development

Install [pixi](https://pixi.prefix.dev).
Then,

- to install Zed and the Rust toolchain,
- compile the extension, and
- open an example with an isolated profile

run:

```sh
pixi run zed
```

### Project organization

- `extension/` is the extension, as Zed builds it.
- `example/` is a pixi workspace with `nodevisa-build` and two example projects:
  - `minimal/`, a valid project, and
  - `minimal-invalid/`, that has an error that the extension reports.
- The rest is the repository's tooling:
pixi tasks, linters, and the Rust standard library for `wasm32-wasip2`.

## Publishing

Zed's extensions are published through a pull request to
[zed-industries/extensions](https://github.com/zed-industries/extensions).

Zed's CI compiles the extension, so `extension.wasm` is not committed.

### First version

```sh
git clone https://github.com/zed-industries/extensions --depth 2 zed-extensions
cd zed-extensions
git submodule add https://github.com/nodevisa/zed.git extensions/nodevisa
```

and in its `extensions.toml`:

```toml
[nodevisa]
submodule = "extensions/nodevisa"
path = "extension"
version = "0.1.0"
```

then `pnpm sort-extensions`, and open the pull request.

### Update version

For a new version:

1. bump `version` in `extension/extension.toml` (and `extension/Cargo.toml`).
2. open a pull request that moves the submodule to that commit
and updates `version` in `extensions.toml`.
