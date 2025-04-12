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
        buildInputs = with pkgs; [
          openssl
          pkg-config
          dbus

          xorg.libX11
          xorg.libXcursor
          xorg.xrandr
          xorg.libXi
          xorg.libxcb
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
            "cryoglyph-0.1.0" = "sha256-X7S9jq8wU6g1DDNEzOtP3lKWugDnpopPDBK49iWvD4o=";
            "dpi-0.1.1" = "sha256-hlVhlQ8MmIbNFNr6BM4edKdZbe+ixnPpKm819zauFLQ=";
          };

          nativeBuildInputs = buildInputs;
          inherit buildInputs;
        };
      }
    );
}
