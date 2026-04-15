{
  description = "vity.nvim colorscheme flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in {
        # build
        # packages.default = pkgs.callPackage ./default.nix {inherit pkgs rustToolchain;};

        # dev shell
        devShells.default = with pkgs;
          mkShell rec {
            buildInputs = [
              openssl
              pkg-config
              rustToolchain

              cmake
              fontconfig

              libxkbcommon
              libGL

              # WINIT_UNIX_BACKEND=wayland
              wayland

              # WINIT_UNIX_BACKEND=x11
              libxcursor
              libxrandr
              libxi
              libx11
            ];

            CMAKE_POLICY_VERSION_MINIMUM = 3.5;
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          };
      }
    );
}
