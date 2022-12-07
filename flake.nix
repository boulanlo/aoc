{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells = utils.lib.flattenTree {
          default = with pkgs; mkShell {
            buildInputs = [ rust-bin.stable.latest.default rust-analyzer ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        };

        packages = utils.lib.flattenTree rec {
          aoc = pkgs.rustPlatform.buildRustPackage {
            pname = "aoc";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [ pkg-config rust-bin.stable.latest.minimal ];
          };

          default = aoc;
        };
        
      });
}
