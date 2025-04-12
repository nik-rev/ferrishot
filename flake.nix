{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        manifest = pkgs.lib.importTOML ./Cargo.toml;
        nativeBuildInputs = with pkgs; [ pkg-config ];
        buildInputs = with pkgs; [
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
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.package.name;
          version = manifest.package.version;

          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;
          cargoLock.outputHashes = {
            "iced-0.14.0-dev" = "sha256-0Pe88fUGPZ6rpOM4yKXw73CMuyRfb90hFW/38SBd7JY=";
            # "iced_core-0.14.0-dev" = "sha256-BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB=";
            # "iced_debug-0.14.0-dev" = "sha256-CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC=";
            # "iced_futures-0.14.0-dev" = "sha256-DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD=";
            # "iced_renderer-0.14.0-dev" = "sha256-EEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE=";
            # "iced_graphics-0.14.0-dev" = "sha256-FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF=";
            # "iced_tiny_skia-0.14.0-dev" = "sha256-GGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGG=";
            # "iced_wgpu-0.14.0-dev" = "sha256-HHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHHH=";
            "cryoglyph-0.1.0" = "sha256-X7S9jq8wU6g1DDNEzOtP3lKWugDnpopPDBK49iWvD4o=";
            # "iced_runtime-0.14.0-dev" = "sha256-JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ=";
            # "iced_widget-0.14.0-dev" = "sha256-KKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK=";
            # "iced_winit-0.14.0-dev" = "sha256-LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL=";
            # "iced_program-0.14.0-dev" = "sha256-MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM=";
            # "winit-0.30.8" = "sha256-NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN=";
            "dpi-0.1.1" = "sha256-hlVhlQ8MmIbNFNr6BM4edKdZbe+ixnPpKm819zauFLQ=";
          };

          inherit nativeBuildInputs;
          inherit buildInputs;
        };
      }
    );
}
