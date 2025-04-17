# Contributing

`ferrishot` takes a screenshot of the current desktop and creates a new window with background set to the taken screenshot.

Some pointers:

- `app.rs` holds the `App` struct which contains all information about the program.
- `App::view` is the entry point for rendering of all the elements.
- `message.rs` holds `Message` enum which defines all events that can happen which mutate the `App`'s state.
- `App::update` responds to a `Message` and mutates state.

100% of the code is documented. To take advantage of that you can use `cargo doc --document-private-items --open`.

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

## Website

- `docs/index.html` is the landing page and served at `ferrishot.com`
- `docs/docs/index.html` is created from `mdbook build` in a github action and is served at `ferrishot.com/docs`
