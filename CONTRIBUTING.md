# Contributing

`ferrishot` takes a screenshot of the current desktop and creates a new window with background set to the taken screenshot.

Some pointers:

- `app.rs` holds the `App` struct which contains all information about the program.
- `App::view` is the entry point for rendering of all the elements.
- `message.rs` holds `Message` enum which defines all events that can happen which mutate the `App`'s state. `App::update` responds to this.
- `config/mod.rs` defines each config option. Make sure to also update `default.kdl` when modifying the config.

100% of the code is documented. To take advantage of that you can use `cargo doc --document-private-items --open`.

#### CLI Markdown Help

You can generate markdown documentation for the CLI interface by running:

```sh
cargo run --features docgen -- --markdown-help
```

## Building

On Windows and MacOS there are no dependencies.

On Linux, you will need these (`apt` package manager names):

- `libgl-dev`
- `libx11-dev`
- `libxcbcommon-dev`

If you use wayland you will also need `libwayland-dev` lib.

For nix users, there is a `flake.nix` which you can use with `nix develop`.

To run:

```sh
cargo run
```

## Debugging

- Use `F12` to toggle the debug overlay which contains shows information about the state of ferrishot.
- The `.explain()` method on `Element` provided by the `Explainer` trait will show a red border around an element and all of its children.

## Logging

- The default log level is `error`. You can set a different log level using `--log-level=<level>` or by specifying the `RUST_LOG=<level>` env variable.
  - Levels: `error`, `warn`, `info`, `debug`, `trace`, `off`.
- The logs get written into a file. You can see where with `ferrishot --print-log-file-path` or choose a custom one with `ferrishot --log-file`.
- To write the logs to standard output use the `--log-stdout` argument.

## Website

- `index.html` is the landing page and served at `ferrishot.com`. You can just open this file in the browser>
- `docs` is built using `mdbook serve`. It is served at `ferrishot.com/docs`.
