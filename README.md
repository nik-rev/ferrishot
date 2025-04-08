# Nik's Rust Template

I like to have clippy config be really strict. This helps to have a much more consistent code style. It's nice to offload more things to automated tools, so we can focus on the higher-level purpose of the software.

Template includes:

- Strict [clippy configuration](./Cargo.toml) with everything on `warn`, for easy prototyping
- Rustfmt options (with many of them requiring nightly).

  I use nightly rustfmt in my editor `helix`.

  Since it isn't possible to have an editor-agnostic setup with auto-save for nightly rustfmt while using the stable Rust toolchain. I do have that in my personal config, but any contributors are only required to follow the stable rustfmt formatting.

- GitHub actions to `check`, `build`, `test`, `format` and `lint`

- Nix flake
