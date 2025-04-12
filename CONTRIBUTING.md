# Building

On Windows and MacOS there are no dependencies.

On Linux, you will need these:

- `ligGL`
- `libxcb`
- `libxkb`
- `libX11`

If you use wayland you will also need `wayland` lib.

For nix users, there is a `flake.nix` which you can use with `nix develop`.

To run:

```sh
cargo run
```
