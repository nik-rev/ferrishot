# Building

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
