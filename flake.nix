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
          mkShell {
            buildInputs = [
              # openssl
              # pkg-config
              rustToolchain
            ];
          };
      }
    );
}
