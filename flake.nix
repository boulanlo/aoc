{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    utils.url = "github:numtide/flake-utils/5aed5285a952e0b949eb3ba02c12fa4fcfef535f";
    rust-overlay.url = "github:oxalica/rust-overlay"; # use master since newer Rust versions are continuously pushed
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
        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [ rust-bin.stable.latest.default rust-analyzer ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
        
      });
}
