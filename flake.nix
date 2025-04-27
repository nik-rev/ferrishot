{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
        manifest = pkgs.lib.importTOML ./Cargo.toml;
        buildInputs = with pkgs; [
          # required for the derivation
          makeWrapper

          # makes it more performant
          libGL

          # required with wayland
          wayland
          # required on Linux
          xorg.libxcb
          xorg.libX11
          libxkbcommon
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs =
            buildInputs
            ++ (with pkgs; [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
              rust-analyzer
              bacon
            ]);
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
        };
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.package.name;
          version = manifest.package.version;

          src = pkgs.lib.cleanSource ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "knus-3.2.0" = "sha256-eh6EH0NEVNLgwgHDD3IdrW+NTnL2QAY8EQIuyDhe5rw=";
            };
          };

          inherit buildInputs;

          postFixup = ''
            wrapProgram $out/bin/ferrishot \
              --suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
          '';
        };
        formatter = pkgs.alejandra;
      }
    );
}
