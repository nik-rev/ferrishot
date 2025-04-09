{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
          ];
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              pkg-config
              dbus
              xorg.libxcb
              xorg.xrandr
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              libxkbcommon
              vulkan-loader
              wayland
              # (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              dbus
              xorg.libxcb
              xorg.xrandr
              xorg.libX11
              xorg.libXcursor
              xorg.libXi
              libxkbcommon
              vulkan-loader
              wayland
            ];
          };
      }
    );
}
