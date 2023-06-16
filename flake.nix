{
  description = "Devement environment for note";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };


  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs
          {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
        rust-night = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rust-analysis" "rust-std" "rust-docs" "clippy" ];
          targets = [ "x86_64-unknown-linux-musl" ];
        };
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            rust-night
            pkgs.pkgsStatic.openssl
            pkgs.pkgsStatic.zlib
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.openssl.out}/lib
            export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${pkgs.zlib.out}/lib
          '';

        };
      });
}
