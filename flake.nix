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

          inherit nativeBuildInputs;
          inherit buildInputs;
        };
      }
    );
}
