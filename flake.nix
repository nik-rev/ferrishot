{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
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
        inputs = with pkgs; [
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
          buildInputs = inputs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath inputs;
        };
      }
    );
}
