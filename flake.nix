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
          pkgs.makeWrapper
          fontconfig

          xorg.libX11
          xorg.libXcursor
          xorg.xrandr
          xorg.libXi
          xorg.libxcb
          libxkbcommon
          vulkan-headers
          vulkan-loader
          libGL

          libxkbcommon
          # WINIT_UNIX_BACKEND=wayland
          wayland

          # WINIT_UNIX_BACKEND=x11
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          xorg.libX11
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

          inherit buildInputs;

          postFixup = ''
            wrapProgram $out/bin/ferrishot \
              --suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
          '';
        };
      }
    );
}
