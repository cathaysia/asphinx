{
  description = "Devement environment for note";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        inherit (pkgs.stdenv) isDarwin isLinux;
      in
      {
        devShell = pkgs.mkShell {
          buildInputs =
            [
              pkgs.asciidoctor-with-extensions
              pkgs.plantuml
              pkgs.openjdk
              pkgs.svgbob
              pkgs.graphviz
              pkgs.d2
            ]
            ++ pkgs.lib.optionals isLinux [
              pkgs.nodePackages_latest.mermaid-cli
            ];
        };
      }
    );
}
